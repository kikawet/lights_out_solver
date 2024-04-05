mod args;
mod chain_of_responsability;

use args::CliArgs;
use chain_of_responsability::{
    chainable::Chainable,
    implementations::{
        print::PrintWorker, sanitize_input::SanitizeWorker, simulator::SimulatorWorker,
        solver::SolverWorker, validate_range::ValidateRangeWorker,
    },
    state::State,
    worker::Worker,
};
use clap::Parser;

use log::info;

use simple_logger::SimpleLogger;

fn main() {
    let input = CliArgs::parse();
    set_up_logger(&input);

    let mut worker = get_worker_chain(&input);
    let state = State::new(input);

    if let Some(err) = worker.execute(state).err() {
        err.exit();
    }
}

fn get_worker_chain(args: &CliArgs) -> Box<dyn Worker> {
    let mut validator = Box::<ValidateRangeWorker>::default();
    let sanitizer = Box::<SanitizeWorker>::default();

    let sanitizer = validator.set_next(sanitizer);

    if args.simulation_steps.is_empty() {
        let solver = Box::<SolverWorker>::default();
        let printer = Box::<PrintWorker>::default();
        sanitizer.set_next(solver).set_next(printer);
    } else {
        let simulator = Box::<SimulatorWorker>::default();
        sanitizer.set_next(simulator);
    }

    validator
}

fn set_up_logger(args: &CliArgs) {
    if args.verbose {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
        info!("Verbose mode enabled");
    }
}
