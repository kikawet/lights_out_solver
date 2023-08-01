use clap::{Command, CommandFactory};

use crate::{args::Input, solvers::board::Board};

pub struct State {
    pub input: Input,
    pub board: Option<Box<dyn Board>>,
    pub command: Command,
    pub solution: Option<Vec<usize>>,
}

impl State {
    pub fn new(input: Input) -> Self {
        Self {
            input,
            board: None,
            command: Input::command(),
            solution: None,
        }
    }
}
