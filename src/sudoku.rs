pub trait SudokuSolver {
    fn new(board: [[u32; 9]; 9]) -> Self;
    fn solve(&mut self) -> bool;
}
