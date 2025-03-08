use crate::cell::Cell;

#[derive(Debug)]
pub struct HistoryEntry {
    pub name: String,
    pub cells: Vec<usize>,
    pub digits: Cell,
    pub changes: Vec<CellChange>,
}

impl HistoryEntry {
    pub fn new(n: &str, cs: Vec<usize>, ds: Cell, ch: Vec<CellChange>) -> Self {
        HistoryEntry {
            name: n.to_string(),
            cells: cs,
            digits: ds,
            changes: ch,
        }
    }

    pub fn from_solution(cell_index: usize, cell: Cell) -> Self {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct CellChange {
    pub id: usize,
    pub new_cell: Cell,
}