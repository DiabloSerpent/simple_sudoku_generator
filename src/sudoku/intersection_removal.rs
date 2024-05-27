use crate::Sudoku;
use crate::index_manip::*;
use crate::cell::DIGIT_RANGE;

impl Sudoku {
    pub fn intersection_removal(&mut self) -> bool {
        // Remove digits that are outside of an intersection.

        // this covers both intersection removal and box-line removal
        // on the sudoku wiki website.


        let mut r = false;

        /* Algorithm:

            search by row, then col:
                create array of 9 vectors, corresponding to each digit

                continue if unsolved count in section < 2

                search by cell in section:
                    continue if cell is solved

                    search by digit in cell:
                        if count of digit in this section <= 3
                            && count of digit in box_of(cell) > 2:
                                array[digit].push(cell)

                for vector in array:
                    let d = current digit

                    // if vector isnt empty, it should have >1 element
                    if vector is empty:
                        continue

                    let box = box_of(vector[0])
                    if vector[1..].iter().position(|&x| box_of(x) != box) == None {
                        continue;
                    }

                    for cell in of_box(box):
                        if cell has digit:
                            r = true;
                            cell.remove_digit(d)


                //if the only 2 or 3 cells that hold a particular digit are in the same box,
                //then clear out that digit from the rest of the cells in that box

            search by box:
                if the only 2 or 3 cells that hold a particular digit are in the same row/col,
                then clear out that digit from the rest of the cells in that row/col
        */

        for si in 0..18 {
            // Only check section if there are at least 2 unsolved cells
            if self.section_digit_sum[si][0] < 2 {
                continue;
            }

            // This stuff could also prolly be used in other rules.

            // Very dumb imo. prolly can be done better.
            let mut profile = Vec::with_capacity(9);
            for _ in DIGIT_RANGE {
                profile.push(Some(Vec::with_capacity(3)));
            }

            for ci in SECTION_INDICES[si] {
                if self.cells[ci].is_solved() {
                    continue;
                }

                let b = box_of(ci);
                let bsi = section_of(BoxSection(b));

                for di in DIGIT_RANGE {
                    let di = di as usize;

                    if self.section_digit_sum[si][di] > 1
                       && self.section_digit_sum[si][di] <= 3
                       && self.section_digit_sum[bsi][di] > 2 {

                        if let Some(v) = &mut profile[di - 1] {
                            if !v.is_empty() && box_of(v[v.len() - 1]) != b {
                                profile[di - 1] = None;
                            }
                            else {
                                v.push(ci);
                            }
                        }
                    }
                }
            }

            for di in DIGIT_RANGE {
                if let Some(v) = &mut profile[di as usize - 1] {
                    if v.is_empty() {
                        continue;
                    }

                    let b = box_of(v[0]);

                    for ci in of_box(b) {
                        if !v.contains(&ci) && self.cells[ci].has_digit(di) {
                            println!("Removing {}, {}, digit {di} from {}", RowSection(row_of(ci)), ColSection(col_of(ci)), BoxSection(b));
                            println!("{self:?}\n");
                            self.cells[ci].remove_digit(di);
                            r = true;
                        }
                    }
                }
            }

            if r {
                return true;
            }
        }

        for si in 18..27 {
            if self.section_digit_sum[si][0] < 2 {
                continue;
            }

            let mut row_profile = Vec::with_capacity(9);
            let mut col_profile = Vec::with_capacity(9);
            for _ in DIGIT_RANGE {
                row_profile.push(Some(Vec::with_capacity(3)));
                col_profile.push(Some(Vec::with_capacity(3)));
            }

            for ci in SECTION_INDICES[si] {
                if self.cells[ci].is_solved() {
                    continue;
                }

                let ro = row_of(ci);
                let rsi = section_of(RowSection(ro));
                let c = col_of(ci);
                let csi = section_of(ColSection(c));

                for di in DIGIT_RANGE {
                    let di = di as usize;

                    if self.section_digit_sum[si][di] > 1
                       && self.section_digit_sum[si][di] <= 3 {

                        if self.section_digit_sum[rsi][di] > 2 {
                            if let Some(v) = &mut row_profile[di - 1] {
                                if !v.is_empty()
                                   && row_of(v[v.len() - 1]) != ro {

                                    row_profile[di - 1] = None;
                                }
                                else {
                                    v.push(ci);
                                }
                            }
                        }

                        if self.section_digit_sum[csi][di] > 2 {
                            if let Some(v) = &mut col_profile[di - 1] {
                                if !v.is_empty()
                                   && col_of(v[v.len() - 1]) != c {

                                    col_profile[di - 1] = None;
                                }
                                else {
                                    v.push(ci);
                                }
                            }
                        }
                    }
                }
            }

            for di in DIGIT_RANGE {
                if let Some(v) = &mut row_profile[di as usize - 1] {
                    if v.len() <= 1 {
                        continue;
                    }

                    let ro = row_of(v[0]);

                    for ci in of_row(ro) {
                        if !v.contains(&ci) && self.cells[ci].has_digit(di) {
                            println!("Removing {}, {}, digit {di} from {}", RowSection(ro), ColSection(col_of(ci)), RowSection(ro));
                            println!("{self:?}\n");
                            self.cells[ci].remove_digit(di);
                            r = true;
                        }
                    }
                }

                if let Some(v) = &mut col_profile[di as usize - 1] {
                    if v.len() <= 1 {
                        continue;
                    }

                    let c = col_of(v[0]);

                    for ci in of_col(c) {
                        if !v.contains(&ci) && self.cells[ci].has_digit(di) {
                            println!("Removing {}, {}, digit {di} from {}", RowSection(row_of(ci)), ColSection(c), ColSection(c));
                            println!("{self:?}\n");
                            self.cells[ci].remove_digit(di);
                            r = true;
                        }
                    }
                }
            }

            if r {
                return true;
            }
        }

        r // Should always be false
    }
}