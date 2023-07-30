use super::board::{Binary, Board};

pub fn solve(board: &dyn Board) -> Option<Vec<usize>> {
    let mut solution: Vec<usize> = vec![];
    let mut best_solution: Option<Vec<usize>> = None;
    let mut available_moves: Vec<bool> = vec![true; board.cols() * board.rows()];

    let mut board = Binary::new_from_values(
        &board.iter().map(|&b| b != 0).collect::<Vec<_>>(),
        board.cols(),
        board.rows(),
    );

    solve_recursive(
        &mut board,
        &mut available_moves,
        &mut solution,
        &mut best_solution,
    );

    best_solution
}

fn is_solution_better(new_solution: &[usize], old_solution: &[usize]) -> bool {
    new_solution.len() < old_solution.len()
}

fn solve_recursive(
    board: &mut dyn Board,
    available_moves: &mut Vec<bool>,
    solution: &mut Vec<usize>,
    best_solution: &mut Option<Vec<usize>>,
) {
    let is_better_solution = match best_solution {
        Some(best) => is_solution_better(solution, best),
        None => false,
    };

    if board.is_solved() || solution.len() > board.iter().len() || is_better_solution {
        return;
    }

    for i in 0..(board.cols() * board.rows()) {
        if !available_moves[i] {
            return;
        }

        available_moves[i] = false;
        board.trigger_index(i);
        solution.push(i);

        solve_recursive(board, available_moves, solution, best_solution);

        if board.is_solved() {
            match best_solution {
                None => {
                    *best_solution = Some(solution.clone());
                }
                Some(best) => {
                    if is_solution_better(solution, best) {
                        *best_solution = Some(solution.clone());
                    }
                }
            };
        }

        available_moves[i] = true;
        solution.pop();
        board.trigger_index(i);
    }
}
