use std::ops::{Index, IndexMut};

use chess::ChessMove;

#[derive(Debug, Clone)]
pub struct HistoryTable([[i32; 64]; 64]);

impl Default for HistoryTable {
    fn default() -> Self { Self::new() }
}

impl Index<ChessMove> for HistoryTable {
    type Output = i32;

    fn index(&self, i: ChessMove) -> &Self::Output {
        &self.0[i.get_source().to_index()][i.get_dest().to_index()]
    }
}

impl IndexMut<ChessMove> for HistoryTable {
    fn index_mut(&mut self, i: ChessMove) -> &mut Self::Output {
        &mut self.0[i.get_source().to_index()][i.get_dest().to_index()]
    }
}

impl HistoryTable {
    pub const fn new() -> Self {
        Self([[0; 64]; 64])
    }

    pub fn add_bonus(&mut self, mov: ChessMove, depth: usize) {
        self[mov] += depth as i32 * depth as i32;
    }
}
