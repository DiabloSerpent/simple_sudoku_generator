mod cell;
mod index_manip;
pub mod sudoku;

use crate::sudoku::Sudoku;

pub fn bench_main() {
    let mut sud = Sudoku::new();

    for i in 0..81 {
        if sud.cells[i].is_solved() {
            continue;
        }
        sud.cells[i].generate_number();

        sud.solve();
    }
}
