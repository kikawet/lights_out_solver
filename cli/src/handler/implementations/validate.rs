use clap::error::ErrorKind;
use solvers::board::Binary;

use crate::handler::r#trait::Handler;
use crate::handler::state::{State, ValidState};

pub struct ValidateHandler;

impl Handler<State, ValidState> for ValidateHandler {
    fn handle(&self, state: State) -> Result<ValidState, clap::error::Error> {
        let rows = state.args.rows;
        let cols = state.args.cols;
        let max_value = rows * cols;

        if let Some(out_of_range) = state.args.lights.iter().find(|&&it| it > max_value) {
            return Err(clap::Error::raw(
                ErrorKind::ArgumentConflict,
                format!("Index {out_of_range} out of range for a {rows}x{cols} size"),
            ));
        }

        let board = Box::new(Binary::new_from_positions(&state.args.lights, cols, rows));
        Ok(ValidState {
            args: state.args,
            board,
        })
    }
}
