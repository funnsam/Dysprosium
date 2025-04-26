use chess::{ChessMove, Piece};

use crate::{eval::PIECE_VALUE, trans_table::TransTableEntry, Game, SmpThread};

impl<const MAIN: bool> SmpThread<'_, MAIN> {
    pub(super) fn move_score(&self, game: &Game, tt: Option<TransTableEntry>, m: ChessMove) -> i32 {
        if tt.is_some_and(|tt| tt.next == m) {
            return i32::MAX;
        }

        if game.is_capture(m) {
            return mvv_lva(game, m);
        }

        0
    }
}

fn mvv_lva(game: &Game, m: ChessMove) -> i32 {
    let victim = game.board().piece_on(m.get_dest()).unwrap();
    let aggressor = game.board().piece_on(m.get_source()).unwrap();

    PIECE_VALUE[victim.to_index()] as i32 * PIECE_VALUE[Piece::Queen.to_index()] as i32
        - PIECE_VALUE[aggressor.to_index()] as i32
}

#[test]
fn mvv_lva_ord() {
    use std::str::FromStr;
    use chess::Square;

    let game = Game::from_str("7k/8/8/2r1p3/3P4/2R2p2/8/7K w - - 0 1").unwrap();

    let d4e5 = ChessMove::new(Square::D4, Square::E5, None);
    let d4c5 = ChessMove::new(Square::D4, Square::C5, None);
    let c3f3 = ChessMove::new(Square::C3, Square::F3, None);
    let c3c5 = ChessMove::new(Square::C3, Square::C5, None);

    let pxp = mvv_lva(&game, d4e5);
    let pxr = mvv_lva(&game, d4c5);
    let rxp = mvv_lva(&game, c3f3);
    let rxr = mvv_lva(&game, c3c5);

    assert!(pxp < pxr);
    assert!(rxp < rxr);
    assert!(rxr < pxr);
    assert!(rxp < pxp);
}
