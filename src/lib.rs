pub mod args;
pub mod program;
pub mod solvers;

#[cfg(test)]
mod solver_tests {
    use crate::solvers::recursive::*;

    #[test]
    fn test_simulate_tl() {
        let mut test = vec![false; 9];

        simulate(&mut test, 0);
        
        const RESULT: [bool; 9] = [
            true,   true ,  false,
            true,   false,  false,
            false,  false,  false
        ];

        assert_eq!(test, RESULT);
    }

    #[test]
    fn test_simulate_mm() {
        let mut test = vec![false; 9];
        simulate(&mut test, 4);
        
        const RESULT: [bool; 9] = [
            false,  true ,  false,
            true,   true,   true,
            false,  true,   false
        ];

        assert_eq!(test, RESULT);
    }

    #[test]
    fn test_simulate_br_mm() {
        let mut test = vec![false; 9];
        simulate(&mut test, 4);
        simulate(&mut test, 8);
        
        let result: Vec<bool> = vec![
            false,  true ,  false,
            true,   true,   false,
            false,  false,  true
        ];

        assert_eq!(test, result);
    }

    #[test]
    fn test_simulate_ml_mr() {
        let mut test = vec![false; 9];
        simulate(&mut test, 3);
        simulate(&mut test, 5);
        
        let result: Vec<bool> = vec![
            true,  false,  true,
            true,  false,  true,
            true,  false,  true
        ];

        assert_eq!(test, result);
    }

    #[test]
    fn test_is_solved() {
        let mut board = vec![
            false,  false ,  false,
            false,  false,   false,
            false,  false,   false
        ];

        let solution = solve(&board).unwrap();

        for step in solution.iter() {
            simulate(&mut board, *step);
        }

        let result = vec![
            true,  true ,  true,
            true,  true,   true,
            true,  true,   true
        ];

        assert_eq!(board, result);
    }

    #[test]
    fn test_minimun_solution() {
        let mut board =  vec![
            true,  false ,  true,
            false,  false,   false,
            true,  false,   true
        ];

        let solution = solve(&board).unwrap();

        for step in solution.iter() {
            simulate(&mut board, *step);
        }

        let result = vec![
            true,  true ,  true,
            true,  true,   true,
            true,  true,   true
        ];

        assert_eq!(board, result);

        assert_eq!(solution, [4]);
    }
}

#[cfg(test)]
mod args_tests {

    use clap::*;

    use crate::args::*;

    fn get_test_cmd() -> Command {
        init_app()
    }

    macro_rules! test_args {
        ($($arg:expr),*) => {
            {
                let mut arg_vec = vec!["<PROGRAM>"];
                $(arg_vec.push($arg);)*
                arg_vec
            }
        };
    }

    #[test]
    fn test_name() {
        let cmd = get_test_cmd();

        assert_eq!(cmd.get_name(), "Lights Out Puzzle Solver");
        assert!(cmd.try_get_matches_from(test_args!["--version"]).is_err()); // Is error because does not match any arg
    }

    #[test]
    fn test_input_lights() {
        let matches = get_test_cmd().try_get_matches_from(test_args!["7", "9", "1", "3"]).unwrap();

        assert_eq!(matches.get_many::<usize>(ProgramArgs::Lights.id()).unwrap().copied().collect::<Vec<_>>(), [7,9,1,3]);
    }
}
