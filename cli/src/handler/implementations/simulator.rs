use super::print::PrintHandler;
use crate::handler::r#trait::Handler;
use crate::handler::state::ValidState;
use log::debug;
use solvers::board::Board;

pub struct SimulatorHandler;

impl SimulatorHandler {
    fn prettify_board(board: &(impl Board + ?Sized)) -> String {
        PrintHandler::vec_to_str(&PrintHandler::board_to_vec(board), board.cols())
    }
}

impl Handler<ValidState, ValidState> for SimulatorHandler {
    fn handle(&self, mut state: ValidState) -> Result<ValidState, clap::error::Error> {
        let board = state.board.as_mut();
        let steps = &state.args.simulation_steps;
        debug!(
            "Board before the simulation:\n {}",
            Self::prettify_board(board)
        );
        debug!("Steps to simulate: {steps:?}");

        for (step, node_to_trigger) in steps.iter().enumerate() {
            board.trigger_index(*node_to_trigger);
            debug!("Step {}:\n {}", step, Self::prettify_board(board));
        }

        debug!("Board after simulation: {}", Self::prettify_board(board));

        Ok(state)
    }
}
