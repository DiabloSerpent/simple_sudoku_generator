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

        {/*Algorithm:
            for si in section_range:
                if r and (si % 9 == 0):
                    return true;

                for n in 2..=4:
                    loop through all combinations of size n of cells in section:
                        if combination is a naked group:
                            remove offending digits
                            r = true if changes occurred
                            continue section loop

                        if combination is hidden group:
                            remove offending digits
                            r = true if changes occurred
                            continue section loop
        */}

        {/*Algorithm 2: (unimplemented)
            for si in section_range:
                if ret and (si % 9 == 0):
                    return true

                for sg in section subgroups:
                    for ci in sg:
                        if ci is solved:
                            remove ci from sg

                    let maxgsize = sg.length / 2

                    if maxgsize < 2:
                        remove sg

                    comboloop:
                    for n in 2..=maxgsize:
                        for combo in choose(maxgsize, n):
                            is group hidden?

                            is group naked?

                            if hidden or naked:
                                break comboloop

                    if naked and not hidden:
                        remove digits

                    if hidden and not naked:
                        remove digits

                    if naked or hidden:
                        remove group from sg

                        if group.length / 2 > 1:
                            add group to section subgroups
        */}

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

        {/*for si in SECTION_RANGE {
            if r && (si % 9 == 0) {
                return true;
            }

            let sec      = &SECTION_INDICES[si];
            let sec_sums = &self.section_digit_sum[si];
            let sec_sgs  = &mut self.section_subgroups[si];

            for sg in sec_sgs {

                let mut i = 0;
                while i < sg.len() {
                    if self.cells[sg[i]].is_solved() {
                        sg.swap_remove(i);
                    }
                    else {
                        i += 1;
                    }
                }

                let maxgsize = sg.len() / 2;

                if maxgsize < 2 {
                    continue;
                }

                let mut naked  = false;
                let mut hidden = false;
                let mut group = None;
                let mut acc = [0; 10];

                'combo: for n in 2..=maxgsize {
                    let mut g   = Vec::with_capacity(n);
                    let mut gc  = Vec::with_capacity(n);
                    let mut max = Vec::with_capacity(n);

                    for i in 0..n {
                        g.push(i);
                        gc.push(0);  // this value will be discarded
                        max.push(sg.len() - 1 - i);
                    }

                    max.reverse();

                    // choose n cells from 9 possible cells
                    loop {
                        // extract info
                        acc = [0; 10];

                        for i in 0..n {
                            gc[i] = sg[g[i]];
                        }

                        loop {
                            for ci in &gc {
                                for d in DIGIT_RANGE {
                                    if self.cells[*ci].has_digit(d) {
                                        if acc[d as usize] == 0 {
                                            acc[0] += 1;
                                        }
                                        acc[d as usize] += 1;
                                    }
                                }
                            }

                            naked = acc[0] == n;

                            let mut sum = 0;

                            for d in DIGIT_RANGE {
                                let di = d as usize;
                                if sec_sums[di] == 0 {
                                    continue;
                                }
                                sum += if acc[di] == sec_sums[di].into() { 1 }
                                       else { 0 };
                            }

                            hidden = sum == n;

                            // handle group type
                            if naked != hidden {
                                group = Some(gc);
                                break 'combo;
                            }

                            break;
                        }

                        // I think this iteration management stuff wastes ~5 ms
                        // total of time. Not sure if it needs to be optimized,
                        // but computing this at compile time would be doable.

                        // Alternatively, a separate function could compute
                        // the unsolved cells that each section has and
                        // this could iterate over just those.

                        // OR, this function could save groups that have been
                        // already found and remove them from the pool of cells
                        // to be looked at until they are further modified.

                        // next
                        let mut i = n;

                        while i > 0 && g[i - 1] == max[i - 1] {
                            i -= 1;
                        }

                        if i == 0 {
                            break;
                        }

                        g[i - 1] += 1;

                        if i == n {
                            continue;
                        }

                        while i < n {
                            g[i] = g[i - 1] + 1;
                            i += 1;
                        }
                    }
                }

                if let Some(group) = group {
                    // remove offending digits

                    if naked {
                        for ci in sec {
                            if !group.contains(&ci) {
                                for d in DIGIT_RANGE {
                                    let di = d as usize;
                                    if acc[di] > 0 {
                                        if self.cells[*ci].has_digit(d) {
                                            r = true;
                                        }
                                        self.cells[*ci].remove_digit(d);
                                    }
                                }
                            }
                        }
                    }

                    if hidden {
                        for ci in &group {
                            for d in DIGIT_RANGE {
                                let di = d as usize;
                                if acc[di] > 0 && acc[di] != sec_sums[di].into() {
                                    if self.cells[*ci].has_digit(d) {
                                        r = true;
                                    }
                                    self.cells[*ci].remove_digit(d);
                                }
                            }
                        }
                    }
                }
            }
        }

        r*/}

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
        //     - should be handled by separate
        //let consider_cell  = [[bool; 9]; 27];
        //let consider_digit = [[bool; 9]; 27];

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
                    let sec_sums  = &self.section_digit_sum[si];

                    // TODO: continue if n > max group size in section

                    let mut check_naked  = true;
                    let mut check_hidden = true;

                    for i in 0..n {
                        cell_combo[i] = sec_cells[combo[i]];
                        if self.cells[cell_combo[i]].is_solved() 
                           || self.cells[cell_combo[i]].get_count() > 4 {
                           // TODO: check if cell is part of solved group
                           //       w/ size <4

                            check_naked = false;
                        }

                        if sec_sums[combo[i] + 1] <= 1
                           || sec_sums[combo[i] + 1] > 4 {

                            check_hidden = false;
                        }
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

                /*loop {
                    for ci in &gc {
                        for d in DIGIT_RANGE {
                            if self.cells[*ci].has_digit(d) {
                                if acc[d as usize] == 0 {
                                    acc[0] += 1;
                                }
                                acc[d as usize] += 1;
                            }
                        }
                    }

                    naked = acc[0] == n;

                    let mut sum = 0;

                    for d in DIGIT_RANGE {
                        let di = d as usize;
                        if sec_sums[di] == 0 {
                            continue;
                        }
                        sum += if acc[di] == sec_sums[di].into() { 1 }
                               else { 0 };
                    }

                    hidden = sum == n;

                    // handle group type
                    if naked != hidden {
                        group = Some(gc);
                        break 'combo;
                    }

                    break;
                }*/

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


// Currently unusable, increases program time by ~100 ms
// prolly because of the cloning, need to figure out lifetime stuff

/*fn combo_array(n: usize, r: usize) -> ComboIter {
    let mut s = ComboIter {
        r,
        current: Vec::from_iter((0..n).take(r)),
        max: Vec::from_iter((0..n).rev().take(r).rev()),
    };

    s.current[r - 1] -= 1;

    s
}

struct ComboIter {
    r: usize,
    current: Vec<usize>,
    max: Vec<usize>,
}

impl Iterator for ComboIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.r;

        while i > 0 && self.current[i - 1] >= self.max[i - 1] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        self.current[i - 1] += 1;

        if i == self.r {
            return Some(self.current.clone());
        }

        while i < self.r {
            self.current[i] = self.current[i - 1] + 1;
            i += 1;
        }

        Some(self.current.clone())
    }
}*/