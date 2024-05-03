use rand::Rng;
use std::fmt;
use std::ops::{RangeInclusive, Range};

// Shamelessly ripped from:
// https://codegolf.stackexchange.com/questions/126930/draw-a-sudoku-board-using-line-drawing-characters

const _SUDOKU_BOARD: &str = "\
╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
║   │   │   ║   │   │   ║   │   │   ║\n\
╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝";

// Helpful website:
// https://www.sudokuwiki.org/Getting_Started


#[derive(Debug)]
struct Sudoku {
    // May want to replace array w/ set or smth
    cells: [Cell; 81],
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

const CELL_SOLVED: u8 = 0b00000001;

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku {
            cells: [CELL_INIT; 81],
            cell_flags: [0; 81],
            section_digit_sum: [[0; 10]; 27],
        }
    }

    fn check(&self) {
        // Each sub array of section_status holds
        // a count of the solved numbers in the section.
        // Structure:
        //      entry 0: holds whether or not there is an error,
        //      entries 1-9: how many cells hold the number
        //                   corresponding to the entry
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

    fn solve(&mut self) {
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

    fn cell_solved(&mut self) -> bool {
        // A solved cell should remove the solution from
        // the related digits
        // 
        // Struct memory usage: 81 bools
        for i in 0..81 {
            if (self.cell_flags[i] & CELL_SOLVED) == 0
               && self.cells[i].is_solved() {

                self.cell_flags[i] |= CELL_SOLVED;

                let to_remove = self.cells[i].get_number();

                if to_remove == 0 {
                    continue;
                }

                let (irow, icol, ibox) = (
                    of_row(row_of(i)),
                    of_col(col_of(i)),
                    of_box(box_of(i))
                );

                for j in 0..9 {
                    self.cells[irow[j]].remove_digit(to_remove);
                    self.cells[icol[j]].remove_digit(to_remove);
                    self.cells[ibox[j]].remove_digit(to_remove);
                }
            }
            // TODO: add else stmt to handle zeroed cells
        }

        // This rule doesn't solve any cells,
        // so it will never need to loop back
        // to itself. (assuming it always goes first!)
        false
    }

    fn naked_single(&mut self) -> bool {
        // A cell has only one digit left
        // 
        // Struct Memory usage: N/A
        let mut r = false;

        for i in 0..81 {
            if !self.cells[i].is_solved()
               && self.cells[i].get_count() == 1 {

                let mut digit = 0;

                for d in DIGIT_RANGE {
                    if self.cells[i].has_digit(d) {
                        digit = d;
                        break;
                    }
                }

                self.cells[i].solve_cell(digit);

                r = true;
            }
        }

        r
    }

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

    fn hidden_single(&mut self) -> bool {
        // A row/col/box has only one cell with a particular digit
        // 
        // Struct memory usage: N/A, won't save time using it
        let mut r = false;

        for si in SECTION_RANGE {
            let sums = self.section_digit_sum[si];

            for j in DIGIT_RANGE {
                let count = sums[j as usize];

                if count == 1 {
                    for ci in SECTION_INDICES[si] {
                        if !self.cells[ci].is_solved()
                           && self.cells[ci].has_digit(j) {

                            self.cells[ci].solve_cell(j);

                            r = true;
                        }
                    }
                }
            }
        }

        r
    }

    fn intersection_removal(&mut self) -> bool {
        // Remove digits that are outside of an intersection.

        false // TODO
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

        /* this alg also does hidden/naked pairs
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
        */
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

        if let Err(e) = write!(f, "{}", top) {
            return Err(e);
        }

        for i in 0..9 {
            if let Err(e) = write!(
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
            ) {
                return Err(e);
            }
            // me when the "government" doesn't let me drive 120 mph in a school zone
            match if i == 8 {
                write!(f, "{}", bot)
            }
            else if i % 3 == 2 {
                write!(f, "{}", boxl)
            }
            else {
                write!(f, "{}", mid)
            } {
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        }

        Ok(())
    }
}

type CellIndex = usize;
type RowIndex = usize;
type ColIndex = usize;
type BoxIndex = usize;
type SecIndex = usize;

// cells, rows, cols, boxs are all 0 indexed
fn row_of(cell_index: CellIndex) -> RowIndex {
    cell_index / 9
}
fn col_of(cell_index: CellIndex) -> ColIndex {
    cell_index % 9
}
fn box_of(cell_index: CellIndex) -> BoxIndex {
    (cell_index / 3) % 3 + (cell_index / 27) * 3
}

// OPTIMIZE LATER (These only need to be computed once)
const fn of_row(row_index: RowIndex) -> [CellIndex; 9] {
    let mut r: [usize; 9] = [0; 9];

    let mut i = 0;
    while i < 9 {
        r[i] = i + (row_index * 9);
        i += 1;
    }

    r
}
const fn of_col(col_index: ColIndex) -> [CellIndex; 9] {
    let mut r: [usize; 9] = [0; 9];

    let mut i = 0;
    while i < 9 {
        r[i] = i * 9 + col_index;
        i += 1;
    }

    r
}
const fn of_box(box_index: BoxIndex) -> [CellIndex; 9] {
    let mut r: [usize; 9] = [0; 9];
    let start = (box_index % 3) * 3 + (box_index / 3) * 27;

    let mut i = 0;
    while i < 3 {
        r[i] = start + i;
        r[i+3] = start + i + 9;
        r[i+6] = start + i + 18;
        i += 1;
    }

    r
}
fn related_cells(index: CellIndex) -> [CellIndex; 21] {
    // size of returned array will be 9 + 8 + 4
    // maybe start w/ box, then add row/col?
    // It may also just be too complicated to exclude duplicates
    todo!()
}

const SECTION_RANGE: Range<SecIndex> = 0..27;

enum SectionType {
    RowSection(RowIndex),
    ColSection(ColIndex),
    BoxSection(BoxIndex),
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RowSection(i) => write!(f, "Row {}", i + 1),
            ColSection(i) => write!(f, "Col {}", i + 1),
            BoxSection(i) => write!(f, "Box {}", i + 1),
        }
    }
}

