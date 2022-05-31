use crate::args::ProgramArgs;
use crate::solver::{simulate, solve};
use clap::{ArgMatches, Command, ErrorKind};
use log::debug;
pub struct Program {
    cmd: Command<'static>,
    matches: ArgMatches,
    active_lights: Vec<usize>,
    simulation_steps: Vec<usize>,
    board: Vec<bool>,
    cols: usize,
    rows: usize,
}

impl Program {
    pub fn new(mut cmd: Command<'static>) -> Self {
        let matches = cmd.get_matches_mut();

        Self {
            cmd,
            matches,
            active_lights: vec![],
            simulation_steps: vec![],
            board: vec![],
            cols: 0,
            rows: 0,
        }
    }

    pub fn load_data(&mut self) {
        let (active_nodes, rows, cols) = self.load_board_data(&self.matches);
        let simulation_steps = self.load_simulation_data(&self.matches);

        let total_nodes = rows * cols;

        self.active_lights = active_nodes;
        self.simulation_steps = simulation_steps;
        self.cols = cols;
        self.rows = rows;

        debug!("Active indices: {:?}", self.active_lights);
        debug!("Rows: {:?}", self.rows);
        debug!("Cols: {:?}", self.cols);

        self.validate_data();

        let mut board: Vec<bool> = vec![false; total_nodes];
        for position in &self.active_lights {
            board[*position] = true;
        }

        self.board = board;

        debug!("Board: {}", self.prettify_board(&self.board));
    }

    pub fn is_enabled(&self, id: &str) -> bool {
        self.matches.is_present(id)
    }

    fn load_board_data(&self, matches: &ArgMatches) -> (Vec<usize>, usize, usize) {
        let mut nodes: Vec<usize> = matches
            .get_many::<usize>(ProgramArgs::Lights.name())
            .unwrap_or_default()
            .copied()
            .collect();
        nodes.sort_unstable();
        nodes.dedup();
        let rows: usize = *matches.get_one(ProgramArgs::Rows.name()).unwrap();
        let cols: usize = *matches.get_one(ProgramArgs::Cols.name()).unwrap();
        (nodes, rows, cols)
    }

    fn load_simulation_data(&self, matches: &ArgMatches) -> Vec<usize> {
        matches
            .get_many(ProgramArgs::RunSimulation.name())
            .unwrap_or_default()
            .copied()
            .collect()
    }

    fn validate_data(&mut self) {
        // @TODO: make validations non-static
        Self::validate_indices(&self.active_lights, &mut self.cmd, self.rows, self.cols);
        Self::validate_range_indices(&self.simulation_steps, &mut self.cmd, self.rows, self.cols);
    }

    fn validate_range_indices(
        active_nodes: &Vec<usize>,
        cmd: &mut Command,
        rows: usize,
        cols: usize,
    ) {
        let max_value = rows * cols - 1;

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

    fn validate_indices(active_nodes: &Vec<usize>, cmd: &mut Command, rows: usize, cols: usize) {
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

        Self::validate_range_indices(active_nodes, cmd, rows, cols);
    }

    fn prettify_board(&self, board: &Vec<bool>) -> String {
        let mapped_board = self.map_board(board);

        self.board_to_str(&mapped_board)
    }

    fn board_to_str(&self, board_as_char: &Vec<String>) -> String {
        let mut board_string = String::new();
        for (index, node) in board_as_char.iter().enumerate() {
            if index % self.cols == 0 {
                board_string.push('\n');
            }

            board_string.push_str(node);
        }

        board_string
    }

    fn map_board(&self, board: &Vec<bool>) -> Vec<String> {
        board
            .iter()
            .map(|node| {
                if *node {
                    "#".to_string()
                } else {
                    "·".to_string()
                }
            })
            .collect()
    }

    pub fn run(&mut self) {
        self.load_data();

        if !self.is_enabled(ProgramArgs::RunSimulation.id()) {
            let solution = self.run_solver(&self.board);
            self.print_solution(
                &self.board,
                solution,
                self.matches
                    .get_one::<String>(ProgramArgs::DisplayMode.name())
                    .unwrap(),
            );
        } else {
            self.run_simulation(&self.board, &self.simulation_steps);
        }
    }

    fn print_solution(&self, board: &Vec<bool>, solution: Option<Vec<usize>>, draw_mode: &String) {
        debug!("Draw mode: {}", draw_mode);

        if draw_mode == "simple" || draw_mode == "all" {
            if let Some(result) = &solution {
                println!("{:?}", result);
            } else {
                println!("{:?}", &solution);
            }
        }

        if draw_mode == "draw" || draw_mode == "all" {
            let mut mapped_board = self.map_board(board);

            for (order, position) in solution
                .or(None)
                .unwrap_or_default()
                .into_iter()
                .enumerate()
            {
                mapped_board[position] = order.to_string();
            }

            println!("{}", self.board_to_str(&mapped_board));
        }
    }

    fn run_solver(&self, board: &Vec<bool>) -> Option<Vec<usize>> {
        debug!("Searching for solution ...");
        let solution = solve(board);
        debug!("Final solution: {:?}", &solution);

        solution
    }

    fn run_simulation(&self, board: &Vec<bool>, simulation_steps: &Vec<usize>) {
        let mut board = board.clone();

        debug!(
            "Board before the simulation:\n {}",
            self.prettify_board(&board)
        );
        debug!("Steps to simulate: {:?}", simulation_steps);

        for (step, node_to_trigger) in simulation_steps.iter().enumerate() {
            simulate(&mut board, *node_to_trigger);
            debug!("Step {}:\n {}", step, self.prettify_board(&board));
        }

        debug!("Board after simulation: {}", self.prettify_board(&board));

        print!("{}", self.prettify_board(&board));
    }

    fn rotate_light_indices(&self, indices:&mut Vec<usize>, current_state:&str, from_top_left: bool){
        match current_state {
            "tr" => Self::reorder_cols(indices, self.cols, from_top_left),
            "bl" => Self::reorder_rows(indices, self.rows, from_top_left),
            "br" => {
                Self::reorder_cols(indices, self.cols, from_top_left);
                Self::reorder_rows(indices, self.rows, from_top_left)
            },
            _ | "tl" => unreachable!()
        };
    }

    fn reorder_rows(indices:&mut Vec<usize>, rows: usize, top_to_bottom: bool) {
        let direction = if top_to_bottom { 1 } else { -1 };
        let rows = rows as isize;

        indices
            .iter_mut()
            .for_each(|undex| {
                let index = *undex as isize;

                let row = (index - 1) / rows;
                let offset = rows * (rows - 1 - 2 * row);
                *undex = (index + offset * direction) as usize;
            });
    }

    fn reorder_cols(indices:&mut Vec<usize>, cols: usize, left_to_right: bool) {
        let direction = if left_to_right { 1 } else { -1 };
        let cols = cols as isize;

        indices
            .iter_mut()
            .for_each(|undex| {
                let index = *undex as isize;

                let col = (index - 1) / cols;
                let offset = cols - 1 - 2*col;
                *undex = (index + offset * direction) as usize                
            });
    }
}
