use lights_out_solver::args::{init_app, CommandArgs, ProgramArgs};
use lights_out_solver::program::Program;
use log::info;

use simple_logger::SimpleLogger;

fn main() {
    let mut program = Program::new(init_app());
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
