use std::{fmt::Display, io::Write};

use crate::sudoku::SudokuSolver;

pub struct BinarySudoku {
    masks: [[u32; 9]; 9],
    board: [[u32; 9]; 9],
}

impl BinarySudoku {
    /// Asserts that the cell at (i, j) has the given value.
    ///
    /// This function will panic if the cell already has a different value, or
    /// if the cell cannot have the given value.
    fn assert_cell(&mut self, i: usize, j: usize, value: u32) {
        self.board[i][j] = value + 1;

        let mask = !(1 << value);
        // if self.masks[i][j] & !mask == 0 {
        //     panic!("Cannot insert {} at ({}, {})", value + 1, i, j);
        // }

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

    /// Finds the cell with the least number of options.
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// let mut sudoku = Sudoku::new();
    /// sudoku.board[0][0] = 1;
    /// sudoku.board[0][1] = 2;
    /// sudoku.board[0][2] = 3;
    /// sudoku.board[0][3] = 4;
    /// sudoku.board[0][4] = 5;
    /// sudoku.board[0][5] = 6;
    /// sudoku.board[0][6] = 7;
    /// sudoku.board[0][7] = 8;
    /// sudoku.board[0][8] = 9;
    ///
    /// assert_eq!(sudoku.find_best_cell(), Some((1, 0)));
    /// ```
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
}

impl SudokuSolver for BinarySudoku {
    /// Create a new BinarySudoku from a 9x9 array of u32s.
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku::BinarySudoku;
    ///
    /// let board = [
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    /// ];
    ///
    /// let sudoku = BinarySudoku::new(board);
    /// ```
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

    /// Solves the Sudoku board.
    ///
    /// Returns true if the board is solved, false otherwise.
    fn solve(&mut self) -> bool {
        let (i, j) = match self.find_best_cell() {
            None => return true,
            Some(key) => key,
        };

        let prev_masks = self.masks.clone();
        let num_options = self.masks[i][j].count_ones();
        for value in 0..9 {
            let value_mask = 1 << value;
            if self.masks[i][j] & value_mask != 0 {
                self.assert_cell(i, j, value);

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
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", board[i][j])?;
                }

                if j == 2 || j == 5 {
                    write!(f, "|")?;
                } else {
                    write!(f, " ")?;
                }
            }

            writeln!(f)?;
            if i == 2 || i == 5 {
                writeln!(f, "-----------------")?;
            }
        }

        Ok(())
    }
}
