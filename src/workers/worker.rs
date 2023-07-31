use clap::Command;

use crate::{args::Input, solvers::board::Board};

pub struct State {
    pub input: Input,
    pub board: Option<Box<dyn Board>>,
    pub command: Command,
    pub solution: Option<Vec<usize>>,
}

pub trait Worker: 'static {
    fn execute(&mut self, state: &mut State) -> Result<(), clap::Error> {
        self.handle(state)?;

        if let Some(next) = self.next() {
            return next.execute(state);
        }

        Ok(())
    }

    fn handle(&mut self, state: &mut State) -> Result<(), clap::Error>;
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker;
    fn next(&mut self) -> Option<&mut dyn Worker>;
}
