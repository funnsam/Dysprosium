use core::cell::LazyCell;
use std::{cell::OnceCell, ops::Deref};

use chess::{Board, ChessMove};

use crate::{evaluate_static, Eval};

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

    pub fn is_improving(&self) -> bool {
        self.n_plies_ago(2).map_or(true, |m| {
            *m.static_eval < *self.static_eval
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

impl Deref for EvalCell<'_> {
    type Target = Eval;

    fn deref(&self) -> &Self::Target {
        self.eval.get_or_init(|| evaluate_static(self.board))
    }
}
