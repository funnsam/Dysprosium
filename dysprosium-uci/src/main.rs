#![feature(str_split_whitespace_remainder)]

use std::io::BufRead;
use dysprosium::*;

mod client;
mod uci;

const DEFAULT_HASH_SIZE_MB: usize = 64;
const DEFAULT_THREADS: usize = 1;
const MB: usize = 1024 * 1024;

fn main() {
    println!("Dysprosium v{VERSION} licensed under GPLv3");

    let mut client = client::State::new();

    let mut args = std::env::args().skip(1);
    if let Some(a) = args.next() {
        let mut a = Some(a);

        while let Some(l) = a {
            let tokens = l.split_whitespace();
            client.handle_command(uci::parse_command(tokens));

            a = args.next();
        }

        std::process::exit(0);
    }

    let stdin = std::io::stdin().lock().lines();
    for l in stdin {
        if let Ok(l) = l {
            let tokens = l.split_whitespace();
            client.handle_command(uci::parse_command(tokens));
        }
    }
}
