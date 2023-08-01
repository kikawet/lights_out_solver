use clap::{CommandFactory, Parser};
use lights_out_solver::{
    args::Input,
    workers::{
        implementations::{
            print::PrintWorker, sanitize_input::SanitizeWorker, simulator::SimulatorWorker,
            solver::SolverWorker, validate_range::ValidateRangeWorker,
        },
        worker::{Chainable, State, Worker},
    },
};
use log::info;

use simple_logger::SimpleLogger;

fn main() {
    let input = Input::parse();
    set_up_logger(&input);

    let mut worker = get_worker_chain(&input);
    let state = build_state(input);

    worker.execute(state).expect("okey dokey");
}

fn get_worker_chain(input: &Input) -> Box<dyn Worker> {
    let mut validator = Box::<ValidateRangeWorker>::default();
    let sanitizer = Box::<SanitizeWorker>::default();

    let sanitizer = validator.set_next(sanitizer);

    if input.simulation_steps.is_empty() {
        let solver = Box::<SolverWorker>::default();
        let printer = Box::<PrintWorker>::default();
        sanitizer.set_next(solver).set_next(printer);
    } else {
        let simulator = Box::<SimulatorWorker>::default();
        sanitizer.set_next(simulator);
    }

    validator
}

fn build_state(input: Input) -> State {
    State {
        input,
        board: None,
        command: Input::command(),
        solution: None,
    }
}

fn set_up_logger(input: &Input) {
    if input.verbose {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
        info!("Verbose mode enabled");
    }
}
