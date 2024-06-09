use std::time::Instant;

use simple_sudoku_generator::sudoku::Sudoku;


fn main() {
    let time = Instant::now();

    let sud = Sudoku::fill_random();

    println!("{}", sud);

    sud.check();
    
    println!("Program time: {:?}", time.elapsed());
}
