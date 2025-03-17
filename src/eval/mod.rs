use chess::*;
use weight::{Tracker, WeightCell};

pub use eval::Eval;

#[cfg(feature = "tuned")]
pub use tuned::{PIECE_VALUE_MID, PIECE_VALUE_END};
#[cfg(feature = "pesto")]
pub use pesto::{PIECE_VALUE_MID, PIECE_VALUE_END};

mod eval;
#[cfg(feature = "tuned")]
mod tuned;
#[cfg(feature = "pesto")]
mod pesto;
pub mod weight;

#[derive(Debug, Clone)]
pub struct EvalParams {
    pst_mid: [WeightCell; 6 * 64],
    pst_end: [WeightCell; 6 * 64],

    rook_open_file_bonus: WeightCell,
    unshield_king_penalty: WeightCell,
}

impl EvalParams {
    #[cfg(feature = "eval-track")]
    pub fn apply_backprop(&self) {
        fn apply(wc: &WeightCell) {
            let mut w = wc.get();

            if w.frequency() != 0 {
                w.value -= (w.derivative() * 1500.0 / w.frequency() as f64).round() as i16;
            }

            w.reset_meta();
            wc.set(w);
        }

        for i in self.pst_mid.iter() { apply(i) };
        for i in self.pst_end.iter() { apply(i) };

        apply(&self.rook_open_file_bonus);
        apply(&self.unshield_king_penalty);
    }

    pub fn piece_values(&self) -> ([i64; 6], [i64; 6]) {
        fn compute(pst: &[WeightCell; 6 * 64]) -> [i64; 6] {
            (
                *pst.chunks_exact(64)
                    .map(|i| i.iter().map(|j| j.get().value as i64).sum::<i64>() / 64)
                    .collect::<arrayvec::ArrayVec<i64, 6>>()
            ).try_into().unwrap()
        }

        (
            compute(&self.pst_mid),
            compute(&self.pst_end),
        )
    }

    /// Mostly PeSTO's evaluation with rook on open file bonus
    pub fn evaluate_static(&self, board: &Board) -> Eval {
        let track = self.evaluate_static_track(board);
        Eval(track.value() as i16)
    }

    pub fn evaluate_static_track(&self, board: &Board) -> Tracker<i32> {
        let mut mid_game = [Tracker::default(), Tracker::default()];
        let mut end_game = [Tracker::default(), Tracker::default()];
        let mut phase = 0;

        for square in board.combined().into_iter() {
            // SAFETY: only squares with things on it are checked
            let piece = unsafe { board.piece_on(square).unwrap_unchecked() };
            let color = unsafe { board.color_on(square).unwrap_unchecked() };

            let mid = &mut mid_game[color.to_index()];
            let end = &mut end_game[color.to_index()];

            // rook on open file bonus
            if piece == Piece::Rook
                && (board.pieces(Piece::Pawn) & chess::get_file(square.get_file())).0 == 0
            {
                *mid += &self.rook_open_file_bonus;
                *end += &self.rook_open_file_bonus;
            }

            // king's pawn shield penalty
            // TODO: maybe make it a king-side dependent PST instead?
            if piece == Piece::King {
                let king_center = square.uforward(color);
                let king_pawns = (board.pieces(Piece::Pawn) & (chess::get_king_moves(king_center) | BitBoard::from_square(king_center))).popcnt();

                *mid += (&self.unshield_king_penalty, 3_i16.saturating_sub(king_pawns as i16));
            }

            let idx = (square.to_index() ^ (0b111_000 * (color == Color::Black) as usize)) | (piece.to_index() << 6);
            *mid += &self.pst_mid[idx];
            *end += &self.pst_end[idx];
            phase += PIECE_PHASE[piece.to_index()];
        }

        let stm = board.side_to_move() as usize;
        let mg_eval = mid_game[stm].clone() - mid_game[1 - stm].clone();
        let eg_eval = end_game[stm].clone() - end_game[1 - stm].clone();
        let mg_phase = phase.min(24);
        let eg_phase = 24 - mg_phase;

        let mut eval = Tracker::from(mg_eval) * mg_phase as i32
            + Tracker::from(eg_eval) * eg_phase as i32;
        eval /= 24;

        eval
    }
}

/// Finds the current phase of the game. 0 is endgame and 24 is midgame.
pub fn game_phase(board: &Board) -> u8 {
    board.combined()
        .into_iter()
        .map(|sq| PIECE_PHASE[unsafe { board.piece_on(sq).unwrap_unchecked() }.to_index()])
        .sum::<u8>()
        .min(24)
}

pub const PIECE_VALUE: [i16; 6] = PIECE_VALUE_MID;
const PIECE_PHASE: [u8; 6] = [0, 1, 1, 2, 4, 0];
