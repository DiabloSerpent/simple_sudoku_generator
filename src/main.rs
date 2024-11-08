use std::time::Instant;

use simple_sudoku_generator::sudoku::Sudoku;


fn main() {
    let time = Instant::now();

    run_once();
    
    println!("Program time: {:?}", time.elapsed());
}

fn create_sudoku() -> Sudoku {
    Sudoku::fill_incremental()
}

#[allow(dead_code)]
fn run_once() {
    let sud = create_sudoku();

    println!("{}", sud);

    sud.print_validity();
}

// 1000 is usually a good number to pick
#[allow(dead_code, unused_variables)]
fn run_amount(amt: u32) {
    let mut failure_count = 0;

    for _ in 0..amt {
        let sud = create_sudoku();

        if !sud.is_valid() {
            sud.print_invalid_cells();
            failure_count += 1;
        }
    }

    println!("Failure Count: {failure_count}");
}

#[allow(dead_code, unused_variables)]
fn run_until_failure(upper_bound: u32) {
    let mut have_failure = false;
    let mut success_count = 0;

    for _ in 0..upper_bound {
        let sud = create_sudoku();

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