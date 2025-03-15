use crate::Sudoku;
use crate::cell::CELL_ACC;
use crate::history::EntryType;
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
                    of_box(box_of(i)),
                );

                // Cell::remove_digit will check if the cell is solved;
                // the newly solved cell won't be zeroed by this.
                for j in 0..9 {
                    for c in [irow[j], icol[j], ibox[j]] {
                        if self.cells[c].remove_digit(to_remove) {
                            self.register_change(c);
                        }
                    }
                }

                let mut digit = CELL_ACC;
                digit.add_digit(to_remove);

                self.add_history_entry_if_changes(
                        EntryType::CellSolved, vec![i], digit);
            }
        }

        // This rule doesn't solve any cells,
        // so it will never need to loop back
        // to itself. (assuming it always goes first!)
        false
    }
}