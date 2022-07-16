use std::{fmt::Display, io::Write, thread, time::Duration};

use crate::sudoku::SudokuSolver;

pub struct BinarySudoku {
    pub masks: [[u32; 9]; 9],
    pub board: [[u32; 9]; 9],
}

impl BinarySudoku {
    fn assert_cell(&mut self, i: usize, j: usize, value: u32) {
        self.board[i][j] = value + 1;

        let mask = !(1 << value);
        if self.masks[i][j] & !mask == 0 {
            panic!("Cannot insert {} at ({}, {})", value + 1, i, j);
        }

        for x in 0..9 {
            self.masks[i][x] &= mask;
            self.masks[x][j] &= mask;
        }

        let row_start = (i / 3) * 3;
        let col_start = (j / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                self.masks[row_start + i][col_start + j] &= mask;
            }
        }

        self.masks[i][j] &= !mask;
    }

    fn find_best_cell(&self) -> Option<(usize, usize)> {
        let mut best_cell = None;
        let mut best_num_options = 10;

        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    // if this cell is not solved
                    let num_options = self.masks[i][j].count_ones();
                    if num_options < best_num_options {
                        // if we have less options
                        best_num_options = num_options;
                        best_cell = Some((i, j));
                    }
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
}

impl SudokuSolver for BinarySudoku {
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
        let (i, j) = match self.find_best_cell() {
            None => return true,
            Some(key) => key,
        };

        let prev_masks = self.masks.clone();
        for value in 0..9 {
            let value_mask = 1 << value;
            if self.masks[i][j] & value_mask != 0 {
                self.assert_cell(i, j, value);

                println!("{}", self);
                thread::sleep(Duration::from_millis(50));

                if self.solve() {
                    return true;
                }

                self.masks = prev_masks.clone();
            }
        }

        self.board[i][j] = 0;
        false
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
