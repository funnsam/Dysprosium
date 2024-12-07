use crate::{*, eval::*, trans_table::*};

impl Engine {
    pub fn best_move_iter_deep(&mut self) -> (chess::ChessMove, Eval) {
        self.time_ref = Instant::now();
        self.reserve_time();

        let mut prev = self.best_move(1);

        for depth in 2.. {
            let this = self.best_move(depth);
            if self.times_up() { break; }

            prev = this;
        }

        prev
    }

    fn best_move(&self, depth: usize) -> (chess::ChessMove, Eval) {
        let mut alpha = EVAL_MIN;
        let mut best = (chess::ChessMove::default(), EVAL_MIN - 1);

        for m in chess::MoveGen::new_legal(self.game.board()) {
            let game = self.game.make_move(m);
            let (neg_eval, _nt) = self.evaluate_search(&game, depth - 1, EVAL_MIN, -alpha);
            let eval = -neg_eval;
            if depth != 1 && self.times_up() { break; }

            if eval > best.1 {
                best = (m, eval);
                alpha = alpha.max(eval);
            }
        }

        best
    }

    /// Perform an alpha-beta (fail-soft) negamax search and return the evaluation
    pub fn evaluate_search(
        &self,
        game: &Game,
        depth: usize,
        mut alpha: Eval,
        beta: Eval,
    ) -> (Eval, NodeType) {
        if let Some(trans) = self.trans_table.get(game.board().get_hash()) {
            if trans.depth as usize >= depth && (trans.node_type == NodeType::Exact
                || (trans.node_type == NodeType::LowerBound && trans.eval >= beta)
                || (trans.node_type == NodeType::UpperBound && trans.eval < alpha)) {
                return (trans.eval, NodeType::None);
            }
        }

        if self.times_up() {
            return (0, NodeType::None);
        }

        if game.can_declare_draw() {
            return (0, NodeType::Exact);
        }

        if depth == 0 {
            return (evaluate_static(game.board()), NodeType::Exact);
        }

        let mut best = EVAL_MIN;

        for m in chess::MoveGen::new_legal(game.board()) {
            let game = game.make_move(m);
            let (neg_eval, nt) = self.evaluate_search(&game, depth - 1, -beta, -alpha);
            if self.times_up() { return (best, NodeType::None); }

            if nt != NodeType::None {
                self.trans_table.insert(game.board().get_hash(), TransTableEntry {
                    depth: (depth - 1) as u8,
                    eval: neg_eval,
                    node_type: nt,
                });
            }

            let eval = -neg_eval;

            if eval > best {
                best = eval;
                alpha = alpha.max(eval);
            }
            if eval >= beta {
                return (best, NodeType::LowerBound);
            }
        }

        (best, if best == alpha { NodeType::UpperBound } else { NodeType::Exact })
    }
}
