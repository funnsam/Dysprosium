use std::{io::BufRead, str::FromStr};

use chess::{BitBoard, Board, Color, Piece, ALL_PIECES, ALL_SQUARES};
use chessbot2::eval::{is_open_file, is_semi_open_file, EvalParamList, MAX_PHASE};

const ALPHA: f64 = 0.01;
const K: f64 = 0.4;
const BATCH: usize = 50000;

fn main() {
    let mut eval_f64 = EvalParamList::<f64>::default();
    let mut eval_params = eval_f64.round_into_i16();

    let lines = std::io::stdin().lock().lines();
    let pos = lines.filter_map(|l| {
        let l = l.ok()?;
        let (fen, r) = l.split_once(',')?;
        let (_, eval) = r.split_once(',')?;

        Some((Board::from_str(fen).ok()?, eval.parse::<f64>().ok()?))
    }).collect::<Vec<_>>();
    let train = &pos[500..];
    let test = &pos[..500];

    println!("{} positions loaded", pos.len());

    for iteration in 0..500 {
        let mut eval_collector = EvalParamList::zeroed();
        let mut eval_frequency = EvalParamList::zeroed();

        for _ in 0..BATCH {
            let (board, r) = &train[fastrand::usize(..train.len())];
            let (mg, eg, w) = eval_params.get_separated_in_white(board);
            let eval = eval_params.evaluate_with((mg, eg, w)).0 as f64 / 100.0;

            let s = sigmoid(eval);
            let d_eval = (2.0 * (s - *r)) * (K * s * (1.0 - s));

            let d_mid = d_eval * (w as f64 / MAX_PHASE as f64);
            let d_end = d_eval * (1.0 - w as f64 / MAX_PHASE as f64);

            for square in board.combined().into_iter() {
                // SAFETY: only squares with things on it are checked
                let piece = unsafe { board.piece_on(square).unwrap_unchecked() };
                let color = unsafe { board.color_on(square).unwrap_unchecked() };

                let p = (color, piece, square);
                let c = if color == Color::White { 1.0 } else { -1.0 };
                eval_collector.pst_mid[p] += 100.0 * d_mid * c;
                eval_collector.pst_end[p] += 100.0 * d_end * c;
                eval_frequency.pst_mid[p] += 1.0;
                eval_frequency.pst_end[p] += 1.0;

                // rook has open file
                if piece == Piece::Rook && is_open_file(board, square.get_file()) {
                    eval_collector.rook_open_file_bonus += 100.0 * d_eval * c;
                    eval_frequency.rook_open_file_bonus += 1.0;
                }

                if piece == Piece::King {
                    let mut open_files = 0;
                    if let Some(sq) = square.left() {
                        open_files += is_semi_open_file(board, color, sq.get_file()) as i16;
                    }
                    if let Some(sq) = square.right() {
                        open_files += is_semi_open_file(board, color, sq.get_file()) as i16;
                    }
                    eval_collector.king_open_file_penalty += 100.0 * d_mid * open_files as f64 * c;
                    eval_frequency.king_open_file_penalty += open_files.min(1) as f64;

                    let king_center = square.uforward(color);
                    let king_pawns = (board.pieces(Piece::Pawn) & (chess::get_king_moves(king_center) | BitBoard::from_square(king_center))).popcnt();
                    eval_collector.king_pawn_penalty += 100.0 * d_mid * 3_u32.saturating_sub(king_pawns) as f64 * c;
                    eval_frequency.king_pawn_penalty += king_pawns.min(1) as f64;
                }
            }
        }

        for piece in ALL_PIECES {
            let (mut mg, mut eg) = (0.0, 0.0);

            for square in ALL_SQUARES {
                let p = (Color::White, piece, square);
                eval_f64.pst_mid[p] -= ALPHA * eval_collector.pst_mid[p] / eval_frequency.pst_mid[p].max(1.0);
                eval_f64.pst_end[p] -= ALPHA * eval_collector.pst_end[p] / eval_frequency.pst_end[p].max(1.0);

                mg += eval_f64.pst_mid[p];
                eg += eval_f64.pst_end[p];
            }

            println!("{piece:?}: {:.02} {:.02}", mg / 64.0, eg / 64.0);
        }

        eval_f64.rook_open_file_bonus   -= ALPHA * eval_collector.rook_open_file_bonus   / eval_frequency.rook_open_file_bonus.max(1.0);
        eval_f64.king_pawn_penalty      -= ALPHA * eval_collector.king_pawn_penalty      / eval_frequency.king_pawn_penalty.max(1.0);
        eval_f64.king_open_file_penalty -= ALPHA * eval_collector.king_open_file_penalty / eval_frequency.king_open_file_penalty.max(1.0);

        eval_params = eval_f64.round_into_i16();

        let mut cost = 0.0;
        for (board, r) in test.iter() {
            let eval = eval_params.evaluate_with(eval_params.get_separated_in_white(board));
            let s = sigmoid(eval.0 as f64 / 100.0);
            let err = *r - s;
            cost += err * err / 500.0;
        }
        println!("{iteration} {cost}");
    }

    std::fs::write("../src/eval_params.bin", postcard::to_stdvec(&eval_params).unwrap()).unwrap();
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-K * x).exp())
}
