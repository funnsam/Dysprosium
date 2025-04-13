use super::*;

impl<const MAIN: bool> SmpThread<'_, MAIN> {
    #[inline]
    pub(super) fn quiescence_search(&mut self, game: &Game, bound: Bound) -> Eval {
        self._quiescence_search(game, bound).0
    }

    pub(super) fn _quiescence_search(&mut self, game: &Game, mut bound: Bound) -> (Eval, NodeType) {
        let in_check = game.board().checkers().0 != 0;

        let mut best = if in_check { Eval::MIN } else {
            let sp = evaluate_static(game.board());

            if sp >= bound.beta {
                return (sp, NodeType::Cut);
            }

            // delta pruning on hopeless nodes
            #[cfg(feature = "qs-big-delta")]
            if sp + 1100 < bound.alpha {
                return (bound.alpha, NodeType::None);
            }

            bound.alpha = bound.alpha.max(sp);

            sp
        };
        let sp = best;

        let moves = MoveGen::new_legal(game.board());

        for m in moves {
            if !in_check {
                if game.is_quiet(m) { continue };

                // delta pruning
                let capt = game.board().piece_on(m.get_dest()).unwrap_or(Piece::Queen);
                #[cfg(feature = "qs-delta")]
                if sp + PIECE_VALUE[capt.to_index()] + 200 < bound.alpha { continue };
            }

            #[cfg(feature = "qs-see")]
            if see(game, m) < 0 { continue };

            let game = game.make_move(m);
            let eval = -self.quiescence_search(&game, -bound);
            self.nodes_searched += 1;

            if eval > best {
                best = eval;
                bound.alpha = bound.alpha.max(eval);
            }

            if eval >= bound.beta {
                return (eval, NodeType::Cut);
            }
        }

        (best, if best != bound.alpha { NodeType::All } else { NodeType::Pv })
    }
}
