pub mod args;
// pub mod program;
pub mod solvers;

#[cfg(test)]
mod solver_tests {
    use crate::solvers::recursive::*;

    #[test]
    fn test_simulate_tl() {
        let mut test = vec![false; 9];

        simulate(&mut test, 0);

        const RESULT: [bool; 9] = [true, true, false, true, false, false, false, false, false];

        assert_eq!(test, RESULT);
    }

    #[test]
    fn test_simulate_mm() {
        let mut test = vec![false; 9];
        simulate(&mut test, 4);

        const RESULT: [bool; 9] = [false, true, false, true, true, true, false, true, false];

        assert_eq!(test, RESULT);
    }

    #[test]
    fn test_simulate_br_mm() {
        let mut test = vec![false; 9];
        simulate(&mut test, 4);
        simulate(&mut test, 8);

        let result: Vec<bool> = vec![false, true, false, true, true, false, false, false, true];

        assert_eq!(test, result);
    }

    #[test]
    fn test_simulate_ml_mr() {
        let mut test = vec![false; 9];
        simulate(&mut test, 3);
        simulate(&mut test, 5);

        let result: Vec<bool> = vec![true, false, true, true, false, true, true, false, true];

        assert_eq!(test, result);
    }

    #[test]
    fn test_is_solved() {
        let mut board = vec![
            false, false, false, false, false, false, false, false, false,
        ];

        let solution = solve(&board).unwrap();

        for step in solution.iter() {
            simulate(&mut board, *step);
        }

        let result = vec![true, true, true, true, true, true, true, true, true];

        assert_eq!(board, result);
    }

    #[test]
    fn test_minimun_solution() {
        let mut board = vec![true, false, true, false, false, false, true, false, true];

        let solution = solve(&board).unwrap();

        for step in solution.iter() {
            simulate(&mut board, *step);
        }

        let result = vec![true, true, true, true, true, true, true, true, true];

        assert_eq!(board, result);

        assert_eq!(solution, [4]);
    }
}

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
