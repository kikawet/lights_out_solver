use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "Lights Out Puzzle Solver",
    version,
    about = "CLI program to solve Lights out puzzle",
    long_about = "CLI program created in Rust to solve Lights out puzzle. It finds the minimal solution and you as well run in simulation mode to check that the board is going to look after a number of steps",
    next_line_help = false
)]
pub struct Args {
    /// Indexes of the active lights
    ///
    /// Values in the range [1, cols*rows]
    pub lights: Vec<usize>,

    /// The number of rows
    ///
    /// Minimum allowed value: 1
    #[arg(short, long, default_value_t = 3)]
    pub rows: usize,

    /// The number of columns
    ///  
    /// Minimum allowed value: 1
    #[arg(short, long, default_value_t = 3)]
    pub cols: usize,

    /// Enable the debug logs
    ///
    /// Default: false
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Run a simulation with the given input
    ///
    /// Values in the range [1, cols*rows] of the positions to toggle
    #[arg(short, long, value_name = "STEPS")]
    pub simulation_steps: Vec<usize>,

    /// Sets the way you display the results
    #[arg(short, long, value_enum, value_name="MODE", default_value_t=Display::Draw)]
    pub display_mode: Display,

    /// Position of the starting index
    ///
    /// Changes where the first index is located in the matrix (eg: bl = bottom left), the default value is "Bottom left" to mimic a numpad
    #[arg(short, long, value_enum, value_name="LOCATION", default_value_t=Origin::BottomLeft)]
    pub origin_location: Origin,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Display {
    Simple,
    Draw,
    All,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Origin {
    /// Bottom Left
    #[value(name = "bl")]
    BottomLeft,
    /// Bottom Right
    #[value(name = "br")]
    BottomRight,
    /// Top Left
    #[value(name = "tl")]
    TopLeft,
    /// Top Right
    #[value(name = "tr")]
    TopRight,
}

#[cfg(test)]
mod args_tests {
    use clap::CommandFactory;
    use clap::Parser;

    use super::Args;
    use crate::args::Display;
    use crate::args::Origin;

    macro_rules! test_args {
        ($($arg:expr),*) => {
            vec!["<PROGRAM>", $($arg),*]
        };
    }

    #[test]
    fn verify_clap() {
        Args::command().debug_assert();
    }

    #[test]
    fn test_name() {
        assert_eq!(Args::command().get_name(), "Lights Out Puzzle Solver");
    }

    #[test]
    fn test_input_lights() {
        let input = Args::try_parse_from(test_args!("7", "9", "1", "3"))
            .expect("lights are not parsed properly");

        assert_eq!(input.lights, vec![7, 9, 1, 3]);
    }

    #[test]
    fn test_defaults() {
        let input = Args::try_parse_from(test_args!()).expect("lights are not parsed properly");

        assert_eq!(input.lights.len(), 0);
        assert_eq!(input.cols, 3);
        assert_eq!(input.rows, 3);
        assert!(!input.verbose);
        assert_eq!(input.simulation_steps.len(), 0);
        assert_eq!(input.display_mode, Display::Draw);
        assert_eq!(input.origin_location, Origin::BottomLeft);
    }
}
