mod cell;
mod index_manip;
mod sudoku;

use crate::sudoku::Sudoku;

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
