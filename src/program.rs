use crate::args::ProgramArgs;
use clap::{ArgMatches, Command};
use log::{debug, info};
pub struct Program {
    cmd: Command<'static>,
    matches: ArgMatches,
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
            board: todo!(),
            cols: todo!(),
            rows: todo!(),
        }
        .load_data()
    }

    fn load_data(mut self) -> Self {
        let (active_nodes, rows, cols) = self.load_board_data(&self.matches);
        let simulation_steps = self.load_simulation_data(&self.matches);

        let total_nodes = rows * cols;

        let mut board: Vec<bool> = vec![false; total_nodes];
        for position in &active_nodes {
            board[*position] = true;
        }

        debug!("Active indices: {:?}", active_nodes);
        debug!("Rows: {:?}", rows);
        debug!("Cols: {:?}", cols);
        debug!("Board: {}", self.prettify_board(&board));

        self.board = board;
        self.cols = cols;
        self.rows = rows;

        self
    }

    pub fn is_enabled(& self, id: &str) -> bool{
        self.matches.is_present(id)
    }

    fn load_board_data(&self, matches: &ArgMatches) -> (Vec<usize>, usize, usize) {
        let mut nodes: Vec<usize> = matches
            .get_many(ProgramArgs::Lights.name())
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

    fn prettify_board(&self, board: &Vec<bool>) -> String {
        let mapped_board = self.map_board(board);

        self.board_to_str(&mapped_board)
    }

    fn board_to_str(&self, board_as_char: &Vec<String>) -> String {
        let mut board_string = String::new();
        for (index, node) in board_as_char.iter().enumerate() {
            if index % 3 == 0 {
                board_string.push_str("\n");
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
}
