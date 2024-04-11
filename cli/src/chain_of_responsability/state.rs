use solvers::{board::Board, Solution};

use crate::args::CliArgs;

pub struct State {
    pub args: CliArgs,
    pub board: Option<Box<dyn Board>>,
    pub solution: Solution,
}

impl State {
    #[must_use]
    pub fn new(args: CliArgs) -> Self {
        Self {
            args,
            board: None,
            solution: None,
        }
    }
}
