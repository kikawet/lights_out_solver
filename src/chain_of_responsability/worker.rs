use super::{chainable::Chainable, handler::Handler, state::State};

pub trait Worker: Chainable + Handler {
    /// # Errors
    ///
    /// Only handlers will return Error
    fn execute(&mut self, state: State) -> Result<State, clap::Error>;
}

impl<T: Handler + Chainable> Worker for T {
    fn execute(&mut self, state: State) -> Result<State, clap::Error> {
        let new_state = self.handle(state)?;

        if let Some(next) = self.next() {
            return next.execute(new_state);
        }

        Ok(new_state)
    }
}
