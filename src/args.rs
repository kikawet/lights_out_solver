extern crate clap;
use clap::{Arg, builder::PossibleValuesParser, value_parser, command, Command};

pub fn init_app() -> Command<'static> {
    command!()
    .name("Lights Out Puzzle Solver")
    .version("0.1.0")
    .about("With the given input of on node it will output the order to toggle the lights to solve the puzzle") 
    .arg(
        Arg::new("NODES")
            .help("Indexes of the active nodes starting at 0 on the top left")
            .multiple_values(true)
            .index(1)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        Arg::new("rows")
            .help("The number of rows")
            .short('r')
            .value_name("rows")
            .takes_value(true)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        Arg::new("cols")
            .help("The number of columns")
            .short('c')
            .value_name("cols")
            .takes_value(true)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        Arg::new("verbose")
            .help("Use verbose output")
            .short('v')
            .value_name("verbose")
            .takes_value(false)
    )
    .arg(
        Arg::new("run_simulation")
            .help("Run a simulation of the puzzle")
            .short('s')
            .value_name("postions_to_trigger")
            .multiple_values(true)
            .takes_value(true)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        Arg::new("output_mode")
            .help("Sets the output mode")
            .short('m')
            .value_name("output_mode")
            .takes_value(false)
            .value_parser(PossibleValuesParser::new(["simple", "draw", "all"]))
            .default_value("draw"),
    )
}
