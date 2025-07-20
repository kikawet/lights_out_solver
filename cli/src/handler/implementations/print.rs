use std::io::Write;

use log::debug;
use solvers::board::Board;

use crate::args::Display;
use crate::handler::implementations::sanitize_input::SanitizeHandler;
use crate::handler::r#trait::HandlerMut;
use crate::handler::state::SolvedState;

pub struct PrintHandler<'a, T = ()> {
    pub out: &'a mut T,
}

impl<T> PrintHandler<'_, T> {
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

impl<T> HandlerMut<SolvedState, SolvedState> for PrintHandler<'_, T>
where
    T: Write,
{
    fn handle(&mut self, state: SolvedState) -> Result<SolvedState, clap::error::Error> {
        let display_mode = state.args.display_mode;
        debug!("Display mode: {display_mode:?}");
        let Some(solution) = &state.solution else {
            return Ok(state);
        };
        let mut solution = solution.clone();
        let (cols, rows) = state.board.size();

        SanitizeHandler::rotate_light_indices(
            &mut solution,
            cols,
            rows,
            state.args.origin_location,
        );

        if display_mode == Display::Simple || display_mode == Display::All {
            let mut solution = solution.clone();
            solution.iter_mut().for_each(|val| *val += 1);
            solution.sort_unstable();

            writeln!(&mut self.out, "{solution:?}")?;
        }

        if display_mode == Display::Draw || display_mode == Display::All {
            let mut mapped_board = Self::board_to_vec(state.board.as_ref());

            for (order, position) in solution.iter().enumerate() {
                mapped_board[*position] = order.to_string();
            }

            writeln!(
                &mut self.out,
                "{}",
                Self::vec_to_str(&mapped_board, state.board.cols())
            )?;
        }

        Ok(state)
    }
}
