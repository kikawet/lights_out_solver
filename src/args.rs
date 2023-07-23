use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "Lights Out Puzzle Solver",
    version,
    about = "CLI program to solve Lights out puzzle",
    long_about = "CLI program created in Rust to solve Lights out puzzle. It finds the minimal solution and you as well run in simulation mode to check that the board is going to look after a number of steps",
    next_line_help = false
)]
pub struct Input {
    /// Indexes of the active lights
    ///
    /// Range from 1 to [cols]*[rows]
    pub lights: Vec<usize>,

    /// The number of rows
    ///
    /// Minimun allowed value: 1
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u64).range(1..))]
    pub rows: u64,
    /// The number of columns
    ///  
    /// Minimun allowed value: 1
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u64).range(1..))]
    pub cols: u64,
    /// Enable the debug logs
    ///
    /// Default: false
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    /// Run a simulation with the given input
    ///
    /// Range from 1 to [cols]*[rows] of the positions to toggle
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
