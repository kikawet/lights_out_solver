pub mod args;
// pub mod program;
pub mod solvers;

#[cfg(test)]
mod args_tests {
    use crate::args::Display;
    use crate::args::Input;
    use crate::args::Origin;
    use clap::CommandFactory;
    use clap::Parser;

    macro_rules! test_args {
        ($($arg:expr),*) => {
            vec!["<PROGRAM>", $($arg),*]
        };
    }

    #[test]
    fn verify_clap() {
        Input::command().debug_assert();
    }

    #[test]
    fn test_name() {
        assert_eq!(Input::command().get_name(), "Lights Out Puzzle Solver");
    }

    #[test]
    fn test_input_lights() {
        let input = Input::try_parse_from(test_args!("7", "9", "1", "3"))
            .expect("ligths are not parsed properly");

        assert_eq!(input.lights, vec![7, 9, 1, 3]);
    }

    #[test]
    fn test_defaults() {
        let input = Input::try_parse_from(test_args!()).expect("ligths are not parsed properly");

        assert_eq!(input.lights.len(), 0);
        assert_eq!(input.cols, 3);
        assert_eq!(input.rows, 3);
        assert_eq!(input.verbose, false);
        assert_eq!(input.simulation_steps.len(), 0);
        assert_eq!(input.display_mode, Display::Draw);
        assert_eq!(input.origin_location, Origin::BottomLeft);
    }
}
