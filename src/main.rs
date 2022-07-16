mod sudoku;

mod binary_sudoku;
mod int_sudoku;

use std::{fmt::Display, time::Instant};

use binary_sudoku::BinarySudoku;
use int_sudoku::IntegerSudoku;

use sudoku::Sudoku;

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

    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    // let board = [[0; 9]; 9];
    for _ in 0..1 {
        benchmark::<IntegerSudoku>(board);
        benchmark::<BinarySudoku>(board);
    }
}

fn benchmark<Board: Sudoku + Display>(board: [[u32; 9]; 9]) {
    let mut board = Board::new(board);

    let start = Instant::now();
    let result = board.solve();
    println! {"{}", board};

    println!("{:?}: {:?}", result, start.elapsed());
}
