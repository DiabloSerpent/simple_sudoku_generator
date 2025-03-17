use std::fmt;
use itertools::Itertools;

use crate::cell::Cell;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EntryType {
    RsCell,
    CellSolved,
    NakedSingle,
    HiddenSingle,
    // IntersectionRemoval,
    PointedGroup,
    BoxLineReduction,
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


impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HistoryEntry: {:?}",  self.name)?;
        write!(f,   "    cells: {:?}",     self.cells)?;
        writeln!(f, "    digits: {:0>9b}", self.digits.get_digits())?;
        write!(f,   "    changes: [{}]",   self.changes.iter().format(", "))
    }
}

impl fmt::Display for CellChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.new_cell)
    }
}