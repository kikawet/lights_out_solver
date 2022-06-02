use crate::args::ProgramArgs;
use crate::solvers::board::{BaseBoard, Board};
use crate::solvers::gf2;
use clap::{ArgMatches, Command, ErrorKind};
use log::debug;
pub struct Program {
    cmd: Command<'static>,
    matches: ArgMatches,
    board: Box<dyn Board>,
    simulation_steps: Vec<usize>,
}

impl Program {
    pub fn new(mut cmd: Command<'static>) -> Self {
        let matches = cmd.get_matches_mut();

        Self {
            cmd,
            matches,
            board: Box::new(BaseBoard::new_blank(0, 0)),
            simulation_steps: vec![],
        }
    }

    pub fn load_data(&mut self) {
        let (mut active_nodes, rows, cols) = self.load_board_data();
        let mut simulation_steps = self.load_simulation_data();

        debug!(
            "Input mode: {:?}",
            self.matches
                .get_one::<String>(ProgramArgs::InputMode.name())
                .unwrap()
        );

        Self::rotate_light_indices(
            &mut active_nodes,
            cols,
            rows,
            self.matches
                .get_one::<String>(ProgramArgs::InputMode.name())
                .unwrap()
                .to_string(),
        );

        Self::rotate_light_indices(
            &mut simulation_steps,
            cols,
            rows,
            self.matches
                .get_one::<String>(ProgramArgs::InputMode.name())
                .unwrap()
                .to_string(),
        );

        // convert from range 1..[cols]*[rows] to 0..[cols]*[rows]-1
        active_nodes.iter_mut().for_each(|val| *val -= 1);
        simulation_steps.iter_mut().for_each(|val| *val -= 1);

        self.board = Box::new(BaseBoard::new_from(&active_nodes, cols, rows));
        self.simulation_steps = simulation_steps;

        debug!("Active indices: {:?}", active_nodes);
        debug!("Rows: {:?}", rows);
        debug!("Cols: {:?}", cols);

        // let mut board: Vec<bool> = vec![false; total_nodes];
        // for position in &self.active_lights {
        //     board[*position] = true;
        // }

        // self.board = board;

        // debug!("Board: {}", self.prettify_board(&self.board));
    }

    pub fn is_enabled(&self, id: &str) -> bool {
        self.matches.is_present(id)
    }

    fn load_board_data(&mut self) -> (Vec<usize>, usize, usize) {
        let mut nodes: Vec<usize> = self.matches
            .get_many::<usize>(ProgramArgs::Lights.name())
            .unwrap_or_default()
            .copied()
            .collect();
        nodes.sort_unstable();
        nodes.dedup();
        let rows: usize = *self.matches.get_one(ProgramArgs::Rows.name()).unwrap();
        let cols: usize = *self.matches.get_one(ProgramArgs::Cols.name()).unwrap();

        Self::validate_indices(&nodes, &mut self.cmd, rows, cols);

        (nodes, rows, cols)
    }

    fn load_simulation_data(&mut self) -> Vec<usize> {
        let simulation_steps = self.matches
            .get_many(ProgramArgs::RunSimulation.name())
            .unwrap_or_default()
            .copied()
            .collect::<Vec<usize>>();

        let rows: usize = *self.matches.get_one(ProgramArgs::Rows.name()).unwrap();
        let cols: usize = *self.matches.get_one(ProgramArgs::Cols.name()).unwrap();

            
        Self::validate_range_indices(&simulation_steps, &mut self.cmd, rows, cols);

        simulation_steps
    }

    fn validate_range_indices(active_nodes: &[usize], cmd: &mut Command, rows: usize, cols: usize) {
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

    fn prettify_board(&self, board: &dyn Board) -> String {
        let mapped_board = self.map_board(board);

        self.board_to_str(&mapped_board)
    }

    fn board_to_str(&self, board_as_char: &[String]) -> String {
        let mut board_string = String::new();
        for (index, node) in board_as_char.iter().enumerate() {
            if index % self.board.cols() == 0 {
                board_string.push('\n');
            }

            board_string.push_str(node);
        }

        board_string
    }

    fn map_board(&self, board: &dyn Board) -> Vec<String> {
        board
            .iter()
            .map(|val| {
                if *val == 1 {
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
            let solution = self.run_solver();
            self.print_solution(
                self.board.as_ref(),
                solution,
                self.matches
                    .get_one::<String>(ProgramArgs::DisplayMode.name())
                    .unwrap(),
            );
        } else {
            self.run_simulation();
        }
    }

    fn print_solution(&self, board: &dyn Board, solution: Option<Vec<usize>>, draw_mode: &String) {
        debug!("Draw mode: {}", draw_mode);

        if draw_mode == "simple" || draw_mode == "all" {
            // need to clone solution bc in display mode 'all' this is going to change the board
            if let Some(result) = &mut solution.clone() {
                result.iter_mut().for_each(|val| *val += 1);

                let (cols, rows) = board.size();

                Self::rotate_light_indices(
                    result,
                    cols,
                    rows,
                    self.matches
                        .get_one::<String>(ProgramArgs::InputMode.name())
                        .unwrap()
                        .to_string(),
                );

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

    fn run_solver(&self) -> Option<Vec<usize>> {
        debug!("Searching for solution ...");

        let solution = gf2::solve(self.board.as_ref());
        debug!("Final solution: {:?}", &solution);

        solution
    }

    fn run_simulation(&mut self) {
        debug!(
            "Board before the simulation:\n {}",
            self.prettify_board(self.board.as_ref())
        );
        debug!("Steps to simulate: {:?}", self.simulation_steps);

        for (step, node_to_trigger) in self.simulation_steps.iter().enumerate() {
            self.board.trigger_index(*node_to_trigger);
            debug!("Step {}:\n {}", step, self.prettify_board(self.board.as_ref()));
        }

        debug!("Board after simulation: {}", self.prettify_board(self.board.as_ref()));

        print!("{}", self.prettify_board(self.board.as_ref()));
    }

    /**
     * Transformation are symectric so calling this twice with the same state is going to undo the changes
     */
    fn rotate_light_indices(indices: &mut [usize], cols: usize, rows: usize, state: String) {
        match state.as_str() {
            "tr" => Self::reorder_cols(indices, cols),
            "bl" => Self::reorder_rows(indices, rows),
            "br" => {
                Self::reorder_cols(indices, cols);
                Self::reorder_rows(indices, rows);
            }
            "tl" => { /*Do nothing 👀*/ }
            _ => unreachable!(),
        };
    }

    fn reorder_rows(indices: &mut [usize], rows: usize) {
        let rows = rows as isize;

        indices.iter_mut().for_each(|undex| {
            let index = *undex as isize;

            let row = (index - 1) / rows;
            let offset = rows * (rows - 1 - 2 * row);
            *undex = (index + offset) as usize;
        });
    }

    fn reorder_cols(indices: &mut [usize], cols: usize) {
        let cols = cols as isize;

        indices.iter_mut().for_each(|undex| {
            let index = *undex as isize;

            let col = (index - 1) % cols;
            let offset = cols - 1 - 2 * col;
            *undex = (index + offset) as usize;
        });
    }
}
