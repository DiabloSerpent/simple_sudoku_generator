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

pub const SECTION_START:     usize = 0;
pub const SECTION_ROW_START: usize = SECTION_START;
#[allow(dead_code)]
pub const SECTION_ROW_END:   usize = SECTION_COL_START;
pub const SECTION_COL_START: usize = 9;
#[allow(dead_code)]
pub const SECTION_COL_END:   usize = SECTION_BOX_START;
#[allow(dead_code)]
pub const SECTION_BOX_START: usize = 18;
#[allow(dead_code)]
pub const SECTION_BOX_END:   usize = SECTION_END;
pub const SECTION_END:       usize = 27;
pub const SECTION_RANGE: Range<SecIndex> = SECTION_START..SECTION_END;

pub enum SectionType {
    Row(RowIndex),
    Col(ColIndex),
    Box(BoxIndex),
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SectionType::Row(i) => write!(f, "Row {}", i + 1),
            SectionType::Col(i) => write!(f, "Col {}", i + 1),
            SectionType::Box(i) => write!(f, "Box {}", i + 1),
        }
    }
}

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
        SectionType::Row(i) => i,
        SectionType::Col(i) => i + 9,
        SectionType::Box(i) => i + 18,
    }
}

pub fn of_section(si: SecIndex) -> SectionType {
    let i = si % 9;

    match si / 9 {
        0 => SectionType::Row(i),
        1 => SectionType::Col(i),
        2 => SectionType::Box(i),
        _ => panic!("Invalid section index {si}")
    }
}
