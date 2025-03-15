use std::fmt;
use itertools::Itertools;

use crate::cell::Cell;

#[derive(Debug, PartialEq)]
pub enum EntryType {
    RsCell,
    CellSolved,
    NakedSingle,
    HiddenSingle,
    //IntersectionRemoval,
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