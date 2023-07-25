use clap::Parser;
use lights_out_solver::{args::Input, program::Program};
use log::info;

use simple_logger::SimpleLogger;

fn main() {
    let input = Input::parse();
    set_up_logger(&input);

    let mut program = Program::new(input);

    program.run();
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
