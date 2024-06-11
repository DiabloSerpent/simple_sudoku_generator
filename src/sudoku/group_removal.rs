use crate::Sudoku;
use crate::cell::{DIGIT_RANGE};
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

        {/*Algorithm: (unimplemented)
            for si in section_range:
                if ret and (si % 9 == 0):
                    return true

                for sg in self.subgroups[si]:
                    for ci in sg:
                        if ci is solved:
                            remove ci from sg

                    // would it be convenient to just remove these?
                    if sg.length < 4:
                        continue
                    let maxgsize = sg.length / 2

                    for r in 2..=maxgsize:
                        for combo in C^{maxgsize}_{r}:
                            if combo is naked group:
                                if combo.length >= 4:
                                    self.subgroups[si].push(combo)
                                remove combo from sg

                                remove offending digits
                                ret = true if changes occurred
                                continue section loop

                            if combo is hidden group:
                                ...
        */}

        let mut r = false;

        for si in SECTION_RANGE {
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

        r
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