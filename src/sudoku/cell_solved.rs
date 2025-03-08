use crate::Sudoku;
use crate::sudoku::CellSize;
use crate::index_manip::*;


// I think it would be fairly simple to consolidate this
// and the naked_single function. It would just require
// that "solving" a cell converts it into a naked single,
// and then that is processed by the naked_single
// function to set the solved bit and remove the digit
// from all related cells.
impl Sudoku {
    pub fn cell_solved(&mut self) -> bool {
        // A solved cell should remove the solution from
        // the related digits
        // 
        // Struct memory usage: 81 bools
        for i in 0..81 {
            if !self.solved_cell_checked[i] && self.cells[i].is_solved() {

                self.solved_cell_checked[i] = true;

                let to_remove = self.cells[i].get_number();

                if to_remove == 0 {
                    continue;
                }

                let (irow, icol, ibox) = (
                    of_row(row_of(i)),
                    of_col(col_of(i)),
                    of_box(box_of(i))
                );

                // Cell::remove_digit will check if the cell is solved;
                // the newly solved cell won't be zeroed by this.
                for j in 0..9 {
                    self.cells[irow[j]].remove_digit(to_remove);
                    self.cells[icol[j]].remove_digit(to_remove);
                    self.cells[ibox[j]].remove_digit(to_remove);
                }
            }
        }

        // This rule doesn't solve any cells,
        // so it will never need to loop back
        // to itself. (assuming it always goes first!)
        false
    }
}