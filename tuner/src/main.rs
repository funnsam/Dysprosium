use std::{fs::{write, File}, io::{BufRead, BufReader}, str::FromStr};

use chess::{Board, Color};
use dysprosium::{eval::weight::{sigmoid, Tracker}, EvalParams};

fn parse_result(s: &str, b: &Board) -> f64 {
    match s {
        r#""1-0";"# => (b.side_to_move() == Color::White) as u8 as f64,
        r#""1/2-1/2";"# => 0.5,
        r#""0-1";"# => (b.side_to_move() == Color::Black) as u8 as f64,
        _ => panic!("{s}"),
    }
}

fn main() {
    let pos = BufReader::new(File::open("../quiet-labeled.v7.epd").unwrap())
        .lines()
        .map(|i| {
            let i = i.unwrap();
            let (a, b) = i.split_once(" c9 ").unwrap();

            let board = Board::from_str(&format!("{a} 0 1")).unwrap();
            let result = parse_result(b, &board);

            (board, result)
        })
        .collect::<Vec<_>>();
    println!("ok");

    let ep = EvalParams::default();
    let mut best_ep = ep.clone();
    let mut best_err = f64::INFINITY;

    let k = find_k(&ep, &pos);
    println!("K = {k}");

    for i in 0..10 {
        let mut err = 0.0;

        for (board, res) in pos.iter() {
            let track = ep.evaluate_static_track(board);
            let track = Tracker::<f64>::from(track);
            err += track.backprop(*res, k);
        }

        ep.apply_backprop();
        println!("{i} {}", err / pos.len() as f64);

        if err < best_err {
            best_ep = ep.clone();
            best_err = err;
        }
    }

    let mean_err = best_err / pos.len() as f64;
    let (piece_mid, piece_end) = ep.piece_values();
    println!("{piece_mid:?} {piece_end:?}");

    let file = format!("\
// K = {k}
// Mean error = {mean_err}

use super::{{weight::Weight, EvalParams}};
impl Default for EvalParams {{ fn default() -> Self {{ {best_ep:?} }} }}");
    write("../src/eval/tuned.rs", file).unwrap();
}

fn find_k(ep: &EvalParams, pos: &[(Board, f64)]) -> f64 {
    let mut best_k = 0.0;
    let mut best_cost = f64::INFINITY;

    for k in 1..=2000 {
        let k = k as f64 / 1000.0;
        let mut cost = 0.0;

        for (board, r) in pos[..5000].iter() {
            let eval = ep.evaluate_static(board);
            let s = sigmoid(eval.0 as f64 / 100.0, k);
            let err = *r - s;
            cost += err * err / pos.len() as f64;
        }

        if cost < best_cost {
            best_k = k;
            best_cost = cost;
        }
    }

    best_k
}
