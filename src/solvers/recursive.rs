pub fn simulate(board: &mut Vec<bool>, position: usize) {
    let side_size: usize = f32::sqrt(board.len() as f32) as usize;
    let mod_position: usize = position % side_size;

    board[position] = !board[position];

    let bottom = position + side_size;
    if bottom < board.len() {
        board[bottom] = !board[bottom]
    }

    if position >= side_size {
        let top = position - side_size;
        board[top] = !board[top];
    }

    let right = position + 1;
    if right % side_size > mod_position {
        board[right] = !board[right];
    }

    if position == 0 {
        return;
    }

    let left = position - 1;
    if left % side_size < mod_position {
        board[left] = !board[left];
    }
}

fn is_solved(board: &[bool]) -> bool {
    board.iter().all(|&x| x)
}

fn is_solution_better(new_solution: &[usize], old_solution: &[usize]) -> bool {
    new_solution.len() < old_solution.len()
}

pub fn solve_recursive(
    board: &mut Vec<bool>,
    available_moves: &mut Vec<bool>,
    solution: &mut Vec<usize>,
    best_solution: &mut Option<Vec<usize>>,
) {
    let is_better_solution = match best_solution {
        Some(best) => is_solution_better(solution, best),
        None => false,
    };

    if is_solved(board) || solution.len() > board.len() || is_better_solution {
        return;
    }

    for i in 0..board.len() {
        if !available_moves[i] {
            continue;
        }

        available_moves[i] = false;
        simulate(board, i);
        solution.push(i);

        solve_recursive(board, available_moves, solution, best_solution);

        if is_solved(board) {
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
        simulate(board, i);
    }
}

pub fn solve(board: &Vec<bool>) -> Option<Vec<usize>> {
    let mut solution: Vec<usize> = vec![];
    let mut best_solution: Option<Vec<usize>> = None;
    let mut available_moves: Vec<bool> = vec![true; board.len()];

    solve_recursive(
        &mut board.clone(),
        &mut available_moves,
        &mut solution,
        &mut best_solution,
    );

    best_solution
}
