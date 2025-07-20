use solvers::{board::Board, Solution};

use crate::args::Args;

pub struct State {
    pub args: Args,
}

pub struct ValidState {
    pub args: Args,
    pub board: Box<dyn Board>,
}

pub struct SolvedState {
    pub args: Args,
    pub board: Box<dyn Board>,
    pub solution: Solution,
}

impl State {
    #[must_use]
    pub fn new(args: Args) -> Self {
        Self { args }
    }
}
