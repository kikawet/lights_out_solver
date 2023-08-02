use clap::error::ErrorKind;

use crate::{
    chain_of_responsability::{
        chainable::Chainable, handler::Handler, state::State, worker::Worker,
    },
    define_chainable,
};

define_chainable!(ValidateRangeWorker);

impl Handler for ValidateRangeWorker {
    fn handle(&mut self, mut state: State) -> Result<State, clap::error::Error> {
        let rows = state.input.rows;
        let cols = state.input.cols;
        let max_value = rows * cols;

        if let Some(out_of_range) = state.input.lights.iter().find(|&&it| it > max_value) {
            return Err(state.command.error(
                ErrorKind::ArgumentConflict,
                format!("Index {out_of_range} out of range for a {rows}x{cols} size"),
            ));
        }

        Ok(state)
    }
}
