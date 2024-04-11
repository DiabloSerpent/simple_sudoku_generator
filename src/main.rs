use rand::Rng;
use std::fmt;

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
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku {
            cells: [DIGIT_MASK; 81],
            invalid_cells: Vec::new(),
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

        let mut finished = false;
        while !finished {
            for rule in Self::RULE_ORDER {
                if rule(self) {
                    break;
                }
            }
            finished = true;
        }
    }

    // Each rule returns true if sudoku was modified,
    // false otherwise.
    const RULE_ORDER: [fn(&mut Sudoku) -> bool; 3] = [
        Sudoku::cell_solved,
        Sudoku::naked_single,
        Sudoku::hidden_single,
    ];

    fn cell_solved(&mut self) -> bool {
        false
    }

    fn naked_single(&mut self) -> bool {false}
    fn hidden_single(&mut self) -> bool {false}
}

// It's just, so PEAK
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    // size of returned array will be 9 + 8 + 4;
    [0; 21]
}

// If I removed the invalid bit, it would be possible
// to also store the digit count of the cell within
// the cell.
// It would just be a lil inconvenient to access
// and also would need to be updated manually.
// to accomplish: remove invalid digit, shift digit/number masks right by 1
//      then, let bits 13-15 be the digit count.
//      bit values of 1-7 will mean a count of 2-8
//      bit value of 0 will mean 0, 1, or 9
//      to differentiate them:
//          count of 0: cell & digit_mask == 0
//          count of 9: cell & digit_mask == digit_mask
//          count of 1: cell & digit_mask != 0, != digit_mask

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
const NUM_SHIFT: u32     = 10;

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

        let remove_digit = get_number(sud.cells[i]);

        let (row, col, sbox) = (row_of(i), col_of(i), box_of(i));

        for j in 0..81 {
            let (jr, jc, jb) = (row_of(j), col_of(j), box_of(j));

            if row == jr || col == jc || sbox == jb {
                sud.cells[j] = sud.cells[j] & !(1 << remove_digit);
            }
        }
    }

    println!("{}", sud);
}
