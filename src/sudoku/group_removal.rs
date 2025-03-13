use std::cmp::{max, min};

use crate::Sudoku;
use crate::cell::{Cell, CELL_ACC};
use crate::index_manip::*;
use crate::history::{HistoryEntry, CellChange, EntryType};

/* Goal of group_removal:
    Look for groups of digits in the sudoku board.
    Currently, the code simply looks for larger naked groups instead
    of hidden groups. The code could be changed to find them,
    and that sounds like a lotta work for more complexity and prolly
    not that much speed increase.
*/

// Def of naked group:
// a set of cells whose amount of unique digits is equal to
// the size of the set.

// Def of hidden group:
// a set of cells where the amount of unique digits in the set that
// satisfy f(x) equals the amount of cells in the set.
//
// let f(x) = true if amount of cells in group that have digit x
//                    == amount of cells in section that have digit x,
//            false otherwise
// 
// Alt def:
// A set of digits whose total size equals the amount of cells in a
// given section that contain those digits.

/* Algorithm:
    for each section:
        Break the section into subsections, where each subsection
        has no overlapping digits with another.
        
        for each subsection large enough to have a group:
            look through all possible combinations of cells to find a
            set of cells that satisfies the definition of a naked group.
            
            if a group is found, use it to remove digits
            if changes were made, return
*/


const MIN_GROUP_SIZE: usize = 2;
const MAX_GROUP_SIZE: usize = 7;

fn max_group_size_of(s: usize) -> usize {
    s - MIN_GROUP_SIZE
}


#[derive(Debug)]
struct Subsection {
    acc: Cell,
    total_cells: usize,
    cand_cells: Vec<Cell>,
    cand_ids: Vec<usize>,
    mgs: usize,
}

impl Subsection {
    fn new() -> Self {
        Self {
            acc: CELL_ACC,
            total_cells: 0,
            cand_cells: Vec::with_capacity(9),
            cand_ids: Vec::with_capacity(9),
            mgs: 0,
        }
    }

    fn add_cell(&mut self, cid: usize, c: Cell) {
        self.acc.union_with(c);

        if usize::from(c.get_count()) < MAX_GROUP_SIZE {
            self.cand_ids.push(cid);
            self.cand_cells.push(c);
        }
        
        self.total_cells += 1;
    }

    fn cell_belongs(&self, c: Cell) -> bool {
        self.acc.has_intersection(c)
    }

    fn calc_mgs(&mut self) {
        self.mgs = min(self.cand_ids.len(),
                       max_group_size_of(self.total_cells));
    }

    fn find_group(&self) -> Option<Vec<usize>> {
        for pos in 0..self.cand_ids.len()-1 {
            if let Some(g) = self.find_group_r(self.cand_cells[pos], 1, pos) {
                return Some(g);
            }
        }

        None
    }

    fn find_group_r(&self, acc: Cell, cell_count: usize,
                        cid: usize) -> Option<Vec<usize>> {
        // Rust's handling of integers is kinda getting on my nerves
        let acc_count = usize::from(acc.get_count());

        let max_count = max(acc_count, cell_count);

        if max_count > self.mgs {
            return None;
        }
        else if acc_count == cell_count {
            let mut output = vec![0; cell_count];

            output[cell_count-1] = self.cand_ids[cid];

            return Some(output);
        }

        for nid in (cid+1)..self.cand_ids.len() {
            if let Some(mut output) = self.find_group_r(
                    acc.union(self.cand_cells[nid]),
                    cell_count+1, nid) {
                output[cell_count-1] = self.cand_ids[cid];
                return Some(output);
            }
        }

        None
    }
}


impl Sudoku {
    pub fn group_removal(&mut self) -> bool {
        for si in SECTION_RANGE {
            let sec_cells = &SECTION_INDICES[si];

            let mut vec_sc = Vec::from(sec_cells);

            let mut i = 0;
            while i < vec_sc.len() {
                if self.cells[vec_sc[i]].is_solved() {
                    vec_sc.swap_remove(i);
                }
                else {
                    i += 1;
                }
            }

            // B/c max_group_size_of uses subtraction, it has a possibility
            // of hitting an error (usize can't be <0), which would be
            // annoying to handle without using multiple if stmts like so.
            if vec_sc.len() <= MIN_GROUP_SIZE {
                continue;
            }

            let section_mgs = max_group_size_of(vec_sc.len());

            if section_mgs < MIN_GROUP_SIZE {
                continue;
            }

            let subsections = self.get_subsections(vec_sc);

            for sb in &subsections {
                if sb.mgs < MIN_GROUP_SIZE {
                    continue;
                }

                if let Some(g) = sb.find_group() {
                    // This if stmt is basically a formality, if the
                    // algorithm finds a group then it is one that
                    // changes the board.
                    if self.handle_group(sec_cells, g) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn get_subsections(&self, mut sec_cells: Vec<usize>) -> Vec<Subsection> {
        let mut sbs = vec![];

        while !sec_cells.is_empty() {
            let mut sb = Subsection::new();
            let cid = sec_cells.swap_remove(0);
            sb.add_cell(cid, self.cells[cid]);

            let mut has_intersection = true;
            while has_intersection {
                has_intersection = false;

                let mut ci = 0;
                while ci < sec_cells.len() {
                    let cell_id = sec_cells[ci];
                    let cell = self.cells[cell_id];

                    if sb.cell_belongs(cell) {
                        sb.add_cell(cell_id, cell);
                        sec_cells.swap_remove(ci);
                        has_intersection = true;
                    }
                    else {
                        ci += 1;
                    }
                }
            }

            sb.calc_mgs();

            sbs.push(sb);
        }

        sbs
    }

    fn handle_group(&mut self, section: &[usize; 9], g: Vec<usize>) -> bool {
        let mut acc = CELL_ACC;

        for cid in &g {
            acc.union_with(self.cells[*cid]);
        }

        let inv_acc = acc.inverse();

        let mut changes: Vec<CellChange> = vec![];

        for sid in *section {
            let cell = self.cells[sid];

            if !cell.is_solved() && cell.has_intersection(acc)
                                 && cell.has_intersection(inv_acc)
                                 && self.cells[sid].remove_digits(acc) {
                changes.push(CellChange {
                    id: sid,
                    new_cell: self.cells[sid]});
            }
        }

        if !changes.is_empty() {
            self.history.push(HistoryEntry::new(
                EntryType::NakedGroup,
                g,
                acc,
                changes));

            true
        }
        else {
            false
        }
    }
}
