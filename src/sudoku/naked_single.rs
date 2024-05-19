use crate::Sudoku;
use crate::cell::DIGIT_RANGE;

impl Sudoku {
    pub fn naked_single(&mut self) -> bool {
        // A cell has only one or 0 digits left
        // 
        // Struct Memory usage: N/A
        let mut r = false;

        for i in 0..81 {
            if !self.cells[i].is_solved() {
                let c = self.cells[i].get_count();

                if c == 1 {
                    let mut digit = 0;

                    for d in DIGIT_RANGE {
                        if self.cells[i].has_digit(d) {
                            digit = d;
                            break;
                        }
                    }

                    self.cells[i].solve_cell(digit);

                    r = true;
                }
                else if c == 0 {
                    self.cells[i].solve_cell(0);

                    r = true;
                }
            }
        }

        r
    }
}