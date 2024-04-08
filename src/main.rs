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
    cells: [Cell; 81],
    invalid_cells: Vec<usize>,
    newly_solved_cells: Vec<usize>,
    modified_cells: Vec<usize>
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku {
            cells: [DIGIT_MASK; 81],
            invalid_cells: Vec::new(),
            newly_solved_cells: Vec::new(),
            modified_cells: Vec::new(),
        }
    }
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
                (self.cells[i*9 + 0] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 1] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 2] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 3] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 4] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 5] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 6] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 7] & NUMBER_MASK) >> NUM_SHIFT,
                (self.cells[i*9 + 8] & NUMBER_MASK) >> NUM_SHIFT,
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

/* Structure:
    bit 0: cell has no valid number and should be filled randomly
    bit 1-9: cell can have numbers 1-9
    bit 10-13: the selected number in binary, zero means none
        this should never have a value above decimal 10
    bit 14-15: unused
*/
type Cell = u16;

const INVALID_MASK: Cell = 0b00000000_00000001;
const DIGIT_MASK: Cell   = 0b00000011_11111110; // Default initialization
const NUMBER_MASK: Cell  = 0b00111100_00000000;
const NUM_SHIFT: u32     = 10;

fn get_digit(c: Cell) -> u16 {
    (c & NUMBER_MASK) >> NUM_SHIFT
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
    // a cell each time. It's not really necesary tho.
    for i in 0..81 {
        sud.cells[i] = generate_number(sud.cells[i]);

        if get_digit(sud.cells[i]) == 0 {
            sud.invalid_cells.push(i);
            continue;
        }

        let remove_digit = get_digit(sud.cells[i]);

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
