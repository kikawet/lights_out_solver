mod args;
mod handler;

use args::Args;
use clap::{Error, Parser};
use log::info;
use simple_logger::SimpleLogger;

use crate::handler::implementations::print::PrintHandler;
use crate::handler::implementations::sanitize_input::SanitizeHandler;
use crate::handler::implementations::simulator::SimulatorHandler;
use crate::handler::implementations::solver::SolverHandler;
use crate::handler::implementations::validate::ValidateHandler;
use crate::handler::r#trait::Handler;
use crate::handler::state::{SolvedState, State, ValidState};

#[macro_export]
macro_rules! chain {
    ($state:ident, [$first:expr, $( $handler:expr ),*]) => {
        {
            $first.handle($state)
            $(
                .and_then(|s| $handler.handle(s))
            )*
        }
    };
}

fn main() {
    let input = Args::parse();
    set_up_logger(&input);

    let chain = run_handlers(input);

    if let Some(err) = chain.err() {
        err.exit();
    }
}

fn run_handlers(input: Args) -> Result<SolvedState, Error> {
    let run_solver = input.simulation_steps.is_empty();
    let state = State::new(input);

    if run_solver {
        chain!(
            state,
            [
                ValidateHandler,
                SanitizeHandler,
                SolverHandler,
                PrintHandler
            ]
        )
    } else {
        chain!(
            state,
            [
                ValidateHandler,
                SanitizeHandler,
                SimulatorHandler,
                MockSolverHandler
            ]
        )
    }
}

fn set_up_logger(args: &Args) {
    if args.verbose {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
        info!("Verbose mode enabled");
    }
}

#[derive(Default)]
struct MockSolverHandler;

impl Handler<ValidState, SolvedState> for MockSolverHandler {
    fn handle(&self, state: ValidState) -> Result<SolvedState, Error> {
        Ok(SolvedState {
            args: state.args,
            board: state.board,
            solution: None,
        })
    }
}
