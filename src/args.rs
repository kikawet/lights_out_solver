extern crate clap;

use clap::{
    builder::PossibleValuesParser, command, value_parser, Arg, ArgAction, ArgMatches, Command,
};

macro_rules! new_argument {
    ($program_arg:path) => {
        Arg::new($program_arg.name()).help($program_arg.help())
    };
}

macro_rules! new_option {
    ($program_arg:path) => {
        new_argument!($program_arg)
            .short($program_arg.short())
            .long($program_arg.long())
            .value_name($program_arg.value_name())
    };
}

fn get_one_match<T>(matches: &ArgMatches, arg: &impl CommandArgs) -> T
where
    T: Sized + Clone + Send + Sync + 'static,
{
    matches
        .get_one::<T>(arg.id())
        .unwrap_or_else(|| panic!("Failed to get required command argument {}", arg.id()))
        .to_owned()
}

pub trait Matcheable<T>
where
    T: Sized + Clone + Send + Sync + 'static,
    Self: CommandArgs + Sized,
{
    fn get_match_from(&self, matches: &ArgMatches) -> T {
        get_one_match(matches, self)
    }
}

impl<T> Matcheable<T> for ProgramArgs where T: Sized + Clone + Send + Sync + 'static {}

pub enum ProgramArgs {
    Lights,
    Rows,
    Cols,
    Verbose,
    RunSimulation,
    DisplayMode,
    InputMode,
}
pub trait CommandArgs {
    fn id(&self) -> &'static str {
        self.name()
    }

    fn name(&self) -> &'static str;
    fn help(self) -> &'static str;
    fn short(self) -> char;
    fn long(self) -> &'static str;
    fn value_name(self) -> &'static str
    where
        Self: Sized,
    {
        self.name()
    }
}

impl CommandArgs for ProgramArgs {
    fn name(&self) -> &'static str {
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

    fn help(self) -> &'static str {
        match self {
            ProgramArgs::Lights => "Indexes of the active lights (range from 1 to [cols]*[rows])",
            ProgramArgs::Rows => "The number of rows",
            ProgramArgs::Cols => "The number of columns",
            ProgramArgs::Verbose => "Enable the debug logs",
            ProgramArgs::RunSimulation => "Run a simulation with the given input",
            ProgramArgs::DisplayMode => "Sets the way you display the results",
            ProgramArgs::InputMode => {
                "Changes where the first index is located in the matrix (eg: bl = bottom left)"
            }
        }
    }

    fn short(self) -> char {
        match self {
            ProgramArgs::Rows => 'r',
            ProgramArgs::Cols => 'c',
            ProgramArgs::Verbose => 'v',
            ProgramArgs::RunSimulation => 's',
            ProgramArgs::DisplayMode => 'd',
            ProgramArgs::InputMode => 'i',
            _ => unreachable!(),
        }
    }

    fn long(self) -> &'static str {
        match self {
            ProgramArgs::Rows => "rows",
            ProgramArgs::Cols => "cols",
            ProgramArgs::Verbose => "verbose",
            ProgramArgs::RunSimulation => "simulate",
            ProgramArgs::DisplayMode => "display",
            ProgramArgs::InputMode => "input",
            _ => unreachable!(),
        }
    }

    fn value_name(self) -> &'static str {
        match self {
            ProgramArgs::DisplayMode | ProgramArgs::InputMode => "mode",
            ProgramArgs::RunSimulation => "steps",
            _ => self.name(),
        }
    }
}

pub fn init_app() -> Command {
    command!()
    .name("Lights Out Puzzle Solver")
    .about("It finds the minimal solution and you aswell run in simulation mode to check that the board is going to look after a number of steps") 
    .arg(
        new_argument!(ProgramArgs::Lights)
            .num_args(1..)
            .index(1)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_option!(ProgramArgs::Rows)            
            .action(ArgAction::Set)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_option!(ProgramArgs::Cols)
            .action(ArgAction::Set)
            .default_value("3")
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_option!(ProgramArgs::Verbose)
            .action(ArgAction::SetTrue)
    )
    .arg(
        new_option!(ProgramArgs::RunSimulation)
            .num_args(1..)
            .value_parser(value_parser!(usize)),
    )
    .arg(
        new_option!(ProgramArgs::DisplayMode)
            .action(ArgAction::Set)
            .value_parser(PossibleValuesParser::new(["simple", "draw", "all"]))
            .default_value("draw"),
    )
    .arg(
        new_option!(ProgramArgs::InputMode)
            .action(ArgAction::Set)
            .value_parser(PossibleValuesParser::new(["tl", "tr", "bl", "br"]))
            .default_value("bl"),
    )
}
