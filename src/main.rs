use lights_out_solver::args::{init_app, ProgramArgs};
use lights_out_solver::solver::{simulate, solve};
use lights_out_solver::program::{Program, self};
use log::{debug, info};

use clap::{ArgMatches, Command, ErrorKind};

use simple_logger::SimpleLogger;

fn main() {
    let mut program = Program::new(init_app()) ;
    set_up_logger(&program);

    program.run();
}

fn set_up_logger(program: &Program) {
    if program.is_enabled(ProgramArgs::Verbose.id()) {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
        info!("Verbose mode enabled");
    }
}