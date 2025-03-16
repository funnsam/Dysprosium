use std::str::FromStr;

use chess::Board;
use dysprosium::{eval::weight::Tracker, Eval, EvalParams};

fn main() {
    let startpos = Board::from_str("8/8/8/8/8/8/P7/K6k w - - 0 1").unwrap();
    let ep = EvalParams::default();
    let track = ep.evaluate_static_track(&startpos);
    let track = Tracker::<f64>::from(track);
    track.backprop(0.0, 0.01);
}
