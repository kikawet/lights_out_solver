extern crate clap;

use clap::{Arg, builder::PossibleValuesParser, value_parser, command, Command};

macro_rules! new_basic_arg {
    ($program_arg:path) => {
        Arg::new($program_arg.name())
            .help($program_arg.help())
    };
}

macro_rules! new_arg {
    ($program_arg:path) => {
        new_basic_arg!($program_arg)
        .short($program_arg.short())
        .long($program_arg.long())
        .value_name($program_arg.value_name())
    };
}

pub enum ProgramArgs {
    Lights,
    Rows,
    Cols,
    Verbose,
    RunSimulation,
    DisplayMode,
    InputMode,
}

impl ProgramArgs {
    pub fn id(self) -> &'static str{
        self.name()
    }

    pub fn name(self) -> &'static str{
        match self {
            ProgramArgs::Lights => "lights",
            ProgramArgs::Rows => "rows",
            ProgramArgs::Cols => "cols",
            ProgramArgs::Verbose => "verbose",
            ProgramArgs::RunSimulation => "run_simulation",
            ProgramArgs::DisplayMode => "display_mode",
            ProgramArgs::InputMode => "input_mode",
        }
    }

    fn help(self) -> &'static str{
        match self {
            ProgramArgs::Lights => "Indexes of the active lights (range from 0 to [cols]*[rows])",
            ProgramArgs::Rows => "The number of rows",
            ProgramArgs::Cols => "The number of columns",
            ProgramArgs::Verbose => "Enable the debug logs",
            ProgramArgs::RunSimulation => "Run a simulation with the given input",
            ProgramArgs::DisplayMode => "Sets the way you display the results",
            ProgramArgs::InputMode => "Changes where the first index is located in the matrix (eg: bl = bottom left)",
        }
    }

    fn short(self) -> char{
        match self {
            ProgramArgs::Rows => 'r',
            ProgramArgs::Cols => 'c',
            ProgramArgs::Verbose => 'v',
            ProgramArgs::RunSimulation => 's',
            ProgramArgs::DisplayMode => 'd',
            ProgramArgs::InputMode => 'i',
            _ => unreachable!()
        }
    }

    fn long(self) -> &'static str{
        match self {
            ProgramArgs::Rows => "rows",
            ProgramArgs::Cols => "cols",
            ProgramArgs::Verbose => "verbose",
            ProgramArgs::RunSimulation => "simulate",
            ProgramArgs::DisplayMode => "display",
            ProgramArgs::InputMode => "input",
            _ => unreachable!()
        }
    }

    fn value_name(self) -> &'static str{
        match self {
            ProgramArgs::DisplayMode | ProgramArgs::InputMode => "mode",
            ProgramArgs::RunSimulation => "steps",
            _ => self.name()
        }        
    }
}

pub fn init_app() -> Command<'static> {
    command!()
    .name("Lights Out Puzzle Solver")
    .version("0.1.0")
    .about("With the given input of on node it will output the order to toggle the lights to solve the puzzle") 
    .arg(
        new_basic_arg!(ProgramArgs::Lights)
            .multiple_values(true)
            .index(1)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::Rows)            
            .takes_value(true)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::Cols)
            .takes_value(true)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::Verbose)
            .takes_value(false)
    )
    .arg(
        new_arg!(ProgramArgs::RunSimulation)
            .multiple_values(true)
            .takes_value(true)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::DisplayMode)
            .takes_value(true)
            .value_parser(PossibleValuesParser::new(["simple", "draw", "all"]))
            .default_value("draw"),
    )
    // .arg(
    //     new_arg!(ProgramArgs::InputMode)
    //         .takes_value(true)
    //         .value_parser(PossibleValuesParser::new(["tl", "tr", "bl", "br"]))
    //         .multiple_values(false)
    //         .multiple_occurrences(false)
    //         .default_value("bl"),
    // )
}
