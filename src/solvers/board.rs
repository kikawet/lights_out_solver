// Credit https://github.com/oovm/deus-rs/blob/master/src/solvers/state2.rs

pub trait Board {
    fn size(&self) -> (usize, usize);
    fn cols(&self) -> usize;
    fn rows(&self) -> usize;
    fn is_solved(&self) -> bool;
    fn trigger_coord(&mut self, col: usize, row: usize) -> &mut dyn Board;
    fn trigger_index(&mut self, index: usize) -> &mut dyn Board;
    fn get(&self, col: usize, row: usize) -> Option<usize>;
    fn set(&mut self, col: usize, row: usize, value: usize) -> bool;
    fn iter(&self) -> std::slice::Iter<'_, usize>;
}

#[derive(Debug)]
pub struct Binary {
    cols: usize,
    rows: usize,
    board: Vec<usize>,
}

impl Binary {
    #[must_use]
    pub fn new_blank(cols: usize, rows: usize) -> Binary {
        Binary {
            cols,
            rows,
            board: vec![0usize; cols * rows],
        }
    }

    #[must_use]
    pub fn new_from_positions(active: &[usize], cols: usize, rows: usize) -> Binary {
        let mut board = vec![0usize; cols * rows];

        active.iter().for_each(|position| board[*position] = 1);

        Binary { cols, rows, board }
    }

    #[must_use]
    pub fn new_from_values(active: &[bool], cols: usize, rows: usize) -> Binary {
        let mut board = vec![0usize; cols * rows];

        board
            .iter_mut()
            .zip(active.iter())
            .for_each(|(b, &a)| *b = usize::from(a));

        Binary { cols, rows, board }
    }

    fn get_index(&self, col: usize, row: usize) -> usize {
        row * self.cols + col
    }
}

impl Board for Binary {
    fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.board.iter()
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
        self.board.iter().all(|val| *val == 1)
    }

    fn trigger_index(&mut self, index: usize) -> &mut dyn Board {
        let col = index % self.cols;
        let row = index / self.cols;

        self.trigger_coord(col, row)
    }

    fn trigger_coord(&mut self, col: usize, row: usize) -> &mut dyn Board {
        fn switch(this: &mut Binary, col: usize, row: usize) {
            match this.get(col, row) {
                Some(1) => this.set(col, row, 0),
                Some(0) => this.set(col, row, 1),
                _ => false,
            };
        }
        if col >= self.cols || row >= self.rows {
            return self;
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
