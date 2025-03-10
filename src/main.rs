use std::time::Instant;

use simple_sudoku_generator::sudoku::Sudoku;

// Program modifiers
const AMOUNT_RUNS:   u32            = 5000;
const RUN_FUNC:      fn()           = run_once;
const CREATE_SUDOKU: fn() -> Sudoku = Sudoku::fill_incremental;


fn main() {
    let time = Instant::now();

    RUN_FUNC();
    
    println!("Program time: {:?}", time.elapsed());
}

#[allow(dead_code)]
fn run_once() {
    let sud = CREATE_SUDOKU();

    println!("{}", sud);

    sud.print_validity();
}

#[allow(dead_code)]
fn run_amount() {
    let mut failure_count = 0;

    for _ in 0..AMOUNT_RUNS {
        let sud = CREATE_SUDOKU();

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