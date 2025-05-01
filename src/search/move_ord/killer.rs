use std::ops::{Index, IndexMut};

use chess::{ChessMove, Square};

#[derive(Clone)]
pub struct KillerTable([KillerEntry; 256]);

#[derive(Clone, Copy)]
pub struct KillerEntry(ChessMove);

impl KillerTable {
    pub const fn new() -> Self {
        Self([const { KillerEntry::new() }; 256])
    }
}

impl Index<usize> for KillerTable {
    type Output = KillerEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for KillerTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl KillerEntry {
    pub const fn new() -> Self {
        Self(ChessMove::new(Square::A1, Square::A1, None))
    }

    pub fn contains(&self, mov: ChessMove) -> bool {
        self.0 == mov
    }

    pub fn push(&mut self, mov: ChessMove) {
        self.0 = mov;
    }
}
