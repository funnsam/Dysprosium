use crate::{evaluate_static, Eval, Game, SmpThread};

use super::bound::Bound;

impl<const MAIN: bool> SmpThread<'_, MAIN> {
    pub(super) fn quiescence_search(&mut self, game: &Game, mut bound: Bound) -> Eval {
        self.nodes_searched += 1;

        let mut best = evaluate_static(game.board());

        if best >= bound.beta { return best };
        bound.update_alpha(best);

        let mut moves = game.board().pseudo_legal_captures(&[]);

        for m in moves {
            let next_game = game.make_move(m);
            if next_game.board().is_illegal() { continue };

            let eval = -self.quiescence_search(&next_game, -bound);

            if eval >= bound.beta { return eval };
            if eval > best {
                best = eval;
                bound.update_alpha(eval);
            }
        }

        best
    }
}
