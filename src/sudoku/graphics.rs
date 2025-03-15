use std::fmt;

use crate::Sudoku;
use crate::cell::{DIGIT_RANGE, Cell, CellSize};
use crate::index_manip::*;

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

/*
╔════╤════╤════╦════╤════╤════╦════╤════╤════╗
║  0 │  1 │  2 ║  3 │  4 │  5 ║  6 │  7 │  8 ║
╟────┼────┼────╫────┼────┼────╫────┼────┼────╢
║  9 │ 10 │ 11 ║ 12 │ 13 │ 14 ║ 15 │ 16 │ 17 ║
╟────┼────┼────╫────┼────┼────╫────┼────┼────╢
║ 18 │ 19 │ 20 ║ 21 │ 22 │ 23 ║ 24 │ 25 │ 26 ║
╠════╪════╪════╬════╪════╪════╬════╪════╪════╣
║ 27 │ 28 │ 29 ║ 30 │ 31 │ 32 ║ 33 │ 34 │ 35 ║
╟────┼────┼────╫────┼────┼────╫────┼────┼────╢
║ 36 │ 37 │ 38 ║ 39 │ 40 │ 41 ║ 42 │ 43 │ 44 ║
╟────┼────┼────╫────┼────┼────╫────┼────┼────╢
║ 45 │ 46 │ 47 ║ 48 │ 49 │ 50 ║ 51 │ 52 │ 53 ║
╠════╪════╪════╬════╪════╪════╬════╪════╪════╣
║ 54 │ 55 │ 56 ║ 57 │ 58 │ 59 ║ 60 │ 61 │ 62 ║
╟────┼────┼────╫────┼────┼────╫────┼────┼────╢
║ 63 │ 64 │ 65 ║ 66 │ 67 │ 68 ║ 69 │ 70 │ 71 ║
╟────┼────┼────╫────┼────┼────╫────┼────┼────╢
║ 72 │ 73 │ 74 ║ 75 │ 76 │ 77 ║ 78 │ 79 │ 80 ║
╚════╧════╧════╩════╧════╧════╩════╧════╧════╝
*/


/*
╔═══════╤═══════╤═══════╦═══════╤═══════╤═══════╦═══════╤═══════╤═══════╗
║ 1 2 3 │ 1 2 3 │ 1 2 3 ║ 1 2 3 │ 1 2 3 │ 1 2 3 ║ 1 2 3 │ 1 2 3 │ 1 2 3 ║
║ 4 5 6 │ 4 5 6 │ 4 5 6 ║ 4 5 6 │ 4 5 6 │ 4 5 6 ║ 4 5 6 │ 4 5 6 │ 4 5 6 ║
║ 7 8 9 │ 7 8 9 │ 7 8 9 ║ 7 8 9 │ 7 8 9 │ 7 8 9 ║ 7 8 9 │ 7 8 9 │ 7 8 9 ║
╟─────┼─────┼─────╫─────┼─────┼─────╫─────┼─────┼─────╢
║123│123│123║123│123│123║123│123│123║
║456│456│456║456│456│456║456│456│456║
║789│789│789║789│789│789║789│789│789║
╟───┼───┼───╫───┼───┼───╫───┼───┼───╢
║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║
║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║
║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║
╠═════╪═════╪═════╬═════╪═════╪═════╬═════╪═════╪═════╣
║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║
║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║
║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║
╟─────┼─────┼─────╫─────┼─────┼─────╫─────┼─────┼─────╢
║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║
║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║
║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║
╟─────┼─────┼─────╫─────┼─────┼─────╫─────┼─────┼─────╢
║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║
║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║
║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║
╠═════╪═════╪═════╬═════╪═════╪═════╬═════╪═════╪═════╣
║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║
║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║
║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║
╟─────┼─────┼─────╫─────┼─────┼─────╫─────┼─────┼─────╢
║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║
║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║
║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║
╟─────┼─────┼─────╫─────┼─────┼─────╫─────┼─────┼─────╢
║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║1 2 3│1 2 3│1 2 3║
║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║4 5 6│4 5 6│4 5 6║
║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║7 8 9│7 8 9│7 8 9║
╚═════╧═════╧═════╩═════╧═════╧═════╩═════╧═════╧═════╝
*/

impl Sudoku {
    pub fn print_on_invalid_state(&self) {
        if !self.is_valid() {
            self.print_invalid_cells();
        }
    }

    pub fn print_validity(&self) {
        if self.is_valid() {
            println!("No invalid cells");
        }
        else {
            self.print_invalid_cells();
        }
    }

    pub fn print_invalid_cells(&self) {
        debug_assert!(self.is_solved(), "only solved sudoku can be invalid");

        let sec_stat = self.get_section_status();

        println!("Invalid solutions:");

        println!("       | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |");

        for si in SECTION_RANGE {
            if si % 9 == 0 {
                println!("       |---|---|---|---|---|---|---|---|---|");
            }

            let sums = self.section_digit_sum[si];

            if sec_stat[si] {
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


struct CellBigDisplay(Cell);

impl CellBigDisplay {
    fn get_row(&self, i: CellSize) -> String {
        if self.0.is_solved() {
            if i == 1 {
                format!("  <{}> ", self.0.get_number())
            }
            else {
                "      ".to_string()
            }
        }
        else {
            format!(" {} {} {}", self.num_or_space(i*3 + 1),
                                 self.num_or_space(i*3 + 2),
                                 self.num_or_space(i*3 + 3))
        }
    }

    fn num_or_space(&self, i: CellSize) -> String {
        if self.0.has_digit(i) {
            format!("{i}")
        }
        else {
            " ".to_string()
        }
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

                    let cell = CellBigDisplay(self.cells[ri*9 + ci]);

                    write!(f, "{}", cell.get_row(digit_row))?;
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