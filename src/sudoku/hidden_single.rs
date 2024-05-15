use crate::Sudoku;
use crate::index_manip::*;
use crate::cell::DIGIT_RANGE;

impl Sudoku {
    pub fn hidden_single(&mut self) -> bool {
        // A row/col/box has only one cell with a particular digit
        // 
        // Struct memory usage: N/A, won't save time using it
        let mut r = false;

        for si in SECTION_RANGE {
            let sums = self.section_digit_sum[si];

            for j in DIGIT_RANGE {
                let count = sums[j as usize];

                if count == 1 {
                    for ci in SECTION_INDICES[si] {
                        if !self.cells[ci].is_solved()
                           && self.cells[ci].has_digit(j) {

                            self.cells[ci].solve_cell(j);

                            r = true;
                        }
                    }
                }
            }
        }

        r
    }
}