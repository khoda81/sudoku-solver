use std::fmt::Display;

use crate::sudoku::SudokuSolver;

pub struct IntegerSudoku {
    board: [[u32; 9]; 9],
}

impl IntegerSudoku {
    /// Solves the Sudoku board using backtracking
    ///
    /// # Arguments
    ///
    /// * `row` - the row index of the cell to solve
    /// * `col` - the column index of the cell to solve
    ///
    /// # Example
    ///
    /// ```
    /// let mut board = IntegerSudoku::new([[0; 9]; 9]);
    /// board.solve_cell(0, 0);
    /// ```
    pub fn solve_cell(&mut self, row: usize, col: usize) -> bool {
        if row == 9 {
            return true;
        }
        if col == 9 {
            return self.solve_cell(row + 1, 0);
        }
        if self.board[row][col] != 0 {
            return self.solve_cell(row, col + 1);
        }
        for i in 1..10 {
            if self.is_valid(row, col, i) {
                self.board[row][col] = i;
                if self.solve_cell(row, col + 1) {
                    return true;
                }
            }
        }
        self.board[row][col] = 0;
        false
    }

    /// Returns true if the number is valid for the given row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row of the board.
    /// * `col` - The column of the board.
    /// * `num` - The number to check.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new();
    /// assert!(board.is_valid(0, 0, 1));
    /// ```
    pub fn is_valid(&self, row: usize, col: usize, num: u32) -> bool {
        for i in 0..9 {
            if self.board[row][i] == num {
                return false;
            }
        }
        for i in 0..9 {
            if self.board[i][col] == num {
                return false;
            }
        }
        let row_start = (row / 3) * 3;
        let col_start = (col / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[row_start + i][col_start + j] == num {
                    return false;
                }
            }
        }
        true
    }
}

impl SudokuSolver for IntegerSudoku {
    /// Create a new IntegerSudoku from a 9x9 array of u32s.
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku::IntegerSudoku;
    ///
    /// let board = [[0; 9]; 9];
    /// let sudoku = IntegerSudoku::new(board);
    /// ```
    fn new(board: [[u32; 9]; 9]) -> Self {
        IntegerSudoku { board }
    }

    /// Solve the sudoku puzzle.
    ///
    /// # Returns
    ///
    /// `true` if the puzzle is solved, `false` otherwise.
    fn solve(&mut self) -> bool {
        self.solve_cell(0, 0)
    }
}

impl Display for IntegerSudoku {
    /// Prints the Sudoku board to the given formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// let mut sudoku = Sudoku::new();
    /// sudoku.board[0][0] = 1;
    ///
    /// println!("{}", sudoku);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", self.board[i][j])?;
                }

                if j % 3 == 2 {
                    write!(f, " ")?;
                } else {
                    write!(f, "|")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
