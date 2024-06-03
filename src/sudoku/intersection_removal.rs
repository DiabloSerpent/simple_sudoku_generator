use crate::Sudoku;
use crate::index_manip::*;
use crate::cell::DIGIT_RANGE;

impl Sudoku {
    pub fn intersection_removal(&mut self) -> bool {
        // Remove digits that are outside of an intersection.

        // this covers both intersection removal and box-line removal
        // on the sudoku wiki website.


        let mut r = false;

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

                for d in DIGIT_RANGE {
                    let di = d as usize;

                    if !self.cells[ci].has_digit(d) {
                        continue;
                    }

                    if self.section_digit_sum[si][di] > 1
                       && self.section_digit_sum[si][di] <= 3 {

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

                    // TODO: convert of_box to SECTION_INDICES
                    for ci in of_box(b) {
                        if !v.contains(&ci) && self.cells[ci].has_digit(di) {
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
                let c  = col_of(ci);

                for d in DIGIT_RANGE {
                    let di = d as usize;

                    if !self.cells[ci].has_digit(d) {
                        continue;
                    }

                    if self.section_digit_sum[si][di] > 1
                       && self.section_digit_sum[si][di] <= 3 {

                        if let Some(v) = &mut row_profile[di - 1] {
                            if !v.is_empty()
                               && row_of(v[v.len() - 1]) != ro {

                                row_profile[di - 1] = None;
                            }
                            else {
                                v.push(ci);
                            }
                        }

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

            for di in DIGIT_RANGE {
                if let Some(v) = &mut row_profile[di as usize - 1] {
                    if v.is_empty() {
                        continue;
                    }

                    let ro = row_of(v[0]);

                    for ci in of_row(ro) {
                        if !v.contains(&ci) && self.cells[ci].has_digit(di) {
                            self.cells[ci].remove_digit(di);
                            r = true;
                        }
                    }
                }

                if let Some(v) = &mut col_profile[di as usize - 1] {
                    if v.is_empty() {
                        continue;
                    }

                    let c = col_of(v[0]);

                    for ci in of_col(c) {
                        if !v.contains(&ci) && self.cells[ci].has_digit(di) {
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