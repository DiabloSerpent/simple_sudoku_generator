use std::fmt;
use itertools::Itertools;

use crate::cell::Cell;

#[derive(Debug, PartialEq)]
pub enum EntryType {
    RsCell,
    CellSolved,
    NakedSingle,
    HiddenSingle,
    IntersectionRemoval,
    NakedGroup,
    HiddenGroup,
}


#[derive(Debug)]
pub struct HistoryEntry {
    pub name: EntryType,
    pub cells: Vec<usize>,
    pub digits: Cell,
    pub changes: Vec<CellChange>,
}

#[derive(Debug, Clone, Copy)]
pub struct CellChange {
    pub id: usize,
    pub new_cell: Cell,
}


impl HistoryEntry {
    pub fn new(n: EntryType, cs: Vec<usize>, ds: Cell, ch: Vec<CellChange>) -> Self {
        Self {
            name: n,
            cells: cs,
            digits: ds,
            changes: ch,
        }
    }

    pub fn from_solution(n: EntryType, cell_index: usize, cell: Cell) -> Self {
        Self::new(
            n,
            vec![cell_index],
            cell.get_unsolved_copy().inverse(),
            vec![CellChange {id: cell_index, new_cell: cell}])
    }
}

impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HistoryEntry: {:?}\n", self.name)?;
        write!(f, "    cells: {:?}", self.cells)?;
        write!(f, "    digits: {:0>9b}\n", self.digits.get_digits())?;
        write!(f, "    changes: [{}]", self.changes.iter().format(", "))
    }
}

impl fmt::Display for CellChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.new_cell)
    }
}