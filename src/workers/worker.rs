use clap::Command;

use crate::{args::Input, solvers::board::Board};

pub struct State {
    pub input: Input,
    pub board: Option<Box<dyn Board>>,
    pub command: Command,
    pub solution: Option<Vec<usize>>,
}

pub trait Handler {
    fn handle(&mut self, state: State) -> Result<State, clap::Error>;
}

pub trait Chainable {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker;
    fn next(&mut self) -> Option<&mut dyn Worker>;
}

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
