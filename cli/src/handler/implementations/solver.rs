use log::debug;
use solvers::gf2;

use crate::handler::r#trait::Handler;
use crate::handler::state::{SolvedState, ValidState};

pub struct SolverHandler;

impl Handler<ValidState, SolvedState> for SolverHandler {
    fn handle(&self, state: ValidState) -> Result<SolvedState, clap::error::Error> {
        debug!("Active lights: {:?}", state.args.lights);
        debug!("Rows: {:?}", state.args.rows);
        debug!("Cols: {:?}", state.args.cols);
        debug!("Origin location: {:?}", state.args.origin_location);

        debug!("Searching for solution ...");
        let solution = gf2::solve(state.board.as_ref());
        debug!("Final solution: {:?}", &solution);

        Ok(SolvedState {
            args: state.args,
            board: state.board,
            solution,
        })
    }
}
