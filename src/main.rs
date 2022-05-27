use lights_out_solver::solver::{simulate, solve};
use log::{debug, info};

extern crate clap;
use clap::{Arg, Command, ErrorKind};
use simple_logger::SimpleLogger;

fn main() {
    let mut cmd = Command::new("Puzzle Solver")
        .version("0.1.0")
        .about("With the given input of on node it will output the order to toggle the lights to solve the puzzle") 
        .arg(
            Arg::new("NODES")
                .help("Indexes of the active nodes starting at 0 on the top left")
                .multiple_values(true)
                .index(1)
                .validator(|s| s.parse::<usize>()),
        )
        .arg(
            Arg::new("rows")
                .help("The number of rows")
                .short('r')
                .value_name("rows")
                .takes_value(true)
                .default_value(&"3")
                .validator(|s| s.parse::<usize>()),
        )
        .arg(
            Arg::new("cols")
                .help("The number of columns")
                .short('c')
                .value_name("cols")
                .takes_value(true)
                .default_value(&"3")
                .validator(|s| s.parse::<usize>()),
        )
        .arg(
            Arg::new("verbose")
                .help("Use verbose output")
                .short('v')
                .value_name("verbose")
                .takes_value(false)
        )
        .arg(
            Arg::new("run_simulation")
                .help("Run a simulation of the puzzle")
                .short('s')
                .value_name("postions_to_trigger")
                .multiple_values(true)
                .takes_value(true)
                .validator(|s| s.parse::<usize>()),
        )
        .arg(
            Arg::new("output_mode")
                .help("Sets the output mode")
                .short('m')
                .value_name("output_mode")
                .takes_value(false)
                .possible_values(["simple", "draw", "all"])
                .default_value("draw"),
        );

    let found_matches = cmd.get_matches_mut();

    set_up_logger(&found_matches);

    let (active_nodes, rows, cols) = load_board_data(&found_matches);
    let simulation_steps = load_simulation_data(&found_matches);

    let total_nodes = rows * cols;

    let mut board: Vec<bool> = vec![false; total_nodes];
    for position in &active_nodes {
        board[*position] = true;
    }

    debug!("Active indices: {:?}", active_nodes);
    debug!("Rows: {:?}", rows);
    debug!("Cols: {:?}", cols);
    debug!("Board: {}", prettify_board(&board));

    validate_indices(&active_nodes, &mut cmd, rows, cols);
    validate_range_indices(&simulation_steps, &mut cmd, rows, cols);

    if !found_matches.is_present("run_simulation") {
        let solution = run_solver(&board);
        print_solution(
            &board,
            solution,
            found_matches
                .get_one::<String>("output_mode")
                .unwrap()
                .to_string(),
            cols,
        );
    } else {
        run_simulation(board, simulation_steps);
    }
}

fn print_solution(board: &Vec<bool>, solution: Option<Vec<usize>>, draw_mode: String, cols: usize) {
    debug!("Draw mode: {}", draw_mode);

    if draw_mode == "simple" || draw_mode == "all"{
        if let Some(result) = &solution {
            println!("{:?}", result);
        } else {
            println!("{:?}", &solution);
        }
    }

    if draw_mode == "draw" || draw_mode == "all"{
        let mut mapped_board = map_board(board);
    
            for (order, position) in solution.or(None).unwrap_or_default().into_iter().enumerate() {
                mapped_board[position] = order.to_string();
            }

            println!("{}", board_to_str(&mapped_board));
    }

}

fn load_board_data(matches: &clap::ArgMatches) -> (Vec<usize>, usize, usize) {
    let mut nodes: Vec<usize> = matches.values_of_t("NODES").unwrap_or_default();
    nodes.sort_unstable();
    nodes.dedup();
    let rows: usize = matches.value_of_t("rows").unwrap();
    let cols: usize = matches.value_of_t("cols").unwrap();
    (nodes, rows, cols)
}

fn load_simulation_data(matches: &clap::ArgMatches) -> Vec<usize> {
    matches.values_of_t("run_simulation").unwrap_or_default()
}

fn set_up_logger(matches: &clap::ArgMatches) {
    if matches.is_present("verbose") {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
        info!("Verbose mode enabled");
    }
}

fn run_solver(board: &Vec<bool>) -> Option<Vec<usize>> {
    debug!("Searching for solution ...");
    let solution = solve(board);
    debug!("Final solution: {:?}", &solution);

    solution
}

fn validate_range_indices(
    active_nodes: &Vec<usize>,
    cmd: &mut clap::Command,
    rows: usize,
    cols: usize,
) {
    let max_value = rows * cols;

    if let Some(out_of_range) = active_nodes.iter().find(|&&it| it > max_value) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            format!(
                "Index {} out of range for a {}x{} size",
                out_of_range, rows, cols
            ),
        )
        .exit();
    }
}

fn validate_indices(active_nodes: &Vec<usize>, cmd: &mut clap::Command, rows: usize, cols: usize) {
    let max_nodes = rows * cols;

    if active_nodes.len() > max_nodes {
        cmd.error(
            ErrorKind::ArgumentConflict,
            format!(
                "Too many parameters given. The maximum number of nodes is {}",
                max_nodes
            ),
        )
        .exit();
    }

    validate_range_indices(active_nodes, cmd, rows, cols);
}

fn run_simulation(board: Vec<bool>, simulation_steps: Vec<usize>) {
    let mut board = board;

    debug!("Board before the simulation:\n {}", prettify_board(&board));
    debug!("Steps to simulate: {:?}", simulation_steps);

    for (step, node_to_trigger) in simulation_steps.iter().enumerate() {
        simulate(&mut board, *node_to_trigger);
        debug!("Step {}:\n {}", step, prettify_board(&board));
    }

    debug!("Board after simulation: {}", prettify_board(&board));

    print!("{}", prettify_board(&board));
}

fn prettify_board(board: &Vec<bool>) -> String {
    let mapped_board = map_board(board);
    
    board_to_str(&mapped_board)
}

fn board_to_str(board_as_char: &Vec<String>) -> String{
    let mut board_string = String::new();
    for (index, node) in board_as_char.iter().enumerate() {
        if index % 3 == 0 {
            board_string.push_str("\n");
        }

        board_string.push_str(node);
    }

    board_string
}

fn map_board(board: &Vec<bool>) -> Vec<String> {
    board
        .iter()
        .map(|node| if *node { "#".to_string() } else { "·".to_string() })
        .collect()
}
