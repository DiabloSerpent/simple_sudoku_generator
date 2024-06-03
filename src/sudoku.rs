use std::fmt;

use crate::cell::{Cell, CellSize, CELL_INIT, DIGIT_RANGE};
use crate::index_manip::*;

mod cell_solved;
mod hidden_single;
mod intersection_removal;
mod naked_single;

pub struct Sudoku {
    // May want to replace array w/ set or smth
    pub cells: [Cell; 81],
    cell_flags: [u8; 81],
    section_digit_sum: [[CellSize; 10]; 27],
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
    pub fn new() -> Sudoku {
        Sudoku {
            cells: [CELL_INIT; 81],
            cell_flags: [0; 81],
            section_digit_sum: [[0; 10]; 27],
        }
    }

    pub fn check(&self) {
        let mut section_status = [false; 27];

        let mut do_print = false;

        for si in SECTION_RANGE {
            let sums = self.section_digit_sum[si];

            if sums[1..].iter().position(|&x| x != 1) != None {
                section_status[si] = true;
                do_print = true;
            }
        }

        if !do_print {
            println!("No invalid cells");
            return;
        }
        else {
            println!("Invalid solutions:");
        }

        println!("       | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |");

        for si in SECTION_RANGE {
            if si % 9 == 0 {
                println!("       |---|---|---|---|---|---|---|---|---|");
            }

            let sums = self.section_digit_sum[si];

            if !section_status[si] {
                continue;
            }

            print!("{}: |", of_section(si));

            for j in DIGIT_RANGE {
                let j = j as usize;
                if sums[j] == 1 {
                    print!("   |");
                }
                else {
                    print!(" {} |", sums[j]);
                }
            }
            println!();
        }
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
    const RULE_ORDER: [fn(&mut Sudoku) -> bool; 8] = [
        Sudoku::cell_solved,
        Sudoku::naked_single,
        Sudoku::update_section_digit_sum,
        Sudoku::hidden_single,
        Sudoku::intersection_removal,
        Sudoku::naked_pair,
        Sudoku::hidden_pair,
        Sudoku::naked_group,
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

    fn naked_pair(&mut self) -> bool {
        // A row/box/col has a pair of cells that only have
        // 2 equal digits remaining
        false // TODO
    }
    fn hidden_pair(&mut self) -> bool {
        // A row/box/col has a pair of cells that are the only cells
        // that can hold 2 digits
        false // TODO
    }
    fn naked_group(&mut self) -> bool {
        // A row/box/col has a group of cells that
        // must be a particular set of digits
        // The group size must be >3 and <=7
        // Total amt of digits shared must be equal to group size
        // Maximum amt of groups in a row/box/col: 3

        // Could make it so that the program loops through each
        // type of section before returning when a change is made,
        // so that none of the sections changed overlap with each other.

        // note: the function should return after a change is made,
        //       b/c it could read cells it has already changed.

        // Def of naked group:
        // a set of cells whose total count of digits is equal to
        // the size of the set and cannot be resolved to a simpler group.

        {/* this alg also does hidden/naked pairs
           going into this, there should be no hidden/naked singles

           it might also be prudent to have this restart every time
           a change is made, as it can modify the things it is searching for.

           I think this alg would break on something like:
           2 cells w/ 478, 1 cell w/ 48, 1 cell w/ 49
           b/c it would assume the 48 and 49 cell form a
           group when they do not

            for each section:
                // Unforch magic number (5); this is just the maximum
                // amt of cells/groups that can fit in a section:
                // 4 pairs and 1 cell
                let there be 5 groups, each with an accumulator cell
                    and a count of cells in the group
                sort cells of section by increasing digit count
                    and remove solved cells
                let the first group start with the first cell, with
                    count equal to the count of the cell
                for each cell in section (sorted, excluding first):
                    for g in groups:
                        if g.acc.digits & cell.digits != 0:
                            g.acc = g.acc union cell
                            g.count++
                    if cell wasn't in any group:
                        create a new group starting w/ the cell
                    if a group has a count equal to the count of its digits:
                        consider the group to be "cemented" and remove it
                            from the list of groups being iterated over

                // Ideally, all of the groups will be "cemented" at this point
                for g in groups:
                    for cell in section:
                        if cell and g.acc have overlapping digits:
                            remove the non-overlapping digits from cell
        */}

        false // TODO
    }
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
            (assumes that a self.section_profile variable exists)

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

        false // TODO
    }
}

// It's just, so PEAK
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This is only intended to display a completed sudoku.

        let top  = "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n";
        let mid  = "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n";
        let boxl = "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n";
        let bot  = "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n";

        write!(f, "{}", top)?;

        for i in 0..9 {
            write!(
                f, "║ {} │ {} │ {} ║ {} │ {} │ {} ║ {} │ {} │ {} ║\n",
                self.cells[i*9 + 0].get_number(),
                self.cells[i*9 + 1].get_number(),
                self.cells[i*9 + 2].get_number(),
                self.cells[i*9 + 3].get_number(),
                self.cells[i*9 + 4].get_number(),
                self.cells[i*9 + 5].get_number(),
                self.cells[i*9 + 6].get_number(),
                self.cells[i*9 + 7].get_number(),
                self.cells[i*9 + 8].get_number(),
            )?;

            if i == 8 {
                write!(f, "{}", bot)?;
            }
            else if i % 3 == 2 {
                write!(f, "{}", boxl)?;
            }
            else {
                write!(f, "{}", mid)?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This shows all cells as a collection of digits,
        // regardless of whether they are already solved.

        let top  = "╔═══════╤═══════╤═══════╦═══════╤═══════╤═══════╦═══════╤═══════╤═══════╗\n";
        let mid  = "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢\n";
        let boxl = "╠═══════╪═══════╪═══════╬═══════╪═══════╪═══════╬═══════╪═══════╪═══════╣\n";
        let bot  = "╚═══════╧═══════╧═══════╩═══════╧═══════╧═══════╩═══════╧═══════╧═══════╝\n";
        //║       │       │       ║       │       │       ║       │       │       ║

        write!(f, "{}", top)?;

        for ri in 0..9 {
            for digit_row in 0..3 {
                for ci in 0..9 {
                    if ci != 0 {
                        write!(f, " ")?;
                    }
                    if ci % 3 == 0 {
                        write!(f, "║")?;
                    }
                    else {
                        write!(f, "│")?;
                    }

                    let cell = self.cells[((ri * 9) + ci) as usize];

                    for di in 0..3 {
                        if cell.has_digit(digit_row * 3 + di + 1) {
                            write!(f, " {}", digit_row * 3 + di + 1)?;
                        }
                        else {
                            write!(f, "  ")?;
                        }
                    }
                }
                write!(f, " ║\n")?;
            }

            if ri == 8 {
                write!(f, "{}", bot)?;
            }
            else if ri % 3 == 2 {
                write!(f, "{}", boxl)?;
            }
            else {
                write!(f, "{}", mid)?;
            }
        }

        Ok(())
    }
}