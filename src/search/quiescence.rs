use chess::MoveGen;

use crate::{evaluate_static, Eval, Game, SmpThread};

use super::bound::Bound;

impl<const MAIN: bool> SmpThread<'_, MAIN> {
    pub(super) fn quiescence_search(&mut self, game: &Game, mut bound: Bound) -> Eval {
        let mut best = evaluate_static(game.board());

        if best >= bound.beta { return best };
        bound.update_alpha(best);

        let mut moves = MoveGen::new_legal(game.board());
        moves.set_iterator_mask(*game.board().combined());

        for m in moves {
            let next_game = game.make_move(m);
            let eval = -self.quiescence_search(&next_game, -bound);
            self.nodes_searched += 1;

            if eval >= bound.beta { return eval };
            if eval > best {
                best = eval;
                bound.update_alpha(eval);
            }
        }

        best
    }
}
