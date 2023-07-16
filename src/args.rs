extern crate clap;

use clap::{Arg, builder::PossibleValuesParser, value_parser, command, Command, ArgAction};

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
            ProgramArgs::Lights => "Indexes of the active lights (range from 1 to [cols]*[rows])",
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

pub fn init_app() -> Command {
    command!()
    .name("Lights Out Puzzle Solver")
    .version("1.0.0")
    .about("It finds the minimal solution and you aswell run in simulation mode to check that the board is going to look after a number of steps") 
    .arg(
        new_basic_arg!(ProgramArgs::Lights)
            .num_args(1..)
            .index(1)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::Rows)            
            .action(ArgAction::Set)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::Cols)
            .action(ArgAction::Set)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::Verbose)
            .action(ArgAction::SetTrue)
    )
    .arg(
        new_arg!(ProgramArgs::RunSimulation)
            .num_args(1..)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_arg!(ProgramArgs::DisplayMode)
            .action(ArgAction::Set)
            .value_parser(PossibleValuesParser::new(["simple", "draw", "all"]))
            .default_value("draw"),
    )
    .arg(
        new_arg!(ProgramArgs::InputMode)
            .action(ArgAction::Set)
            .value_parser(PossibleValuesParser::new(["tl", "tr", "bl", "br"]))
            .default_value("bl"),
    )
}
