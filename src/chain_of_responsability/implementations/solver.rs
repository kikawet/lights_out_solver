use log::debug;

use crate::{
    chain_of_responsability::{
        chainable::Chainable, handler::Handler, state::State, worker::Worker,
    },
    define_chainable,
    solvers::gf2,
};

define_chainable!(SolverWorker);

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
