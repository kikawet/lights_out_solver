// Credit https://github.com/oovm/deus-rs/blob/master/src/solvers/state2.rs

pub trait Board {
    fn size(&self) -> (usize, usize);
    fn is_solved(&self) -> bool;
    fn make_move<'a>(&'a mut self, col: usize, row: usize) -> &'a mut dyn Board;
    fn get(&self, col: usize, row: usize) -> Option<usize>;
    fn set(&mut self, col: usize, row: usize, value: usize) -> bool;
}

#[derive(Debug)]
pub struct BaseBoard {
    cols: usize,
    rows: usize,
    board: Vec<usize>,
}

impl BaseBoard {
    pub fn new(cols: usize, rows: usize) -> BaseBoard {
        BaseBoard {
            cols,
            rows,
            board: vec![0usize; cols * rows],
        }
    }

    fn get_index(&self, col: usize, row: usize) -> usize {
        row * self.cols + col
    }
}

impl Board for BaseBoard {
    fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    fn get(&self, col: usize, row: usize) -> Option<usize> {
        if col < self.cols && row < self.rows {
            let index = self.get_index(col, row);
            Some(self.board[index])
        } else {
            None
        }
    }

    fn set(&mut self, col: usize, row: usize, value: usize) -> bool {
        if col < self.cols && row < self.rows {
            match value {
                0..=1 => {
                    let index = self.get_index(col, row);
                    self.board[index] = value;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_solved(&self) -> bool {
        self.board.iter().all(|val| *val == 0)
    }

    fn make_move<'a>(&'a mut self, col: usize, row: usize) -> &'a mut dyn Board {
        if col >= self.cols || row >= self.rows {
            return self;
        }
        fn switch(this: &mut BaseBoard, col: usize, row: usize) {
            match this.get(col, row) {
                Some(1) => this.set(col, row, 0),
                Some(0) => this.set(col, row, 1),
                _ => false,
            };
        }
        if row > 0 {
            switch(self, col, row - 1);
        }
        if col > 0 {
            switch(self, col - 1, row);
        }
        switch(self, col, row);
        switch(self, col + 1, row);
        switch(self, col, row + 1);
        self
    }
}
