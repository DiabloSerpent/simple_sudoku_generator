use crate::Sudoku;
use crate::index_manip::*;
use crate::cell::{DIGIT_RANGE, CELL_ACC, Cell, CELL_EMPTY};

const INDEX_MATRIX: [[[[usize; 3]; 3]; 3]; 6] = make_id_matrix();

const fn make_id_matrix() -> [[[[usize; 3]; 3]; 3]; 6] {
    let mut mat = [[[[0; 3]; 3]; 3]; 6];

    let mut i = 0;
    while i < 3 {
        mat[i]   = get_box_ids(i, SECTION_ROW_START);
        mat[i+3] = get_box_ids(i, SECTION_COL_START);
        i += 1;
    }

    mat
}

const fn get_box_ids(i: usize, offset: usize) -> [[[usize; 3]; 3]; 3] {
    let mut mat = [[[0; 3]; 3]; 3];

    let mut s = 0; while s < 3 {
        let mut os = 0; while os < 3 {
            let mut c = 0; while c < 3 {
                mat[s][os][c] = SECTION_INDICES[offset+i*3+s][os*3+c];
                c += 1;
            }
            os += 1;
        } 
        s += 1;
    }

    mat
}


impl Sudoku {
    pub fn intersection_removal(&mut self) -> bool {
        for bd in &INDEX_MATRIX {
            let trio_mat = self.get_trio_mat(bd);

            for x in 0..3 {
                for y in 0..3 {
                    let trio = trio_mat[x][y];

                    let (nx1, nx2) = ((x+1)%3, (x+2)%3);
                    let (ny1, ny2) = ((y+1)%3, (y+2)%3);

                    let other_sec = trio_mat[nx1][y].union(trio_mat[nx2][y]);
                    let other_box = trio_mat[x][ny1].union(trio_mat[x][ny2]);

                    let ds = trio.intersect(other_sec.xor(other_box));

                    let mut r = false;
                    if ds.has_intersection(other_sec) {
                        r = self.handle_intersection(bd, ds, (nx1, y), (nx2, y));
                    }
                    else if ds.has_intersection(other_box) {
                        r = self.handle_intersection(bd, ds, (x, ny1), (x, ny2));
                    }

                    // Early return b/c its prolly quicker overall and
                    // dealing with cells that may have been updated is
                    // a hassle.
                    if r {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn get_trio_mat(&self, bd: &[[[usize; 3]; 3]; 3]) -> [[Cell; 3]; 3] {
        let mut trio_mat = [[CELL_EMPTY; 3]; 3];

        for x in 0..3 {
            for y in 0..3 {
                let mut c = CELL_ACC;

                for ci in 0..3 {
                    let cell = self.cells[bd[x][y][ci]];
                    if !cell.is_solved() {
                        c.union_with(cell);
                    }
                }

                trio_mat[x][y] = c;
            }
        }

        trio_mat
    }

    fn handle_intersection(&mut self, bd: &[[[usize; 3]; 3]; 3],
                                    ds: Cell, id1: (usize, usize),
                                    id2: (usize, usize)) -> bool {
        let mut r = false;

        // TODO: DESTROY code repetition
        for cid in &bd[id1.0][id1.1] {
            let cell = &mut self.cells[*cid];
            
            if !cell.is_solved() {
                r = cell.remove_digits(ds) || r;
            }
        }
        for cid in &bd[id2.0][id2.1] {
            let cell = &mut self.cells[*cid];
            
            if !cell.is_solved() {
                r = cell.remove_digits(ds) || r;
            }
        }

        r
    }

    pub fn intersection_removal_old(&mut self) -> bool {
        // Remove digits that are outside of an intersection.

        // this covers both intersection removal and box-line removal
        // on the sudoku wiki website.


        let mut r = false;

        for si in SECTION_ROW_START..SECTION_COL_END {
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

        for si in SECTION_BOX_START..SECTION_BOX_END {
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