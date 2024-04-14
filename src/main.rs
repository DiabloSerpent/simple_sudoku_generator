use rand::Rng;
use std::fmt;
use std::ops::RangeInclusive;

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

#[derive(Debug)]
struct Sudoku {
    // May want to replace array w/ set or smth
    cells: [Cell; 81],
    invalid_cells: Vec<usize>,
    cell_flags: [u8; 81],
}

/* Structure:
    cells:
        all cells in a sudoku board, index 0 is top left,
        index 80 is the bottom right.
        Read left to right, top to bottom.

    cell_flags:
        flags associated with each individual cell,
        used by the sudoku rule associated functions.
        bit 0: (of each value)
            used by cell_solved()
            1 denotes a cell that already has a solution
            and was checked by cell_solved(), 0 otherwise
*/

const CELL_SOLVED: u8 = 0b00000001;

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku {
            cells: [DIGIT_MASK; 81],
            invalid_cells: Vec::new(), // blank_cells ?
            cell_flags: [0; 81],
        }
    }

    fn check(&self) {
        let sections = [of_row, of_col, of_box];
        let names = ["Row", "Col", "Box"];

        // Each sub array of section_status holds
        // a count of the solved numbers in the section.
        // Structure:
        //      entry 0: holds whether or not there is an error,
        //      entries 1-9: how many cells hold the number
        //                   corresponding to the entry
        let mut section_status = [[0; 10]; 27];

        let mut do_print = false;

        for i in 0..9 {
            for (si, s) in sections.iter().enumerate() {
                let section_cell_indices = s(i);
                let number_count = &mut section_status[i*3 + si];

                for j in section_cell_indices {
                    let n = get_number(self.cells[j]) as usize;
                    if n != 0 {
                        number_count[n] += 1;
                    }
                }

                // number_count[0] = 1 if sudoku error, 0 otherwise
                if number_count[1..].iter().position(|&x| x != 1) != None {
                    number_count[0] = 1;
                    do_print = true;
                }
            }
        }

        if !do_print {
            return;
        }

        println!("       | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |");

        for i in 0..9 {
            for si in 0..3 {
                let number_count = &section_status[i*3 + si];
                if number_count[0] == 0 {
                    continue;
                }

                print!("{} {i}: |", names[si]);

                for j in 1..=9 {
                    if number_count[j] == 1 {
                        print!("   |");
                    }
                    else {
                        print!(" {} |", number_count[j]);
                    }
                }
                println!();
            }
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
    const RULE_ORDER: [fn(&mut Sudoku) -> bool; 6] = [
        Sudoku::cell_solved,
        Sudoku::naked_single,
        Sudoku::hidden_single,
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
               && get_number(self.cells[i]) != 0 {

                let remove_mask = !(1 << get_number(self.cells[i]));
                let (irow, icol, ibox) = (
                    of_row(row_of(i)),
                    of_col(col_of(i)),
                    of_box(box_of(i))
                );

                for j in 0..9 {
                    self.cells[irow[j]] &= remove_mask;
                    self.cells[icol[j]] &= remove_mask;
                    self.cells[ibox[j]] &= remove_mask;
                }

                self.cell_flags[i] |= CELL_SOLVED;
            }
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

        'cell_loop: for i in 0..81 {
            if get_number(self.cells[i]) == 0 
               && self.cells[i] & DIGIT_MASK != 0 {

                let mut digit = 0;

                for d in 1..=9 {
                    if self.cells[i] & (1 << d) != 0 {
                        if digit == 0 {
                            digit = d;
                        }
                        else {
                            continue 'cell_loop;
                        }
                    }
                }

                self.cells[i] |= digit << NUM_SHIFT;

                r = true;
            }
        }

        r
    }

    fn hidden_single(&mut self) -> bool {
        // A row/col/box has only one cell with a particular digit
        // 
        // Struct memory usage: N/A, won't save time using it
        let mut r = false;

        // This code is half copied from Sudoku::check

        let sections = [of_row, of_col, of_box];

        for i in 0..9 {
            for s in sections {
                let section_cell_indices = s(i);
                let mut digit_count = [0; 9];

                for ci in section_cell_indices {
                    for j in 1..=9 {
                        if self.cells[ci] & (1 << j) != 0 {
                            digit_count[j-1] += 1;
                        }
                    }
                }

                for j in 1..=9 {
                    let count = digit_count[j-1];

                    if count == 1 {
                        for ci in section_cell_indices {
                            if get_number(self.cells[ci]) == 0
                               && self.cells[ci] & (1 << j) != 0 {

                                self.cells[ci] &= !DIGIT_MASK;
                                self.cells[ci] |= 1 << j;
                                self.cells[ci] |= (j << NUM_SHIFT) as Cell;

                                r = true;
                            }
                        }
                    }
                }
            }
        }

        r
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

        match write!(f, "{}", top) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
        for i in 0..9 {
            match write!(
                f, "║ {} │ {} │ {} ║ {} │ {} │ {} ║ {} │ {} │ {} ║\n",
                get_number(self.cells[i*9 + 0]),
                get_number(self.cells[i*9 + 1]),
                get_number(self.cells[i*9 + 2]),
                get_number(self.cells[i*9 + 3]),
                get_number(self.cells[i*9 + 4]),
                get_number(self.cells[i*9 + 5]),
                get_number(self.cells[i*9 + 6]),
                get_number(self.cells[i*9 + 7]),
                get_number(self.cells[i*9 + 8]),
            ) {
                Err(e) => return Err(e),
                Ok(_) => {}
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

        if self.invalid_cells.len() == 0 {
            write!(f, "No invalid cells")
        }
        else {
            write!(f, "{} invalid cells", self.invalid_cells.len())
        }
    }
}

// cells, rows, cols, boxs are all 0 indexed
fn row_of(cell_index: usize) -> usize {
    cell_index / 9
}
fn col_of(cell_index: usize) -> usize {
    cell_index % 9
}
fn box_of(cell_index: usize) -> usize {
    (cell_index / 3) % 3 + (cell_index / 27) * 3
}

// OPTIMIZE LATER
fn of_row(row_index: usize) -> [usize; 9] {
    let mut r: [usize; 9] = [0; 9];

    for i in 0..9 {
        r[i] = i + (row_index * 9);
    }

    r
}
fn of_col(col_index: usize) -> [usize; 9] {
    let mut r: [usize; 9] = [0; 9];

    for i in 0..9 {
        r[i] = i * 9 + col_index;
    }

    r
}
fn of_box(box_index: usize) -> [usize; 9] {
    let mut r: [usize; 9] = [0; 9];
    let start = (box_index % 3) * 3 + (box_index / 3) * 27;

    for i in 0..3 {
        r[i] = start + i;
        r[i+3] = start + i + 9;
        r[i+6] = start + i + 18;
    }

    r
}
fn related_cells(index: usize) -> [usize; 21] {
    // size of returned array will be 9 + 8 + 4
    // maybe start w/ box, then add row/col?
    // It may also just be too complicated to exclude duplicates
    todo!()
}

// If I removed the invalid bit, it would be possible
// to also store the digit count of the cell within
// the cell.
// It would just be a lil inconvenient to access
// and also would need to be updated manually.
// to accomplish: remove invalid bit, shift digit/number masks right by 1
//      then, let bits 13-15 be the digit count.
//      bit values of 1-7 will mean a count of 2-8
//      bit value of 0 will mean 0, 1, or 9
//      to differentiate them:
//          count of 0: cell & digit_mask == 0
//          count of 9: cell & digit_mask == digit_mask
//          count of 1: cell & digit_mask != 0, != digit_mask
// It might also be possible to use the number mask
// for both the solved cell and the cell count,
// as a solved cell will always have a count of 1.
// Also would need to switch invalid bit to solution bit.

/* Structure:
    bit 0: if set, cell should be filled randomly
    bit 1-9: cell can have numbers 1-9
    bit 10-13: the selected number in binary
        this should never have a value above decimal 10
        zero means no valid digit or no selected digit
    bit 14-15: unused
*/
type Cell = u16;

// I actually don't know if Windows is lil-endian or big-endian.
// I also don't care.
const INVALID_MASK: Cell = 0b00000000_00000001;
const DIGIT_MASK: Cell   = 0b00000011_11111110; // Default initialization
const NUMBER_MASK: Cell  = 0b00111100_00000000;
const DIGIT_RANGE: RangeInclusive<u16> = 1..=9;
const NUM_SHIFT: u16     = 10;

fn get_number(c: Cell) -> u16 {
    (c & NUMBER_MASK) >> NUM_SHIFT
}

fn count_digits(c: Cell) -> u32 {
    let mut s = 0;

    for i in 1..=9 {
        if (c & (1 << i)) == 1 {
            s += 1;
        }
    }

    s
}

fn generate_number(mut c: Cell) -> Cell {
    if (c & INVALID_MASK) == 1 {
        c = c | DIGIT_MASK;
    }
    let mut chosen = 0;
    let mut factor = -1.0;
    let mut r = rand::thread_rng();

    // Not sure if this is absolutely perfect,
    // but it works.
    for i in 1..=9 {
        if (c & (1 << i)) != 0 {
            let f = r.gen_range(0.0..=1.0);
            if f > factor {
                chosen = i;
                factor = f;
            }
        }
    }
    return (c & !DIGIT_MASK) | (1 << chosen) | (chosen << NUM_SHIFT);
}


fn main() {
    let mut sud = Sudoku::new();

    // Would be nice if there was a convenient way to randomly select
    // a cell each time. It's not really necessary tho.
    for i in 0..81 {
        sud.cells[i] = generate_number(sud.cells[i]);

        if get_number(sud.cells[i]) == 0 {
            sud.invalid_cells.push(i);
            continue;
        }

        sud.solve();
    }

    println!("{}\n", sud);

    sud.check();
}
