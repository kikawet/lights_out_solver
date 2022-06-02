use super::board::{BaseBoard, Board};

pub fn solve(board: &dyn Board) -> Option<Vec<usize>> {
    let (cols, rows) = board.size();

    let mut matrix = vec![vec![0usize; rows * cols]; rows * cols];
    let mut blank = BaseBoard::new_blank(cols, rows);
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
            expected[index] = board.get(col, row).unwrap() + 1 % 2;
        }
    }

    let sol = gauss_jordan_zf2(matrix, expected);
    match sol {
        Some(s) => {
            let mut ret = Vec::with_capacity(s.len());
            for i in 0..s.len() {
                if s[i] != 0 {
                    let col = i % cols;
                    let row = i / cols;
                    let index = row * cols + col;
                    ret.push(index);
                }
            }
            Some(ret)
        }
        None => None,
    }
}
fn gauss_jordan_zf2(mat: Vec<Vec<usize>>, expected: Vec<usize>) -> Option<Vec<usize>> {
    let mut m = mat.clone();
    let mut bs = expected.clone();
    let rows = m.len();
    if rows == 0 {
        return None;
    }
    let cols = m[0].len();
    if cols == 0 {
        return None;
    }

    fn swap(m: &mut Vec<Vec<usize>>, bs: &mut Vec<usize>, i: usize, j: usize) {
        if i == j {
            return;
        }
        // XXX is cloning optimal here?
        let tmpi = m.get(i).unwrap().clone();
        let tmpj = m.get(j).unwrap().clone();
        *m.get_mut(i).unwrap() = tmpj;
        *m.get_mut(j).unwrap() = tmpi;

        let tmp = *bs.get(i).unwrap();
        *bs.get_mut(i).unwrap() = *bs.get(j).unwrap();
        *bs.get_mut(j).unwrap() = tmp;
    }

    fn add(m: &mut Vec<Vec<usize>>, bs: &mut Vec<usize>, i: usize, j: usize) {
        if i == j {
            panic!("trying to add row to itself");
        }
        for x in 0..m.get(i).unwrap().len() {
            *m.get_mut(i).unwrap().get_mut(x).unwrap() += *m.get(j).unwrap().get(x).unwrap();
            *m.get_mut(i).unwrap().get_mut(x).unwrap() %= 2;
        }
        *bs.get_mut(i).unwrap() += *bs.get(j).unwrap();
        *bs.get_mut(i).unwrap() %= 2;
    }

    for pivot in 0..rows {
        // 1. find pivot row
        for i in pivot..rows {
            if m[i][pivot] != 0 {
                swap(&mut m, &mut bs, i, pivot);
                break;
            }
        }
        if m[pivot][pivot] == 0 && bs[pivot] != 0 {
            return None;
        }

        // 2. add pivot to all rows that have 1 in this column
        for i in 0..rows {
            if i == pivot {
                continue;
            }
            if m[i][pivot] != 0 {
                add(&mut m, &mut bs, i, pivot);
            }
        }
    }
    Some(bs)
}
