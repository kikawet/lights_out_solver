use crate::args::{Display, Input, Origin};
use crate::solvers::board::{Binary, Board};
use crate::solvers::gf2;
use clap::error::ErrorKind;
use clap::CommandFactory;
use log::debug;
pub struct Program {
    input: Input,
    board: Box<dyn Board>,
}

impl Program {
    #[must_use]
    pub fn new(input: Input) -> Self {
        Self {
            input,
            board: Box::new(Binary::new_blank(0, 0)),
        }
    }

    fn load_data(&mut self) {
        self.prepare_board_data();
        self.prepare_simulation_data();

        let input_mode = &self.input.origin_location;
        let cols = self.input.cols;
        let rows = self.input.rows;

        self.board = Box::new(Binary::new_from_positions(&self.input.lights, cols, rows));

        debug!("Active lights: {:?}", self.input.lights);
        debug!("Rows: {:?}", rows);
        debug!("Cols: {:?}", cols);
        debug!("Origin location: {:?}", input_mode);
    }

    fn prepare_board_data(&mut self) {
        let rows = self.input.rows;
        let cols = self.input.cols;
        let nodes = &mut self.input.lights;
        nodes.sort_unstable();
        nodes.dedup();

        Self::validate_indices(nodes, rows, cols);

        Self::rotate_light_indices(nodes, cols, rows, self.input.origin_location);

        // convert from range 1..[cols]*[rows] to 0..[cols]*[rows]-1
        for val in nodes {
            *val -= 1;
        }
    }

    fn prepare_simulation_data(&mut self) {
        let rows = self.input.rows;
        let cols = self.input.cols;
        let simulation_steps = &mut self.input.simulation_steps;

        Self::validate_range_indices(simulation_steps, rows, cols);

        Self::rotate_light_indices(simulation_steps, cols, rows, self.input.origin_location);

        simulation_steps.iter_mut().for_each(|val| *val -= 1);
    }

    fn validate_range_indices(active_nodes: &[usize], rows: usize, cols: usize) {
        let max_value = rows * cols;

        if let Some(out_of_range) = active_nodes.iter().find(|&&it| it > max_value) {
            Input::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    format!("Index {out_of_range} out of range for a {rows}x{cols} size"),
                )
                .exit();
        }
    }

    fn validate_indices(active_nodes: &Vec<usize>, rows: usize, cols: usize) {
        let max_nodes = rows * cols;

        if active_nodes.len() > max_nodes {
            Input::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    format!(
                        "Too many parameters given. The maximum number of nodes is {max_nodes}"
                    ),
                )
                .exit();
        }

        Self::validate_range_indices(active_nodes, rows, cols);
    }

    fn prettify_board(&self, board: &dyn Board) -> String {
        let mapped_board = Self::map_board(board);

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

    fn map_board(board: &dyn Board) -> Vec<String> {
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

        if self.input.simulation_steps.is_empty() {
            let solution = self.run_solver();
            self.print_solution(self.board.as_ref(), solution, self.input.display_mode);
        } else {
            self.run_simulation();
        }
    }

    fn print_solution(&self, board: &dyn Board, solution: Option<Vec<usize>>, draw_mode: Display) {
        debug!("Draw mode: {:?}", draw_mode);

        if draw_mode == Display::Simple || draw_mode == Display::All {
            // need to clone solution bc in display mode 'all' this is going to change the board
            if let Some(result) = &mut solution.clone() {
                result.iter_mut().for_each(|val| *val += 1);

                let (cols, rows) = board.size();

                Self::rotate_light_indices(result, cols, rows, self.input.origin_location);

                println!("{result:?}");
            } else {
                println!("{:?}", &solution);
            }
        }

        if draw_mode == Display::Draw || draw_mode == Display::All {
            let mut mapped_board = Self::map_board(board);

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
        debug!("Steps to simulate: {:?}", self.input.simulation_steps);

        for (step, node_to_trigger) in self.input.simulation_steps.iter().enumerate() {
            self.board.trigger_index(*node_to_trigger);
            debug!(
                "Step {}:\n {}",
                step,
                self.prettify_board(self.board.as_ref())
            );
        }

        debug!(
            "Board after simulation: {}",
            self.prettify_board(self.board.as_ref())
        );

        print!("{}", self.prettify_board(self.board.as_ref()));
    }

    /**
     * Transformation are symectric so calling this twice with the same state is going to undo the changes
     */
    fn rotate_light_indices(indices: &mut [usize], cols: usize, rows: usize, location: Origin) {
        match location {
            Origin::TopRight => Self::reorder_cols(indices, cols),
            Origin::BottomLeft => Self::reorder_rows(indices, rows),
            Origin::BottomRight => {
                Self::reorder_cols(indices, cols);
                Self::reorder_rows(indices, rows);
            }
            Origin::TopLeft => { /*Do nothing 👀*/ }
        };
    }

    fn reorder_rows(indices: &mut [usize], rows: usize) {
        let rows = rows as isize;

        for undex in indices.iter_mut() {
            let index = *undex as isize;

            let row = (index - 1) / rows;
            let offset = rows * (rows - 1 - 2 * row);
            *undex = (index + offset) as usize;
        }
    }

    fn reorder_cols(indices: &mut [usize], cols: usize) {
        let cols = cols as isize;

        for undex in indices.iter_mut() {
            let index = *undex as isize;

            let col = (index - 1) % cols;
            let offset = cols - 1 - 2 * col;
            *undex = (index + offset) as usize;
        }
    }
}
