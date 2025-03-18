use crate::Sudoku;
use crate::index_manip::*;
use crate::cell::DIGIT_RANGE;
use crate::history::EntryType;

impl Sudoku {
    pub fn hidden_single(&mut self) -> bool {
        // A row/col/box has only one cell with a particular digit
        // 
        // Struct memory usage: N/A, won't save time using it
        let mut r = false;

        for si in SECTION_RANGE {
            let sums = self.section_digit_sum[si];

            for j in DIGIT_RANGE {
                let count = sums[j];

                if count == 1 {
                    // It seems like an if stmt is more efficient than the
                    // r = stmt || r; thingy I was trying to do.
                    if self.find_hidden_single(si, j) {
                        r = true;
                    }
                }
            }
        }

        r
    }

    fn find_hidden_single(&mut self, si: usize, digit: usize) -> bool {
        for ci in SECTION_INDICES[si] {
            if !self.cells[ci].is_solved()
                    && self.cells[ci].has_digit(digit) {

                self.cells[ci].solve_cell(digit);

                self.add_history_entry_from_solution(
                            EntryType::HiddenSingle, ci);

                return true;
            }
        }

        false
    }
}