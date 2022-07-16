use std::fmt::Display;

use crate::sudoku::SudokuSolver;

pub struct IntegerSudoku {
    board: [[u32; 9]; 9],
}

impl IntegerSudoku {
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
    fn new(board: [[u32; 9]; 9]) -> Self {
        IntegerSudoku { board }
    }

    fn solve(&mut self) -> bool {
        self.solve_cell(0, 0)
    }
}

impl Display for IntegerSudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", self.board[i][j])?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
