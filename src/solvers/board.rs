// Credit https://github.com/oovm/deus-rs/blob/master/src/solvers/state2.rs

pub trait Board {
    fn size(&self) -> (usize, usize);
    fn cols(&self) -> usize;
    fn rows(&self) -> usize;
    fn is_solved(&self) -> bool;
    fn trigger_coord<'a>(&'a mut self, col: usize, row: usize) -> &'a mut dyn Board;
    fn trigger_index<'a>(&'a mut self, index: usize) -> &'a mut dyn Board;
    fn get(&self, col: usize, row: usize) -> Option<usize>;
    fn set(&mut self, col: usize, row: usize, value: usize) -> bool;
    fn iter(&self) -> std::slice::Iter<'_, usize>;
}

#[derive(Debug)]
pub struct BaseBoard {
    cols: usize,
    rows: usize,
    board: Vec<usize>,
}

impl BaseBoard {
    pub fn new_blank(cols: usize, rows: usize) -> BaseBoard {
        BaseBoard {
            cols,
            rows,
            board: vec![0usize; cols * rows],
        }
    }

    pub fn new_from(active: &Vec<usize>, cols: usize, rows: usize) -> BaseBoard {
        let mut board = vec![0usize; cols*rows];

        active.iter().for_each(|position| board[*position] = 1);

        BaseBoard {
            cols,
            rows,
            board,
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

    fn cols(&self) -> usize {
        self.cols
    }

    fn rows(&self) -> usize {
        self.rows
    }
    
    fn iter(&self) -> std::slice::Iter<'_, usize>{
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
        self.board.iter().all(|val| *val == 0)
    }

    fn trigger_index<'a>(&'a mut self, index: usize) -> &'a mut dyn Board {
        let col = index % self.cols;
        let row = index / self.cols;

        self.trigger_coord(col, row)
    }

    fn trigger_coord<'a>(&'a mut self, col: usize, row: usize) -> &'a mut dyn Board {
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
