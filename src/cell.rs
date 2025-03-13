use rand::Rng;
use std::fmt;
use std::ops::RangeInclusive;

/* Structure:
    bit 0: if set, cell is considered to be solved
        there should only be one digit set or none
        this also controls the meaning of bits 10-13:
            if bit 0 is set, they signify the number and the count is 1 or 0
            if unset, they signify the count of digits and the number is 0
    bit 1-9: cell can have numbers 1-9
    bit 10-13: the selected number in binary,
               or the count of the digits
        this should never have a value above decimal 9,
            except for 0b1111, which means the count is to be ignored.
                This only applies when the solved bit is not set.
        zero means no valid digit or no selected digit
    bit 14-15: unused
*/
pub type CellSize = u16;

#[derive(Debug, Clone, Copy)]
pub struct Cell(CellSize);

// I actually don't know if Windows is lil-endian or big-endian.
// I also don't care.
const SOLUTION_MASK: CellSize = 0b00000000_00000001;
const DIGIT_MASK:    CellSize = 0b00000011_11111110;
const NUMBER_MASK:   CellSize = 0b00111100_00000000;
const COUNT_MASK:    CellSize = NUMBER_MASK;
const _UNUSED_MASK:  CellSize = 0b11000000_00000000;

pub const DIGIT_RANGE: RangeInclusive<CellSize> = 1..=9;
#[allow(non_snake_case)]
fn DIGIT(x: CellSize) -> CellSize {
    debug_assert!(
        *DIGIT_RANGE.start() <= x && x <= *DIGIT_RANGE.end(),
        "{x} is not a valid digit!"
    );
    
    1 << x
}

const NUM_SHIFT:    u16 = 10;
const COUNT_SHIFT:  u16 = NUM_SHIFT;
const IGNORE_COUNT: u16 = COUNT_MASK;

pub const CELL_INIT:  Cell = Cell(DIGIT_MASK | (9 << COUNT_SHIFT));
pub const CELL_EMPTY: Cell = Cell(0);
pub const CELL_ACC:   Cell = Cell(IGNORE_COUNT);


impl Cell {
    pub fn get_number(&self) -> CellSize {
        if self.is_solved() {
            // Add check for value?
            (self.0 & NUMBER_MASK) >> NUM_SHIFT
        }
        else {
            0
        }
    }

    // This only checks if count is off. It makes no guarentees as to
    // the validity of count beyond it being not off.
    pub fn count_is_off(&self) -> bool {
        self.0 & COUNT_MASK == IGNORE_COUNT
    }

    pub fn turn_count_off(&mut self) {
        // This kinda relies on IGNORE_COUNT being all 1s but whatever
        self.0 |= IGNORE_COUNT;
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
            // Add check for value?
            debug_assert!(!self.count_is_off(), "Count should be on");
            (self.0 & COUNT_MASK) >> COUNT_SHIFT
        }
    }

    // private
    fn set_count(&mut self, c: CellSize) {
        // Does not check if count is off b/c I don't want to make reset_count
        // more complicated.
        
        debug_assert!(!self.is_solved(), "solved cell can't set count");
        // According to rust compiler, c must be >= 0 b/c of its type
        // so checking it is pointless. However, it should be noted
        // that c should be >= 0.
        debug_assert!(c <= 9);

        self.0 = (self.0 & !COUNT_MASK) | (c << COUNT_SHIFT);
    }

    pub fn reset_count(&mut self) {
        // Cell should be unsolved

        let mut c = 0;
        for d in DIGIT_RANGE {
            if self.has_digit(d) {
                c += 1;
            }
        }

        self.set_count(c);
    }

    // In set terms, the complement
    pub fn inverse(&self) -> Cell {
        debug_assert!(!self.is_solved(), "Can't take inverse of solved cell");

        let mut c = Cell(!self.0 & DIGIT_MASK);

        if self.count_is_off() {
            Cell(c.0 | IGNORE_COUNT)
        }
        else {
            c.set_count(9 - c.get_count());
            c
        }
    }

    pub fn has_digit(&self, digit: CellSize) -> bool {
        self.0 & DIGIT(digit) != 0
    }

    pub fn get_digits(&self) -> CellSize {
        (self.0 & DIGIT_MASK) >> 1
    }

    pub fn add_digit(&mut self, digit: CellSize) {
        if self.is_solved() || self.has_digit(digit) {
            return;
        }

        if !self.count_is_off() {
            self.set_count(self.get_count() + 1);
        }
        self.0 |= DIGIT(digit);
    }

    pub fn remove_digit(&mut self, digit: CellSize) -> bool {
        if self.is_solved() || !self.has_digit(digit) {
            return false;
        }

        if !self.count_is_off() {
            self.set_count(self.get_count() - 1);
        }
        self.0 &= !DIGIT(digit);

        true
    }

    pub fn remove_digits(&mut self, other: Cell) -> bool {
        self.intersect_with(other.inverse())
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

    pub fn get_unsolved_copy(&self) -> Cell {
        let mut c = Cell(self.0);

        if !self.is_solved() {
            return c;
        }

        c.0 &= !SOLUTION_MASK & !NUMBER_MASK;
        c.set_count(1);

        c
    }

    pub fn has_intersection(&self, other: Cell) -> bool {
        (self.0 & other.0) & DIGIT_MASK != 0
    }

    pub fn union(&self, other: Cell) -> Cell {
        // Maybe this shouldn't panic
        debug_assert!(!self.is_solved() && !other.is_solved(),
            "Can't apply union to solved cell");

        let mut c = Cell(self.0 | (other.0 & DIGIT_MASK));

        if c.0 != self.0 && !c.count_is_off() {
            c.reset_count();
        }

        c
    }

    pub fn intersect(&self, other: Cell) -> Cell {
        // Maybe this shouldn't panic
        debug_assert!(!self.is_solved() && !other.is_solved(),
            "Can't apply intersection to solved cell");

        let mut c = Cell(self.0 & (other.0 | !DIGIT_MASK));

        if c.0 != self.0 && !c.count_is_off() {
            c.reset_count();
        }

        c
    }

    pub fn xor(&self, other: Cell) -> Cell {
        debug_assert!(!self.is_solved() && !other.is_solved(),
            "Can't apply intersection to solved cell");
        
        let mut c = Cell(self.0 ^ (other.0 & DIGIT_MASK));

        if c.0 != self.0 && !c.count_is_off() {
            c.reset_count();
        }

        c
    }

    pub fn union_with(&mut self, other: Cell) -> bool {
        let nc = self.union(other);

        let r = self.0 != nc.0;

        self.0 = nc.0;

        r
    }

    pub fn intersect_with(&mut self, other: Cell) -> bool {
        let nc = self.intersect(other);

        let r = self.0 != nc.0;

        self.0 = nc.0;

        r
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
        write!(f, "Cell({:0>14b})", self.0)
    }
}