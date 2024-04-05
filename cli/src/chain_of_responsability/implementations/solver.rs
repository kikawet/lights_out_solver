use log::debug;
use solvers::gf2;

use crate::{
    chain_of_responsability::{
        chainable::Chainable, handler::Handler, state::State, worker::Worker,
    },
    define_chainable,
};

define_chainable!(SolverWorker);

impl Handler for SolverWorker {
    fn handle(&self, mut state: State) -> Result<State, clap::error::Error> {
        debug!("Active lights: {:?}", state.args.lights);
        debug!("Rows: {:?}", state.args.rows);
        debug!("Cols: {:?}", state.args.cols);
        debug!("Origin location: {:?}", state.args.origin_location);

        debug!("Searching for solution ...");
        let board = state.board.as_deref().expect("Unable to access board");

        let solution = gf2::solve(board);
        debug!("Final solution: {:?}", &solution);

        state.solution = solution;

        Ok(state)
    }
}
