use rand::Rng;
use std::fmt;
use std::ops::RangeInclusive;

/* Structure:
    bit 0: if set, cell is considered to be solved
        there should only be one digit set or none
        this also controls the meaning of bits 10-13:
            if bit 0 is set, they signify the number and the count is 1
            if unset, they signify the count of digits and the number is 0
    bit 1-9: cell can have numbers 1-9
    bit 10-13: the selected number in binary,
               or the count of the digits
        this should never have a value above decimal 9
        zero means no valid digit or no selected digit
    bit 14-15: unused
*/
pub type CellSize = u16;

#[derive(Debug, Clone, Copy)]
pub struct Cell(pub CellSize);

// I actually don't know if Windows is lil-endian or big-endian.
// I also don't care.
const SOLUTION_MASK: CellSize = 0b00000000_00000001;
const DIGIT_MASK:    CellSize = 0b00000011_11111110;
const NUMBER_MASK:   CellSize = 0b00111100_00000000;
const COUNT_MASK:    CellSize = NUMBER_MASK;
const _UNUSED_MASK:  CellSize = 0b11000000_00000000;

pub const DIGIT_RANGE: RangeInclusive<CellSize> = 1..=9;
#[allow(non_snake_case)]
pub fn DIGIT(x: CellSize) -> CellSize {
    debug_assert!(
        *DIGIT_RANGE.start() <= x && x <= *DIGIT_RANGE.end(),
        "{x} is not a valid digit!"
    );
    
    1 << x
}

const NUM_SHIFT:   u16 = 10;
const COUNT_SHIFT: u16 = NUM_SHIFT;

pub const CELL_INIT: Cell = Cell(DIGIT_MASK | (9 << COUNT_SHIFT));
pub const CELL_EMPTY: Cell = Cell(0);


impl Cell {
    pub fn get_number(&self) -> CellSize {
        if self.is_solved() {
            (self.0 & NUMBER_MASK) >> NUM_SHIFT
        }
        else {
            0
        }
    }

    pub fn get_count(&self) -> CellSize {
        if self.is_solved() {
            if self.0 & DIGIT_MASK == 0 {
                0
            }
            else {
                1
            }
        }
        else {
            (self.0 & COUNT_MASK) >> COUNT_SHIFT
        }
    }

    // private
    fn set_count(&mut self, c: CellSize) {
        self.0 = (self.0 & !COUNT_MASK) | (c << COUNT_SHIFT);
    }

    pub fn has_digit(&self, digit: CellSize) -> bool {
        self.0 & DIGIT(digit) != 0
    }

    pub fn add_digit(&mut self, digit: CellSize) {
        if self.is_solved() || self.has_digit(digit) {
            return;
        }

        self.set_count(self.get_count() + 1);
        self.0 |= DIGIT(digit);
    }

    pub fn remove_digit(&mut self, digit: CellSize) {
        // assert(digit is in digit_range);
        if self.is_solved() || !self.has_digit(digit) {
            return;
        }

        self.set_count(self.get_count() - 1);
        self.0 &= !DIGIT(digit);
    }

    pub fn solve_cell(&mut self, digit: CellSize) {
        if self.is_solved() {
            return;
        }

        debug_assert!(
            digit == 0 || self.has_digit(digit),
            "digit {digit} is not available in {self}"
        );

        self.0 = ((self.0 & !DIGIT_MASK) & !COUNT_MASK)
                   | SOLUTION_MASK;

        // A cell being solved to 0 should be allowed
        if digit != 0 {
            self.0 |= DIGIT(digit) | (digit << NUM_SHIFT);
        }
    }

    pub fn is_solved(&self) -> bool {
        (self.0 & SOLUTION_MASK) != 0
    }

    #[allow(dead_code)]
    pub fn union(&self, _c: Cell) -> Cell {
        todo!()
    }

    #[allow(dead_code)]
    pub fn intersection(&self, _c: Cell) -> Cell {
        todo!()
    }

    pub fn generate_number(&mut self) {
        if self.is_solved() {
            return;
        }

        let mut chosen = 0;
        let mut factor = -1.0;
        let mut r = rand::thread_rng();

        // Not sure if this is absolutely perfect,
        // but it works.
        for i in DIGIT_RANGE {
            if self.has_digit(i) {
                let f = r.gen_range(0.0..=1.0);
                if f > factor {
                    chosen = i;
                    factor = f;
                }
            }
        }

        self.solve_cell(chosen);
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cell({:0>16b})", self.0)
    }
}