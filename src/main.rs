struct Sudoku {
    cells: [Cell; 81],
}

/* Structure:
    bit 0: invalid bit
    bit 1-9: can have numbers 1-9
    bit 10-15: unused
*/
type Cell = i16;

fn generate_number(c: Cell) -> i32 {
    for i in 1..9 {
        println!("{}", c & (1 << i));
    }
    return 0;
}

fn main() {
    println!("\nHello, sudoku!\n");
    let coolidea: Cell = 16;
    let _sud = Sudoku { cells: [0; 81] };
    generate_number(coolidea);
}
