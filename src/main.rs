use std::time::Instant;

use simple_sudoku_generator::sudoku::Sudoku;
use simple_sudoku_generator::history::EntryType;

// Program modifiers
const AMOUNT_RUNS:   u32            = 5000;
const RUN_FUNC:      fn()           = run_once;
const CREATE_SUDOKU: fn() -> Sudoku = Sudoku::fill_incremental;
const PRINT_HISTORY: bool           = false;

// Controls which history entries will be displayed
#[allow(non_snake_case)]
const fn DISPLAY_ENTRY_TYPE(he: EntryType) -> bool {
    use EntryType::*; match he {
        RsCell           => true,
        CellSolved       => true,
        NakedSingle      => true,
        HiddenSingle     => true,
        PointedGroup     => true,
        BoxLineReduction => true,
        NakedGroup       => true,
        HiddenGroup      => true,
    }
}


fn main() {
    let time = Instant::now();

    RUN_FUNC();
    
    println!("Program time: {:?}", time.elapsed());
}

#[allow(dead_code)]
fn run_once() {
    let sud = CREATE_SUDOKU();

    print_history(&sud);

    println!("{}", sud);

    sud.print_validity();
}

#[allow(dead_code)]
fn run_amount() {
    let mut failure_count = 0;

    for _ in 0..AMOUNT_RUNS {
        let sud = CREATE_SUDOKU();

        print_history(&sud);

        if !sud.is_valid() {
            sud.print_invalid_cells();
            failure_count += 1;
        }
    }

    println!("Failure Count: {failure_count}");
}

#[allow(dead_code)]
fn run_until_failure() {
    let mut have_failure = false;
    let mut success_count = 0;

    for _ in 0..AMOUNT_RUNS {
        let sud = CREATE_SUDOKU();

        if !sud.is_valid() {
            print_history(&sud);
            println!("{sud}");
            sud.print_invalid_cells();
            have_failure = true;
            break;
        }
        else {
            success_count += 1;
        }
    }

    if have_failure {
        println!("Successes until failure: {success_count}");
    }
    else {
        println!("No invalid state found");
    }
}

fn print_history(sud: &Sudoku) {
    if !PRINT_HISTORY {
        return;
    }

    let mut new_sud = Sudoku::new();

    println!("{new_sud}");

    for h in &sud.history {
        for cc in &h.changes {
            new_sud.cells[cc.id] = cc.new_cell;
        }

        if DISPLAY_ENTRY_TYPE(h.name) {
            println!("{h}\n{new_sud:?}");
        }
    }
}