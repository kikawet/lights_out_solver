use log::debug;

use crate::{
    args::Display,
    chain_of_responsability::{
        chainable::Chainable, handler::Handler, implementations::sanitize_input::SanitizeWorker,
        state::State, worker::Worker,
    },
    solvers::board::Board,
};

#[derive(Default)]
pub struct PrintWorker {
    next: Option<Box<dyn Worker>>,
}

impl PrintWorker {
    pub fn board_to_vec(board: &(impl Board + ?Sized)) -> Vec<String> {
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

    #[must_use]
    pub fn vec_to_str(board_as_char: &[String], cols: usize) -> String {
        let mut board_string = String::new();
        for (index, node) in board_as_char.iter().enumerate() {
            if index % cols == 0 {
                board_string.push('\n');
            }

            board_string.push_str(node);
        }

        board_string
    }
}

impl Handler for PrintWorker {
    fn handle(&mut self, state: State) -> Result<State, clap::error::Error> {
        let display_mode = state.input.display_mode;
        debug!("Display mode: {:?}", display_mode);
        let Some(solution) = &state.solution else { return Ok(state) };
        let board = state.board.as_deref().expect("Unable to access board");

        if display_mode == Display::Simple || display_mode == Display::All {
            // need to clone solution bc in display mode 'all' this is going to change the board
            let mut solution = solution.clone();
            solution.iter_mut().for_each(|val| *val += 1);

            let (cols, rows) = board.size();

            SanitizeWorker::rotate_light_indices(
                &mut solution,
                cols,
                rows,
                state.input.origin_location,
            );

            println!("{solution:?}");
        }

        if display_mode == Display::Draw || display_mode == Display::All {
            let mut mapped_board = Self::board_to_vec(board);

            for (order, position) in solution.iter().enumerate() {
                mapped_board[*position] = order.to_string();
            }

            println!("{}", Self::vec_to_str(&mapped_board, board.cols()));
        }

        Ok(state)
    }
}

impl Chainable for PrintWorker {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker {
        &mut **self.next.insert(next)
    }

    fn next(&mut self) -> Option<&mut dyn Worker> {
        self.next.as_deref_mut()
    }
}
