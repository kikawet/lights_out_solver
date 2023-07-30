use super::board::{Binary, Board};

pub fn solve(board: &dyn Board) -> Option<Vec<usize>> {
    let (cols, rows) = board.size();

    let mut matrix = vec![vec![0usize; rows * cols]; rows * cols];
    let mut blank = Binary::new_blank(cols, rows);
    let mut expected = vec![0usize; cols * rows];

    for row in 0..rows {
        for col in 0..cols {
            let index = row * cols + col;
            blank.trigger_coord(col, row);
            for sub_row in 0..rows {
                for sub_col in 0..cols {
                    let sub_index = sub_row * cols + sub_col;
                    matrix[index][sub_index] = blank.get(sub_col, sub_row).unwrap();
                }
            }
            blank.trigger_coord(col, row);
            expected[index] = (board.get(col, row).unwrap() + 1) % 2;
        }
    }

    let sol = gauss_jordan_zf2(matrix, expected);

    sol.map(|solution| {
        solution
            .iter()
            .enumerate()
            .filter(|(_, val)| **val != 0)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>()
    })
}
fn gauss_jordan_zf2(mut mat: Vec<Vec<usize>>, expected: Vec<usize>) -> Option<Vec<usize>> {
    fn swap(m: &mut [Vec<usize>], sol: &mut [usize], i: usize, j: usize) {
        if i == j {
            return;
        }

        m.swap(i, j);
        sol.swap(i, j);
    }

    fn add(m: &mut [Vec<usize>], bs: &mut [usize], i: usize, j: usize) {
        assert!(i != j, "trying to add row to itself");

        for x in 0..m[i].len() {
            m[i][x] += m[j][x];
            m[i][x] %= 2;
        }

        bs[i] += bs[j];
        bs[i] %= 2;
    }

    let mut solution = expected;
    let rows = mat.len();
    if rows == 0 {
        return None;
    }
    let cols = mat[0].len();
    if cols == 0 {
        return None;
    }

    for pivot in 0..rows {
        // 1. find pivot row
        for i in pivot..rows {
            if mat[i][pivot] != 0 {
                swap(&mut mat, &mut solution, i, pivot);
                break;
            }
        }
        if mat[pivot][pivot] == 0 && solution[pivot] != 0 {
            return None;
        }

        // 2. add pivot to all rows that have 1 in this column
        for i in 0..rows {
            if i == pivot {
                continue;
            }
            if mat[i][pivot] != 0 {
                add(&mut mat, &mut solution, i, pivot);
            }
        }
    }
    Some(solution)
}
