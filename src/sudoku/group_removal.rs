use std::cmp::{max, min};

use crate::Sudoku;
use crate::cell::{Cell, CELL_EMPTY, CELL_ACC, CellSize};
use crate::index_manip::*;
use crate::history::CellChange;


const MIN_GROUP_SIZE: usize = 2;
const MAX_GROUP_SIZE: usize = 7;

fn max_group_size_of(s: usize) -> usize {
    s - 2
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
        // Identify hidden and naked groups within each section
        // and remove offending digits.

        // Also detects hidden/naked pairs

        // the maximum size of a group should be 4, or floor(9 / 2)

        // function should return immediately after solving a group
        // if a change is detected

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

        {/* Algorithm:
            // 2 is the min group size, 4 is the max
            for n in 2..=4:
                // 9 is the max amt of cells/digits in a section
                for combo in choose(9, n):
                    let digit_combo = combo of digits

                    for section in section range:
                        let cell_combo = combo of cells in section

                        let acc = a cell with all unique digits in cell_combo

                        if acc.count == n:
                            for cell in section:
                                if cell has digits in both acc and !acc:
                                    cell.remove_digits(acc)

                            if changes were made:
                                return true

                        let sum = amount of cells in section that
                                    intersect with digit_combo

                        if sum == n:
                            for cell in section:
                                if cell has digits in both digit_combo
                                        and !digit_combo:
                                    cell = cell.intersect(digit_combo)

                            if changes were made:
                                return true

            return false
        */}

        let mut r = false;

        // Eventually Important:
        //   How to tell function to ignore combos based on
        //   max group size available in section.

        // Eventually Important:
        //   How to tell function to ignore combos that dont have
        //   a cell/digit that's been modified.

        // Should encompass:
        //   - cell/digit isnt solved
        //   - cell/digit has <4 digits/cells
        //   - cell/digit isnt part of previously discovered group
        //     - how to handle cells belonging to size 4 group?
        //         - don't
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

        /*
        The primary reason for putting the combination logic in the
        outer loop is to make sure the next combo logic isn't called
        more than strictly necessary. I don't think it saves that much
        runtime overall, but it saves some.
        */

        // TODO: rename n to groupsize
        for n in 2..=4 {
            let mut combo = Vec::with_capacity(n);
            let mut max   = Vec::with_capacity(n);

            for i in 0..n {
                combo.push(i);
                max.push(9 - 1 - i);
            }

            max.reverse();

            let mut cell_combo  = combo.clone();

            loop { // for combo in choose(9, n)
                let mut hidden_acc = CELL_EMPTY;

                for i in 0..n {
                    hidden_acc.add_digit(combo[i] as CellSize + 1);
                }

                for si in SECTION_RANGE {
                    let sec_cells = &SECTION_INDICES[si];

                    // TODO: continue if n > max group size in section

                    let mut check_naked  = true;
                    let mut check_hidden = true;

                    for i in 0..n {
                        cell_combo[i] = sec_cells[combo[i]];

                        check_naked = check_naked && !reject_cell[si][combo[i]];

                        check_hidden = check_hidden && !reject_digit[si][combo[i]];
                    }

                    if check_naked {
                        let mut naked_acc = CELL_ACC;
    
                        for ci in &cell_combo {
                            naked_acc.union_with(self.cells[*ci]);
                        }
    
                        naked_acc.reset_count();
    
                        let is_naked = naked_acc.get_count() == n as CellSize;
    
                        if is_naked {
                            // println!("Naked group (old):");
                            // println!("{}: {:?}", of_section(si), cell_combo);
                            // println!("{self:?}");

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
                            
                            let inv_naked_acc = naked_acc.inverse();

                            for ci in sec_cells {
                                let cell = &mut self.cells[*ci];

                                if !cell.is_solved()
                                        && cell.has_intersection(inv_naked_acc)
                                        && cell.has_intersection(naked_acc) {
                                    r = true;
                                    cell.intersect_with(inv_naked_acc);
                                }
                            }

                            if r {
                                return /*false*/ true;
                            }
                        }
                    }

                    if check_hidden {
                        let mut sum = 0;

                        for ci in sec_cells {
                            sum += if self.cells[*ci].has_intersection(hidden_acc)
                                   { 1 } else { 0 };
                        }

                        let is_hidden = sum == n;

                        if is_hidden {
                            /*println!("Hidden group:");
                            println!("{}: {}", of_section(si), hidden_acc);
                            println!("{self:?}");*/

                            if n < 4 {
                                for i in 0..9 {
                                    if self.cells[sec_cells[i]].has_intersection(hidden_acc) {
                                        self.section_cell_groups[si][i] = true;
                                    }

                                    if hidden_acc.has_digit(i as CellSize + 1) {
                                        self.section_digit_groups[si][i] = true;
                                    }
                                }
                            }

                            let inv_hidden_acc = hidden_acc.inverse();

                            for ci in sec_cells {
                                let cell = &mut self.cells[*ci];

                                if cell.is_solved()
                                        && cell.has_intersection(hidden_acc)
                                        && cell.has_intersection(inv_hidden_acc) {
                                    r = true;
                                    cell.intersect_with(hidden_acc);
                                }
                            }

                            if r {
                                // println!("{}: {}", of_section(si), hidden_acc);
                                // println!("{self:?}");
                                return /*false*/ true;
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

    pub fn group_removal_new(&mut self) -> bool {
        // This alg only handles naked groups, but I think it should
        // be easy enough to modify for hidden groups if I get a lil
        // wacky with it.

        {/* New Algorithm:
            // first, collect all the different groups for each section which
            // have no overlapping digits.

            section_subsections: [Vec<Vec<usize>>; 27]

            for each section in section range:
                let sec_cells = vec copy of cells in section

                subsections = vec![vec![CELL_EMPTY]]
                    // the first cell in each subsection will be
                    // the accumulator which is used to detect whether
                    // a cell belongs or not.

                subsections[0][0].union_with(first cell in sec_cells)
                subsections[0].push(first cell)

                remove first cell from sec_cells

                for subsection in subsections:
                    while cells have intersection w/ subsection:
                        for cell in sec_cells:
                            if cell intersects with acc:
                                move cell from sec_cells to subsection
                            else if cell is solved or has >4 digits:
                                remove cell from sec_cells

            // alright, onto the real stuff

            for sbs in subsections:
                for pos in 1..sbs.len()-1:
                    g = findGroups(sbs[1..], cell at pos, 1, pos)
                    if g is not null:
                        removeDigits(section cells, g)
                        continue to next sbs

            findGroups(id_list: vec<CellSize>, acc: Cell, cell_count, cid):
                if acc.count or cell_count is > (length of id_list / 2)
                    return null
                if acc.count == cell_count:
                    g = [CELL_EMPTY; cell_count]
                    g[last] = id_list[cid]
                    return g

                for id after cid:
                    g = findGroups(id_list, acc.intersect(cell at id),
                            cell_count+1, id)
                    if g is not null, return g
                return null

            removeDigits(section: [Cell; 9], g: Vec<Cell>):
                // idk yet.
                // but I think it should be easy enough.
        */}

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

        for cid in g {
            acc.union_with(self.cells[cid]);
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

        !changes.is_empty()
    }
}
