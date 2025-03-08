pub mod cell;
mod index_manip;
pub mod history;
pub mod sudoku;

use crate::sudoku::Sudoku;

pub fn bench_main() {
    let _sud = Sudoku::fill_incremental();
}
