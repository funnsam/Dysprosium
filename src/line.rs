use core::cell::OnceCell;

use chess::{Board, ChessMove};

use crate::{Eval, EvalParams};

#[derive(Debug, Clone)]
pub struct PrevMove<'a> {
    pub mov: ChessMove,
    pub static_eval: EvalCell<'a>,
    pub prev_move: Option<&'a Self>,
}

#[derive(Debug, Clone)]
pub struct EvalCell<'a> {
    board: &'a Board,
    eval: OnceCell<Eval>,
}

impl PrevMove<'_> {
    pub fn n_plies_ago(&self, n: usize) -> Option<&Self> {
        let mut at = self;

        for _ in 0..n {
            at = at.prev_move?;
        }

        Some(at)
    }

    pub fn is_improving(&self, ep: &EvalParams) -> bool {
        self.n_plies_ago(2).map_or(true, |m| {
            m.static_eval.get(ep) < self.static_eval.get(ep)
        })
    }
}

impl<'a> EvalCell<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            eval: OnceCell::new(),
        }
    }
}

impl EvalCell<'_> {
    pub fn get(&self, ep: &EvalParams) -> Eval {
        *self.eval.get_or_init(|| ep.evaluate_static(self.board))
    }
}
