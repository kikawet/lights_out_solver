use lights_out_solver::args::{init_app, ProgramArgs};
use lights_out_solver::solver::{simulate, solve};
use lights_out_solver::program::{Program, self};
use log::{debug, info};

use clap::{ArgMatches, Command, ErrorKind};

use simple_logger::SimpleLogger;

fn main() {
    let mut program = Program::new(init_app()) ;

    // set_up_logger(&program);

    

    // validate_indices(&active_nodes, &mut program.cmd, rows, cols);
    // validate_range_indices(&simulation_steps, &mut program.cmd, rows, cols);

    // if !program.matches.is_present(ProgramArgs::RunSimulation.id()) {
    //     let solution = run_solver(&board);
    //     print_solution(
    //         &board,
    //         solution,
    //         program.matches
    //             .get_one::<String>(ProgramArgs::DisplayMode.name())
    //             .unwrap(),
    //         cols,
    //     );
    // } else {
    //     run_simulation(board, simulation_steps);
    // }
}

fn print_solution(
    board: &Vec<bool>,
    solution: Option<Vec<usize>>,
    draw_mode: &String,
    cols: usize,
) {
    debug!("Draw mode: {}", draw_mode);

    if draw_mode == "simple" || draw_mode == "all" {
        if let Some(result) = &solution {
            println!("{:?}", result);
        } else {
            println!("{:?}", &solution);
        }
    }

    if draw_mode == "draw" || draw_mode == "all" {
        let mut mapped_board = vec![];//map_board(board);

        for (order, position) in solution
            .or(None)
            .unwrap_or_default()
            .into_iter()
            .enumerate()
        {
            mapped_board[position] = order.to_string();
        }

        // println!("{}", board_to_str(&mapped_board));
    }
}

fn set_up_logger(program: &Program) {
    if program.is_enabled(ProgramArgs::Verbose.id()) {
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


fn run_simulation(board: Vec<bool>, simulation_steps: Vec<usize>) {
    let mut board = board;

    // debug!("Board before the simulation:\n {}", prettify_board(&board));
    debug!("Steps to simulate: {:?}", simulation_steps);

    for (step, node_to_trigger) in simulation_steps.iter().enumerate() {
        simulate(&mut board, *node_to_trigger);
        // debug!("Step {}:\n {}", step, prettify_board(&board));
    }

    // debug!("Board after simulation: {}", prettify_board(&board));

    // print!("{}", prettify_board(&board));
}

