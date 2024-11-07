use std::ops::Range;
use std::fmt;

pub type CellIndex = usize;
pub type RowIndex = usize;
pub type ColIndex = usize;
pub type BoxIndex = usize;
pub type SecIndex = usize;

// cells, rows, cols, boxs are all 0 indexed
pub fn row_of(cell_index: CellIndex) -> RowIndex {
    cell_index / 9
}
pub fn col_of(cell_index: CellIndex) -> ColIndex {
    cell_index % 9
}
pub fn box_of(cell_index: CellIndex) -> BoxIndex {
    (cell_index / 3) % 3 + (cell_index / 27) * 3
}

// OPTIMIZE LATER (These only need to be computed once)
pub const fn of_row(row_index: RowIndex) -> [CellIndex; 9] {
    let mut r: [usize; 9] = [0; 9];

    let mut i = 0;
    while i < 9 {
        r[i] = i + (row_index * 9);
        i += 1;
    }

    r
}
pub const fn of_col(col_index: ColIndex) -> [CellIndex; 9] {
    let mut r: [usize; 9] = [0; 9];

    let mut i = 0;
    while i < 9 {
        r[i] = i * 9 + col_index;
        i += 1;
    }

    r
}
pub const fn of_box(box_index: BoxIndex) -> [CellIndex; 9] {
    let mut r: [usize; 9] = [0; 9];
    let start = (box_index % 3) * 3 + (box_index / 3) * 27;

    let mut i = 0;
    while i < 3 {
        r[i] = start + i;
        r[i+3] = start + i + 9;
        r[i+6] = start + i + 18;
        i += 1;
    }

    r
}
#[allow(dead_code, unused_variables)]
pub fn related_cells(index: CellIndex) -> [CellIndex; 21] {
    // size of returned array will be 9 + 8 + 4
    // maybe start w/ box, then add row/col?
    // It may also just be too complicated to exclude duplicates
    todo!()
}

pub const SECTION_RANGE: Range<SecIndex> = 0..27;

pub enum SectionType {
    RowSection(RowIndex),
    ColSection(ColIndex),
    BoxSection(BoxIndex),
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RowSection(i) => write!(f, "Row {}", i + 1),
            ColSection(i) => write!(f, "Col {}", i + 1),
            BoxSection(i) => write!(f, "Box {}", i + 1),
        }
    }
}

pub use SectionType::*;

pub const SECTION_INDICES: [[CellIndex; 9]; 27] = make_section_index();

pub const fn make_section_index() -> [[CellIndex; 9]; 27] {
    let mut temp = [[0; 9]; 27];

    // I love code repetition

    let mut i = 0;
    while i < 9 {
        temp[i] = of_row(i);
        i += 1;
    }
    let mut i = 0;
    while i < 9 {
        temp[i+9] = of_col(i);
        i += 1;
    }
    let mut i = 0;
    while i < 9 {
        temp[i+18] = of_box(i);
        i += 1;
    }

    temp
}

#[allow(dead_code)]
pub fn section_of(s: SectionType) -> SecIndex {
    match s {
        RowSection(i) => i,
        ColSection(i) => i + 9,
        BoxSection(i) => i + 18,
    }
}

pub fn of_section(si: SecIndex) -> SectionType {
    let i = si % 9;

    match si / 9 {
        0 => RowSection(i),
        1 => ColSection(i),
        2 => BoxSection(i),
        _ => panic!("Invalid section index {si}")
    }
}
