use crate::Sudoku;
use crate::cell::{DIGIT_RANGE, CELL_INIT, Cell, DIGIT, CellSize};
use crate::index_manip::*;

impl Sudoku {
    pub fn group_removal(&mut self) -> bool {
        // Identify hidden and naked groups within each section
        // and remove offending digits.

        // Also detects hidden/naked pairs

        // the maximum size of a group should be 4, or floor(9 / 2)

        // function should return if a change is detected, but only
        // when changing the type of section (ie rows to cols, cols to boxes)

        // Def of naked group:
        // a set of cells whose total count of digits is equal to
        // the size of the set.

        // Def of hidden group:
        // a set of cells where the amount of digits in the group that
        // satisfy f(x) equals the amount of cells in the set.
        //
        // let f(x) = true if amount of cells in group that have digit x
        //                    == amount of cells in section that have digit x,
        //            false otherwise

        {/*Alg 3: (unimplemented)
            for n in 2..=4:
                for combo in choose(9, n):
                    // because naked and hidden groups have
                    // separate detection mechanisms, I can
                    // now remove undesired iterations with
                    // greater accuracy.

                    // For naked groups:
                    //   a cell w/ more than 4 digits is not worth considering
                    //   likewise for cell w/ <2 digits

                    // For hidden groups:
                    //   a digit w/ more than 4 cells is not worth considering
                    //   likewise for digit w/ <2 cells

                    // idk how exactly to implement tho.

                    let digit_combo = combo of digits

                    for section in section range:
                        let cell_combo = combo of cells in section

                        for cell in cell_combo:
                            acc |= cell

                        is_naked = acc.count == n

                        if is_naked:
                            for cell in section:
                                if cell isnt in cell_combo and cell has digit:
                                    r = true
                                    cell.remove_digits(acc)

                            if r:
                                return true

                        for cell in section:
                            sum += (digit_combo & cell) == 1

                        is_hidden = sum == n

                        if is_hidden:
                            for cell in section:
                                if cell has intersection w/ digit_combo:
                                    r = true
                                    cell = cell.intersect(digit_combo)

                            if r:
                                return true

            return false
        */}

        let mut r = false;

        //let digit_array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Eventually Important:
        //   How to tell function to ignore combos based on
        //   max group size available in section.

        // Eventually Important:
        //   How to tell function to ignore combos that dont have
        //   a cell/digit that's been modified.

        // Should encompass:
        //   - cell/digit isnt solved
        //   - cell/digit has <4 digits
        //   - cell/digit isnt part of previously discovered group
        //     - how to handle cells belonging to size 4 group?
        //   - has cell been modified?
        //     - should be handled separately

        let mut reject_cell  = [[false; 9]; 27];
        let mut reject_digit = [[false; 9]; 27];

        for si in SECTION_RANGE {
            let sec_cells = &SECTION_INDICES[si];
            let sec_sums  = &self.section_digit_sum[si];

            for i in 0..9 {
                let cell = &self.cells[sec_cells[i]];
                reject_cell[si][i] = cell.is_solved()
                                     || cell.get_count() > 4
                                     || self.section_cell_groups[si][i];

                let digit = i + 1;
                reject_digit[si][i] = sec_sums[digit] <= 1
                                      || sec_sums[digit] > 4
                                      || self.section_digit_groups[si][i];
            }
        }

        for n in 2..=4 {
            let mut combo = Vec::with_capacity(n);
            let mut max   = Vec::with_capacity(n);

            for i in 0..n {
                combo.push(i);
                max.push(9 - 1 - i);
            }

            max.reverse();

            //let mut digit_combo = combo.clone();
            let mut cell_combo  = combo.clone();

            loop { // for combo in choose(9, n)
                let mut hidden_acc = Cell(0);

                for i in 0..n {
                    hidden_acc.0 |= DIGIT((combo[i] + 1) as CellSize);
                }

                for si in SECTION_RANGE {
                    let sec_cells = &SECTION_INDICES[si];
                    //let sec_sums  = &self.section_digit_sum[si];

                    // TODO: continue if n > max group size in section

                    let mut check_naked  = true;
                    let mut check_hidden = true;

                    for i in 0..n {
                        cell_combo[i] = sec_cells[combo[i]];

                        check_naked = check_naked && !reject_cell[si][combo[i]];
                        // TODO: check if cell is part of solved group
                        //       w/ size <4

                        check_hidden = check_hidden && !reject_digit[si][combo[i]];
                    }

                    if check_naked {    
                        let mut naked_acc = Cell(0);
    
                        for ci in &cell_combo {
                            naked_acc.0 |= self.cells[*ci].0;
                        }
    
                        let mut sum = 0;
    
                        for d in DIGIT_RANGE {
                            sum += if naked_acc.has_digit(d) { 1 }
                                   else { 0 };
                        }
    
                        let is_naked = sum == n;
    
                        if is_naked {
                            /*println!("{}: {:?}", of_section(si), cell_combo);
                            println!("{self:?}");*/

                            if n < 4 {
                                for i in 0..9 {
                                    if cell_combo.contains(&sec_cells[i]) {
                                        self.section_cell_groups[si][i] = true;
                                    }

                                    if naked_acc.has_digit(i as CellSize + 1) {
                                        self.section_digit_groups[si][i] = true;
                                    }
                                }
                            }

                            for ci in sec_cells {
                                if !cell_combo.contains(&ci) {
                                    for d in DIGIT_RANGE {
                                        if naked_acc.has_digit(d) && self.cells[*ci].has_digit(d) {
                                            r = true;
                                            self.cells[*ci].remove_digit(d);
                                        }
                                    }
                                }
                            }

                            if r {
                                return true;
                            }
                        }
                    }

                    if check_hidden {
                        let mut sum = 0;

                        for ci in sec_cells {
                            sum += if self.cells[*ci].0 & hidden_acc.0 != 0
                                   { 1 } else { 0 };
                        }

                        let is_hidden = sum == n;

                        if is_hidden {
                            /*println!("{}: {}", of_section(si), hidden_acc);
                            println!("{self:?}");*/

                            if n < 4 {
                                for i in 0..9 {
                                    if self.cells[sec_cells[i]].0 & hidden_acc.0 != 0 {
                                        self.section_cell_groups[si][i] = true;
                                    }

                                    if hidden_acc.has_digit(i as CellSize + 1) {
                                        self.section_digit_groups[si][i] = true;
                                    }
                                }
                            }

                            for ci in sec_cells {
                                if self.cells[*ci].0 & hidden_acc.0 != 0 {
                                    for d in DIGIT_RANGE {
                                        if !hidden_acc.has_digit(d) && self.cells[*ci].has_digit(d) {
                                            r = true;
                                            self.cells[*ci].remove_digit(d);
                                        }
                                    }
                                }
                            }

                            if r {
                                /*println!("{}: {}", of_section(si), hidden_acc);
                                println!("{self:?}");*/
                                return true;
                            }
                        }
                    }
                }

                // #########################################
                // NEXT COMBO
                // #########################################

                let mut i = n;

                while i > 0 && combo[i - 1] == max[i - 1] {
                    i -= 1;
                }

                if i == 0 {
                    break;
                }

                combo[i - 1] += 1;

                if i == n {
                    continue;
                }

                while i < n {
                    combo[i] = combo[i - 1] + 1;
                    i += 1;
                }
            }
        }

        false
    }
}
