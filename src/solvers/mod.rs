pub mod board;
pub mod gf2;
pub mod recursive;

#[cfg(test)]
mod solver_tests {
    use crate::solvers::{
        board::{BaseBoard, Board},
        gf2,
    };

    fn assert_board_eq(board: &dyn Board, expected: &[usize]) {
        assert!(board.iter().zip(expected.iter()).all(|(&a, &b)| a == b));
    }

    #[test]
    fn board_simulate_tl() {
        let mut board = BaseBoard::new_blank(3, 3);
        board.trigger_index(0);

        let expected = [
            1, 1, 0, //
            1, 0, 0, //
            0, 0, 0, //
        ];
        assert_board_eq(&board, &expected);
    }

    #[test]
    fn test_simulate_mm() {
        let mut board = BaseBoard::new_blank(3, 3);
        board.trigger_index(4);

        let expected = [
            0, 1, 0, //
            1, 1, 1, //
            0, 1, 0, //
        ];
        assert_board_eq(&board, &expected);
    }

    #[test]
    fn test_simulate_br_mm() {
        let mut board = BaseBoard::new_blank(3, 3);
        board.trigger_index(4);
        board.trigger_index(8);

        let expected = [
            0, 1, 0, //
            1, 1, 0, //
            0, 0, 1, //
        ];
        assert_board_eq(&board, &expected);
    }

    #[test]
    fn test_simulate_ml_mr() {
        let mut board = BaseBoard::new_blank(3, 3);
        board.trigger_index(3);
        board.trigger_index(5);

        let expected = [
            1, 0, 1, //
            1, 0, 1, //
            1, 0, 1, //
        ];
        assert_board_eq(&board, &expected);
    }

    #[test]
    fn test_is_solved() {
        let mut board = BaseBoard::new_blank(3, 3);

        let solution = gf2::solve(&board).unwrap();

        solution.iter().for_each(|&step| {
            board.trigger_index(step);
        });

        assert!(board.is_solved());
    }

    #[test]
    fn test_minimun_solution() {
        let mut board = BaseBoard::new_from_values(
            &[
                true, false, true, //
                false, false, false, //
                true, false, true, //
            ],
            3,
            3,
        );

        let solution = gf2::solve(&board).unwrap();

        solution.iter().for_each(|&step| {
            board.trigger_index(step);
        });

        assert!(board.is_solved());

        assert_eq!(solution, [4]);
    }
}
