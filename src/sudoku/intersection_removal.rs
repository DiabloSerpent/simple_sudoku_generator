use crate::Sudoku;
use crate::index_manip::*;
use crate::cell::{CELL_ACC, Cell, CELL_EMPTY};
use crate::sudoku::EntryType;

// Definition of pointed group:
// A set of digits within cells that share the same box and row or column,
// where the digits are only present within the set and not the box.
// 
// This means the digits within the same row/column can be eliminated.

// Definition of box-line reduction:
// A set of digits within cells that share the same box and row or column,
// where the digits are only present within the set and not the row/column.
// 
// This means the digits within the same box can be eliminated.


// TODO: Create BoxDivision type?
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

                    let other_box = trio_mat[nx1][y].union(trio_mat[nx2][y]);
                    let other_sec = trio_mat[x][ny1].union(trio_mat[x][ny2]);

                    let ds = trio.intersect(other_box.xor(other_sec));

                    let ds_box = ds.intersect(other_box);
                    let ds_sec = ds.intersect(other_sec);

                    let mut r = false;
                    if ds_box.has_digits() {
                        r = self.handle_intersection(
                                EntryType::BoxLineReduction,
                                bd, ds_box,
                                (x, y), nx1, nx2);
                    }
                    if ds_sec.has_digits() {
                        r = self.handle_intersection(
                                EntryType::PointedGroup,
                                bd, ds_sec,
                                (x, y), ny1, ny2) || r;
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

    fn handle_intersection(&mut self, t:  EntryType,
                                      bd: &[[[usize; 3]; 3]; 3],
                                      ds: Cell,
                                      (x, y): (usize, usize),
                                      id1: usize, id2: usize) -> bool {
        let to_check = match t {
            EntryType::BoxLineReduction => [(id1, y), (id2, y)],
            EntryType::PointedGroup     => [(x, id1), (x, id2)],
            _ => panic!("{t:?} is not a valid value here"),
        };

        for (i, j) in to_check {
            for cid in &bd[i][j] {
                let cell = &mut self.cells[*cid];
                
                if !cell.is_solved() && cell.remove_digits(ds) {
                    self.register_change(*cid);
                }
            }
        }

        let r = self.has_changes();

        if r {
            let mut v = Vec::new();

            for c in bd[x][y] {
                let cell = self.cells[c];
                if !cell.is_solved() && ds.has_intersection(cell) {
                    v.push(c);
                }
            }

            self.add_history_entry(t, v, ds);
        }

        r
    }
}