use super::{chainable::Chainable, handler::Handler, state::State};

pub trait Worker: 'static + Chainable + Handler {
    fn execute(&mut self, state: State) -> Result<State, clap::Error>;
}

impl<T: Handler + Chainable + 'static> Worker for T {
    fn execute(&mut self, state: State) -> Result<State, clap::Error> {
        let new_state = self.handle(state)?;

        if let Some(next) = self.next() {
            return next.execute(new_state);
        }

        Ok(new_state)
    }
}
