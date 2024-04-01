use rand::Rng;

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
    let mut chosen = 0;
    let mut factor = 0.0;
    let mut r = rand::thread_rng();

    for i in 1..(9+1) {
        println!("{}", c & (1 << i));
        if (c & (1 << i)) != 0 {
            let f = r.gen_range(0.0..0.1);
            if f > factor {
                chosen = i;
                factor = f;
            }
        }
    }
    return chosen;
}

fn main() {
    println!("\nHello, sudoku!\n");
    let coolidea: Cell = 16;
    let _sud = Sudoku { cells: [0; 81] };
    println!("\n{}", generate_number(coolidea));
}
