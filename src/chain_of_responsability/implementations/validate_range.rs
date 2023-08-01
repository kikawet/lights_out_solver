use clap::error::ErrorKind;

use crate::chain_of_responsability::{
    chainable::Chainable, handler::Handler, state::State, worker::Worker,
};

#[derive(Default)]
pub struct ValidateRangeWorker {
    next: Option<Box<dyn Worker>>,
}

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

impl Chainable for ValidateRangeWorker {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker {
        &mut **self.next.insert(next)
    }

    fn next(&mut self) -> Option<&mut dyn Worker> {
        self.next.as_deref_mut()
    }
}
