use std::time::Instant;

use simple_sudoku_generator::sudoku::Sudoku;


fn main() {
    let time = Instant::now();

    run_once();
    
    println!("Program time: {:?}", time.elapsed());
}

fn run_once() {
    let sud = Sudoku::fill_random();

    println!("{}", sud);

    sud.print_validity();
}

#[allow(dead_code, unused_variables)]
fn run_amount(n: u32) {
    for _ in 0..n {
        let sud = Sudoku::fill_random();
        sud.print_on_invalid_state();
    }
}

#[allow(dead_code, unused_variables)]
fn run_until_failure(n: u32) {
    for _ in 0..n {
        let sud = Sudoku::fill_random();
        if !sud.is_valid() {
            break;
        }
    }

    sud.print_validity();
}