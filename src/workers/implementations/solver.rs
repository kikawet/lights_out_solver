use log::debug;

use crate::{
    solvers::gf2,
    workers::worker::{Chainable, Handler, State, Worker},
};

#[derive(Default)]
pub struct SolverWorker {
    next: Option<Box<dyn Worker>>,
}

impl Handler for SolverWorker {
    fn handle(&mut self, mut state: State) -> Result<State, clap::error::Error> {
        debug!("Active lights: {:?}", state.input.lights);
        debug!("Rows: {:?}", state.input.rows);
        debug!("Cols: {:?}", state.input.cols);
        debug!("Origin location: {:?}", state.input.origin_location);

        debug!("Searching for solution ...");
        let board = state.board.as_deref().expect("Unable to access board");

        let solution = gf2::solve(board);
        debug!("Final solution: {:?}", &solution);

        state.solution = solution;

        Ok(state)
    }
}

impl Chainable for SolverWorker {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker {
        &mut **self.next.insert(next)
    }

    fn next(&mut self) -> Option<&mut dyn Worker> {
        self.next.as_deref_mut()
    }
}
