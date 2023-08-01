use log::debug;

use crate::{
    solvers::board::Board,
    workers::worker::{Chainable, Handler, State, Worker},
};

use super::print::PrintWorker;

#[derive(Default)]
pub struct SimulatorWorker {
    next: Option<Box<dyn Worker>>,
}

impl SimulatorWorker {
    fn prettify_board(board: &(impl Board + ?Sized)) -> String {
        PrintWorker::vec_to_str(&PrintWorker::board_to_vec(board), board.cols())
    }
}

impl Handler for SimulatorWorker {
    fn handle(&mut self, mut state: State) -> Result<State, clap::error::Error> {
        let board = state.board.as_deref_mut().expect("Unable to access board");
        let steps = &state.input.simulation_steps;
        debug!(
            "Board before the simulation:\n {}",
            Self::prettify_board(board)
        );
        debug!("Steps to simulate: {:?}", steps);

        for (step, node_to_trigger) in steps.iter().enumerate() {
            board.trigger_index(*node_to_trigger);
            debug!("Step {}:\n {}", step, Self::prettify_board(board));
        }

        debug!("Board after simulation: {}", Self::prettify_board(board));

        Ok(state)
    }
}

impl Chainable for SimulatorWorker {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker {
        &mut **self.next.insert(next)
    }

    fn next(&mut self) -> Option<&mut dyn Worker> {
        self.next.as_deref_mut()
    }
}
