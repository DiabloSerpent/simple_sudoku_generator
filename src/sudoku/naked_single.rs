use crate::Sudoku;
use crate::cell::{DIGIT_RANGE, CELL_EMPTY};
use crate::sudoku::EntryType;

impl Sudoku {
    pub fn naked_single(&mut self) -> bool {
        // A cell has only one or 0 digits left
        // 
        // Struct Memory usage: N/A
        let mut r = false;

        for i in 0..81 {
            if !self.cells[i].is_solved() {
                let c = self.cells[i].get_count();

                if c > 1 {
                    continue;
                }

                let mut digit = 0;

                // if c == 0, then ignore this part
                if c == 1 {
                    for d in DIGIT_RANGE {
                        if self.cells[i].has_digit(d) {
                            digit = d;
                            break;
                        }
                    }
                }

                self.cells[i].solve_cell(digit);

                self.register_change(i);

                self.add_history_entry(
                        EntryType::NakedSingle,
                        vec![i],
                        // This might be an odd choice, but the digit pattern
                        // var is supposed to show which digits have been
                        // removed.
                        // And in this case, there are no such digits.
                        CELL_EMPTY);

                r = true;
            }
        }

        r
    }
}