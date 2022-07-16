use std::{
    fmt::Display,
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::sudoku::Sudoku;

pub struct BinarySudoku {
    pub masks: [[u32; 9]; 9],
    pub board: [[u32; 9]; 9],
}

impl BinarySudoku {
    fn assert_cell(&mut self, i: usize, j: usize, value: u32) {
        self.board[i][j] = value + 1;

        let mask = 1 << value;

        for i in 0..9 {
            self.masks[i][j] &= !mask;
        }

        for j in 0..9 {
            self.masks[i][j] &= !mask;
        }

        let row_start = (i / 3) * 3;
        let col_start = (j / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                self.masks[row_start + i][col_start + j] &= !mask;
            }
        }

        self.masks[i][j] = mask;
    }

    fn find_best_cell(&self) -> (usize, usize) {
        let mut best_cell = (0, 0);
        let mut num_options = 9;

        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 && self.masks[i][j].count_ones() < num_options {
                    num_options = self.masks[i][j].count_ones();
                    best_cell = (i, j);
                }
            }
        }

        best_cell
    }

    fn count_remaining(&self) -> u32 {
        let mut count = 0;

        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    count += 1;
                }
            }
        }

        count
    }

    fn solve_cells(&mut self, remaining: u32) -> bool {
        if remaining <= 0 {
            return true;
        }

        let (i, j) = self.find_best_cell();
        let prev_masks = self.masks.clone();
        for value in 0..9 {
            let value_mask = 1 << value;
            if self.masks[i][j] & value_mask != 0 {
                self.assert_cell(i, j, value);

                // println!("\n({}, {}), {}", i, j, value);
                // println!("{}", self);
                // thread::sleep(Duration::from_millis(200));

                if self.solve_cells(remaining - 1) {
                    return true;
                }

                self.masks = prev_masks.clone();
            }
        }

        false
    }
}

impl Sudoku for BinarySudoku {
    fn new(board: [[u32; 9]; 9]) -> Self {
        let mut this = BinarySudoku {
            masks: [[0b111111111; 9]; 9],
            board: [[0; 9]; 9],
        };

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != 0 {
                    this.assert_cell(i, j, board[i][j] - 1)
                }
            }
        }

        this
    }

    fn solve(&mut self) -> bool {
        let remaining = self.count_remaining();
        self.solve_cells(remaining)
    }
}

impl Display for BinarySudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board.clone();

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", board[i][j])?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
