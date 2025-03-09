use rand::Rng;

use crate::cell::{Cell, CellSize, CELL_INIT, DIGIT_RANGE};
use crate::index_manip::*;
use crate::history::{HistoryEntry, EntryType};

mod cell_solved;
mod hidden_single;
mod intersection_removal;
mod naked_single;
mod group_removal;
mod graphics;

pub struct Sudoku {
    // May want to replace array w/ set or smth
    pub cells: [Cell; 81],
    solved_cell_checked: [bool; 81],
    section_digit_sum: [[CellSize; 10]; 27],
    section_cell_groups: [[bool; 9]; 27],
    section_digit_groups: [[bool; 9]; 27],
    pub history: Vec<HistoryEntry>,
}

/* Structure:
    cells:
        all cells in a sudoku board, index 0 is top left,
        index 80 is the bottom right.
        Read left to right, top to bottom.

    cell_flags:
        flags associated with each individual cell,
        used by the sudoku rule associated functions.
        For each element:
            bit 0:
                used by cell_solved()
                1 denotes a cell that already has a solution
                and was checked by cell_solved(), 0 otherwise

    section_digit_sum:
        sum of each digit in each section
        the first 9 sections are the rows, then cols, then boxes
        each element will have:
            first entry: sum of entries in the section w/ sum above 1
            entries 1-9: sum of count per section corresponding to each digit
*/

pub const CELL_SOLVED: u8 = 0b00000001;

impl Sudoku {
    pub fn new() -> Self {
        Self {
            cells: [CELL_INIT; 81],
            solved_cell_checked: [false; 81],
            section_digit_sum: [[0; 10]; 27],
            section_cell_groups: [[false; 9]; 27],
            section_digit_groups: [[false; 9]; 27],
            history: Vec::with_capacity(1000),
        }
    }

    pub fn fill_incremental() -> Self {
        let mut s = Self::new();

        for i in 0..81 {
            if s.cells[i].is_solved() {
                continue;
            }

            s.rs_cell(i);

            s.solve();

            //println!("{s:?}");
        }

        s
    }

    pub fn fill_random() -> Self {
        let mut s = Self::new();

        let mut cell_pool = Vec::from_iter(0..81);

        let mut r = rand::thread_rng();

        while !cell_pool.is_empty() {
            let i = r.gen_range(0..cell_pool.len());

            if !s.cells[cell_pool[i]].is_solved() {

                s.rs_cell(cell_pool[i]);
            
                s.solve();

                //println!("{s:?}");
            }

            cell_pool.swap_remove(i);
        }

        s
    }

    fn rs_cell(&mut self, c: usize) {
        self.cells[c].generate_number();

        self.history.push(HistoryEntry::from_solution(
            EntryType::RsCell, c, self.cells[c]));
        //println!("{}\n{:?}", self.history.last().unwrap(), self);
    }

    /*fn generate_subgroups() -> [Vec<Vec<usize>>; 27] {
        let mut sg: [Vec<Vec<usize>>; 27] = Default::default();

        for si in SECTION_RANGE {
            sg[si].push(Vec::from(SECTION_INDICES[si]));
        }

        sg
    }*/

    pub fn solve(&mut self) {
        // Idea:
        // loop through all rules until none result in a modification.
        // 
        // Important: digits should only be removed, never added
        // 
        // if/when a rule finishes applying changes,
        // the loop should restart.
        // if a rule solves a cell, then the loop at the top should run again.
        // will prolly be more convenient if rules have state
        // 
        // rule 1: solved cells remove digit from related cells
        // rule 2: cells w/ only 1 digit should be solved
        // rule 3: if a cell is the only one with a given digit
        //         in its row/box/col, then it should be solved
        // so on, so forth

        'ruling: loop {
            for rule in Self::RULE_ORDER {
                if rule(self) {
                    // This is basically just a complicated goto statement
                    continue 'ruling;
                }
            }
            break;
        }
    }

    // Each rule returns true if sudoku was modified,
    // false otherwise.
    const RULE_ORDER: &'static [fn(&mut Self) -> bool] = &[
        Self::cell_solved,
        Self::naked_single,
        Self::update_section_digit_sum,
        Self::hidden_single,
        Self::intersection_removal,
        Self::group_removal,
        //Self::group_removal_new,
    ];

    fn update_section_digit_sum(&mut self) -> bool {
        // Update the contents of the variable that keeps track of
        // the count of digits by section

        for si in SECTION_RANGE {
            let mut temp = [0; 10];

            for ci in SECTION_INDICES[si] {
                for di in DIGIT_RANGE {
                    if self.cells[ci].has_digit(di) {
                        let di = di as usize;

                        if temp[di] == 1 {
                            temp[0] += 1;
                        }

                        temp[di] += 1;
                    }
                }
            }

            self.section_digit_sum[si] = temp;
        }

        // This method does not modify the sudoku board
        false
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..81 {
            if !self.cells[i].is_solved() {
                return false;
            }
        }

        return true;
    }

    pub fn get_section_status(&self) -> [bool; 27] {
        let mut section_status = [true; 27];

        for si in SECTION_RANGE {
            let sums = self.section_digit_sum[si];

            if sums[1..].iter().position(|&x| x != 1) != None {
                section_status[si] = false;
            }
        }

        section_status
    }

    pub fn is_valid(&self) -> bool {
        debug_assert!(self.is_solved(), "only solved sudoku can be valid");

        let sec_stat = self.get_section_status();

        for b in sec_stat {
            if !b {
                return false;
            }
        }

        return true;
    }
}