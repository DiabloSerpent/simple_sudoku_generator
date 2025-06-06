use rand::Rng;

use crate::cell::{Cell, CELL_INIT, DIGIT_RANGE};
use crate::index_manip::*;
use crate::history::{HistoryEntry, EntryType, CellChange};

mod cell_solved;
mod hidden_single;
mod intersection_removal;
mod naked_single;
mod group_removal;
mod graphics;


pub type SudokuBoard = [Cell; 81];

pub struct Sudoku {
    pub cells: SudokuBoard,
    solved_cell_checked: [bool; 81],
    section_digit_sum: [[usize; 10]; 27],
    cell_change_stack: Vec<CellChange>,
    pub history: Vec<HistoryEntry>,
}

/* Structure:
    cells:
        all cells in a sudoku board, index 0 is top left,
        index 80 is the bottom right.
        Read left to right, top to bottom.

    solved_cell_checked:
        used by cell_solved() to flag which cells have already been
        checked by the code.

    section_digit_sum:
        sum of each digit in each section
        the first 9 sections are the rows, then cols, then boxes
        each element will have:
            first entry: sum of entries in the section w/ sum above 1
            entries 1-9: sum of count per section corresponding to each digit

    cell_change_stack:
        helper var that is used to keep track of changes made by rules in
        an unobtrusive way

    history:
        a complete record of changes made by the solving process
*/

// Not sure if this is going to be useful, but I might as well keep it here.
impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            cells: [CELL_INIT; 81],
            solved_cell_checked: [false; 81],
            section_digit_sum: [[9; 10]; 27],
            cell_change_stack: Vec::with_capacity(27),
            history: Vec::with_capacity(1000),
        }
    }

    pub fn from_solutions(sol_arr: [usize; 81]) -> Self {
        let mut s = Self::new();

        for i in 0..81 {
            // Unfortunately the sudoku can't tell if the cell should be
            // undeveloped or solved to 0. But the solve() method can be
            // called to remedy that.
            if sol_arr[i] != 0 {
                let mut c = CELL_INIT;
                c.solve_cell(sol_arr[i]);
                s.cells[i] = c;
            }
        }

        s
    }

    pub fn fill_incremental() -> Self {
        let mut s = Self::new();

        for i in 0..81 {
            if s.cells[i].is_solved() {
                continue;
            }

            s.rs_cell(i);

            s.solve();
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
            }

            cell_pool.swap_remove(i);
        }

        s
    }

    fn rs_cell(&mut self, c: usize) {
        self.cells[c].generate_number();

        self.add_history_entry_from_solution(EntryType::RsCell, c);
    }

    pub fn register_change(&mut self, id: usize) {
        self.cell_change_stack.push(CellChange {
            id,
            new_cell: self.cells[id],
        });
    }

    pub fn has_changes(&self) -> bool {
        !self.cell_change_stack.is_empty()
    }

    pub fn add_history_entry(&mut self, name: EntryType,
                             cells: Vec<usize>, digits: Cell) {
        debug_assert!(self.has_changes(),
            "self.cell_change_stack shouldn't be empty");

        self.history.push(HistoryEntry {
            name,
            cells,
            digits,
            changes: self.cell_change_stack.clone(),
        });

        self.cell_change_stack.clear();
    }

    pub fn add_history_entry_from_solution(&mut self, name: EntryType,
                                                      id: usize) {
        debug_assert!(!self.has_changes(),
            "self.cell_change_stack should be empty");

        let cell = self.cells[id];

        self.history.push(HistoryEntry {
            name,
            cells: vec![id],
            digits: cell.get_unsolved_copy().inverse(),
            changes: vec![CellChange {id, new_cell: cell}]});
    }

    pub fn add_history_entry_if_changes(&mut self, name: EntryType,
                                cells: Vec<usize>, digits: Cell) -> bool {
        let r = self.has_changes();

        if r {
            self.add_history_entry(name, cells, digits);
        }

        r
    }

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
    ];

    fn update_section_digit_sum(&mut self) -> bool {
        // Update the contents of the variable that keeps track of
        // the count of digits by section

        for si in SECTION_RANGE {
            let mut temp = [0; 10];

            for ci in SECTION_INDICES[si] {
                for di in DIGIT_RANGE {
                    if self.cells[ci].has_digit(di) {
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

        true
    }

    pub fn get_section_status(&self) -> [bool; 27] {
        let mut section_status = [true; 27];

        for si in SECTION_RANGE {
            let sums = self.section_digit_sum[si];

            if sums[1..].iter().any(|&x| x != 1) {
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

        true
    }
}