use SectionType::*;

const SECTION_INDICES: [[CellIndex; 9]; 27] = make_section_index();

const fn make_section_index() -> [[CellIndex; 9]; 27] {
    let mut temp = [[0; 9]; 27];

    // I love code repetition

    let mut i = 0;
    while i < 9 {
        temp[i] = of_row(i / 3);
        i += 1;
    }
    let mut i = 0;
    while i < 9 {
        temp[i+9] = of_col(i / 3);
        i += 1;
    }
    let mut i = 0;
    while i < 9 {
        temp[i+18] = of_box(i / 3);
        i += 1;
    }

    temp
}

fn section_of(s: SectionType) -> SecIndex {
    match s {
        RowSection(i) => i,
        ColSection(i) => i + 9,
        BoxSection(i) => i + 18,
    }
}

fn of_section(si: SecIndex) -> SectionType {
    let i = si % 9;

    match si / 9 {
        0 => RowSection(i),
        1 => ColSection(i),
        2 => BoxSection(i),
        _ => panic!("Invalid section index {i}")
    }
}


/* Structure:
    bit 0: if set, cell is considered to be solved
        there should only be one digit set or none
        this also controls the meaning of bits 10-13:
            if bit 0 is set, they signify the number and the count is 1
            if unset, they signify the count of digits and the number is 0
    bit 1-9: cell can have numbers 1-9
    bit 10-13: the selected number in binary,
               or the count of the digits
        this should never have a value above decimal 9
        zero means no valid digit or no selected digit
    bit 14-15: unused
*/
type CellSize = u16;

#[derive(Debug, Clone, Copy)]
struct Cell(CellSize);

// I actually don't know if Windows is lil-endian or big-endian.
// I also don't care.
const SOLUTION_MASK: CellSize = 0b00000000_00000001;
const DIGIT_MASK:    CellSize = 0b00000011_11111110;
const NUMBER_MASK:   CellSize = 0b00111100_00000000;
const COUNT_MASK:    CellSize = NUMBER_MASK;
const _UNUSED_MASK:  CellSize = 0b11000000_00000000;

const DIGIT_RANGE: RangeInclusive<CellSize> = 1..=9;
fn DIGIT(x: CellSize) -> CellSize {
    debug_assert!(
        *DIGIT_RANGE.start() <= x && x <= *DIGIT_RANGE.end(),
        "{x} is not a valid digit!"
    );
    
    1 << x
}

const NUM_SHIFT:   u16 = 10;
const COUNT_SHIFT: u16 = NUM_SHIFT;

const CELL_INIT: Cell = Cell(DIGIT_MASK | (9 << COUNT_SHIFT));

impl Cell {
    fn get_number(&self) -> CellSize {
        if self.is_solved() {
            (self.0 & NUMBER_MASK) >> NUM_SHIFT
        }
        else {
            0
        }
    }

    fn get_count(&self) -> CellSize {
        if self.is_solved() {
            if self.0 & DIGIT_MASK == 0 {
                0
            }
            else {
                1
            }
        }
        else {
            (self.0 & COUNT_MASK) >> COUNT_SHIFT
        }
    }

    fn has_digit(&self, digit: CellSize) -> bool {
        self.0 & DIGIT(digit) != 0
    }

    fn remove_digit(&mut self, digit: CellSize) {
        // assert(digit is in digit_range);
        if self.is_solved() {
            return;
        }

        if self.has_digit(digit) {
            let c = self.get_count() - 1;
            self.0 = (self.0 & !COUNT_MASK) | (c << COUNT_SHIFT);
        }

        // It doesn't matter if the cell has the digit for this operation
        self.0 &= !DIGIT(digit);
    }

    fn solve_cell(&mut self, digit: CellSize) {
        if self.is_solved() {
            return;
        }

        debug_assert!(
            digit == 0 || self.has_digit(digit),
            "digit {digit} is not available in {self}"
        );

        self.0 = ((self.0 & !DIGIT_MASK) & !COUNT_MASK)
                   | SOLUTION_MASK;

        // A cell being solved to 0 should be allowed
        if digit != 0 {
            self.0 |= DIGIT(digit) | (digit << NUM_SHIFT);
        }
    }

    fn is_solved(&self) -> bool {
        (self.0 & SOLUTION_MASK) != 0
    }

    fn union(&self, c: Cell) -> Cell {
        todo!()
    }

    fn intersection(&self, c: Cell) -> Cell {
        todo!()
    }

    fn generate_number(&mut self) {
        if self.is_solved() {
            return;
        }

        let mut chosen = 0;
        let mut factor = -1.0;
        let mut r = rand::thread_rng();

        // Not sure if this is absolutely perfect,
        // but it works.
        for i in DIGIT_RANGE {
            if self.has_digit(i) {
                let f = r.gen_range(0.0..=1.0);
                if f > factor {
                    chosen = i;
                    factor = f;
                }
            }
        }

        self.solve_cell(chosen);
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cell({:0>16b})", self.0)
    }
}


fn main() {
    let mut sud = Sudoku::new();

    // Would be nice if there was a convenient way to randomly select
    // a cell each time. It's not really necessary tho.
    for i in 0..81 {
        sud.cells[i].generate_number();

        sud.solve();
    }

    println!("{}", sud);

    sud.check();
}
