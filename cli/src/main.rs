mod args;
mod handler;

use args::Args;
use clap::{Error, Parser};
use handler::{
    implementations::{
        print::PrintHandler, sanitize_input::SanitizeHandler, simulator::SimulatorHandler,
        solver::SolverHandler, validate::ValidateHandler,
    },
    state::State,
};

use log::info;

use crate::handler::r#trait::Handler;
use crate::handler::state::{SolvedState, ValidState};
use simple_logger::SimpleLogger;

#[macro_export]
macro_rules! chain {
    ($state:ident, $first:expr, $( $handler:expr ),* ) => {
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

    // let chain = get_handler_chain(&input);
    let run_solver = input.simulation_steps.is_empty();
    let state = State::new(input);

    let chain = chain!(state, ValidateHandler, SanitizeHandler, Branch(run_solver));

    if let Some(err) = chain.err() {
        err.exit();
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
struct MappingHandler;

impl Handler<ValidState, SolvedState> for MappingHandler {
    fn handle(&self, state: ValidState) -> Result<SolvedState, Error> {
        Ok(SolvedState {
            args: state.args,
            board: state.board,
            solution: None,
        })
    }
}

#[derive(Default)]
struct Branch(bool);

impl Handler<ValidState, SolvedState> for Branch {
    fn handle(&self, state: ValidState) -> Result<SolvedState, Error> {
        if self.0 {
            chain!(state, SolverHandler, PrintHandler)
        } else {
            chain!(state, SimulatorHandler, MappingHandler)
        }
    }
}
