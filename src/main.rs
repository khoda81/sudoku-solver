mod sudoku;

mod binary_sudoku;
mod int_sudoku;

use std::{
    fmt::Display,
    time::{self, Duration, Instant},
};

use binary_sudoku::BinarySudoku;
use int_sudoku::IntegerSudoku;

use sudoku::SudokuSolver;

fn main() {
    // sudoku board

    // let board = [
    //     [0, 0, 3, 0, 2, 0, 6, 0, 0],
    //     [9, 0, 0, 3, 0, 5, 0, 0, 1],
    //     [0, 0, 1, 8, 0, 6, 4, 0, 0],
    //     [0, 0, 8, 1, 0, 2, 9, 0, 0],
    //     [7, 0, 0, 0, 0, 0, 0, 0, 8],
    //     [0, 0, 6, 7, 0, 8, 2, 0, 0],
    //     [0, 0, 2, 6, 0, 9, 5, 0, 0],
    //     [8, 0, 0, 2, 0, 3, 0, 0, 9],
    //     [0, 0, 5, 0, 1, 0, 3, 0, 0],
    // ];

    // let board = [
    //     [1, 0, 0, 0, 0, 7, 0, 9, 0],
    //     [0, 3, 0, 0, 2, 0, 0, 0, 8],
    //     [0, 0, 9, 6, 0, 0, 5, 0, 0],
    //     [0, 0, 5, 3, 0, 0, 9, 0, 0],
    //     [0, 1, 0, 0, 8, 0, 0, 0, 2],
    //     [6, 0, 0, 0, 0, 4, 0, 0, 0],
    //     [3, 0, 0, 0, 0, 0, 0, 1, 0],
    //     [0, 4, 0, 0, 0, 0, 0, 0, 7],
    //     [0, 0, 7, 0, 0, 0, 3, 0, 0],
    // ];

    // let board = [
    //     [1, 6, 8, 5, 0, 7, 2, 9, 3],
    //     [5, 3, 4, 1, 2, 9, 6, 7, 8],
    //     [7, 2, 9, 6, 3, 8, 5, 4, 1],
    //     [4, 7, 5, 3, 1, 2, 9, 0, 0],
    //     [0, 1, 3, 9, 8, 6, 7, 0, 0],
    //     [6, 0, 2, 7, 5, 4, 1, 0, 0],
    //     [3, 0, 6, 8, 7, 5, 4, 1, 9],
    //     [9, 4, 1, 2, 6, 3, 8, 5, 7],
    //     [8, 5, 7, 4, 9, 1, 3, 2, 6],
    // ];

    // let board = [
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 1, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    // ];

    let board = [[0; 9]; 9];

    let mut total_time = Duration::ZERO;
    let n = 100000;
    for _ in 0..n {
        // total_time += benchmark::<IntegerSudoku>(board);
        total_time += benchmark::<BinarySudoku>(board);
    }

    println!("{:?}", total_time / n);
}

fn benchmark<Board: SudokuSolver + Display>(board: [[u32; 9]; 9]) -> time::Duration {
    let mut board = Board::new(board);

    let start = Instant::now();
    board.solve();

    let elapsed = start.elapsed();
    elapsed
}
