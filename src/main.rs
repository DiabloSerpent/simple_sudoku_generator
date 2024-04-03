use rand::Rng;

// Shamelessly ripped from:
// https://codegolf.stackexchange.com/questions/126930/draw-a-sudoku-board-using-line-drawing-characters

const sudoku_board: &str = "\
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

struct Sudoku {
    cells: [Cell; 81],
}

/* Structure:
    bit 0: invalid bit
    bit 1-9: cell can have numbers 1-9
    bit 10-15: unused
*/
type Cell = i16;

const DIGIT_MASK: Cell = 0b00000011_11111110;
const INVALID_MASK: Cell = 0b00000000_0000001;

fn generate_number(mut c: Cell) -> Cell {
    if (c & INVALID_MASK) == 1 {
        c = c | DIGIT_MASK;
    }
    let mut chosen = 0;
    let mut factor = -1.0;
    let mut r = rand::thread_rng();

    for i in 1..(9+1) {
        print!("{}", i);
        if (c & (1 << i)) != 0 {
            let f = r.gen_range(0.0..1.0);
            print!(" {:.2}", f);
            if f > factor {
                chosen = i;
                factor = f;
            }
        }
        println!("");
    }
    return (c & !DIGIT_MASK) | (1 << chosen);
}

fn main() {
    println!("\nHello, sudoku!\n");
    let coolidea: Cell = DIGIT_MASK;
    let _sud = Sudoku { cells: [0; 81] };
    println!("\n{:b}", generate_number(coolidea));
}
