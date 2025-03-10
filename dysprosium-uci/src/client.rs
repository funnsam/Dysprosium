use std::time::Duration;

use crate::*;

use dysprosium::Engine;

fn print_uci_info() {
    println!("id name dysprosium v{VERSION}");
    println!("id author funnsam");
    println!("option name Hash type spin default {DEFAULT_HASH_SIZE_MB} min 1 max 16384");
    println!("option name Threads type spin default {DEFAULT_THREADS} min 1 max 256");
}

pub struct State {
    engine: Engine,
    debug_mode: bool,
}

impl State {
    pub fn new() -> Self {
        let mut engine = Engine::new(Game::new(chess::Board::default()), DEFAULT_HASH_SIZE_MB * MB);
        engine.start_smp(DEFAULT_THREADS - 1);

        Self {
            engine,
            debug_mode: false,
        }
    }

    pub fn handle_command<'a>(&mut self, command: Option<uci::UciCommand<'a>>) {
        match command {
            Some(uci::UciCommand::Uci) => {
                print_uci_info();
                println!("uciok");
            },
            Some(uci::UciCommand::SetOption(name, value)) => match name.to_ascii_lowercase().as_str() {
                "hash" => self.engine.resize_hash(value.unwrap().parse::<usize>().unwrap() * MB),
                "threads" => {
                    self.engine.kill_smp();
                    self.engine.start_smp(value.unwrap().parse::<usize>().unwrap() - 1);
                },
                _ => println!("info string got invalid setoption"),
            },
            Some(uci::UciCommand::Debug(d)) => self.debug_mode = d,
            Some(uci::UciCommand::IsReady) => println!("readyok"),
            Some(uci::UciCommand::Quit) => std::process::exit(0),
            Some(uci::UciCommand::UciNewGame) => {},
            Some(uci::UciCommand::Position { mut position, moves }) => {
                for m in moves {
                    position = position.make_move(m);
                }

                *self.engine.game.write() = position;
            },
            Some(uci::UciCommand::Move(m)) => {
                *self.engine.game.write() = self.engine.game.read().make_move(m)
            },
            Some(uci::UciCommand::Go { depth: target_depth, movetime, wtime, btime, movestogo }) => {
                let tc = if matches!(self.engine.game.read().board().side_to_move(), chess::Color::White) {
                    wtime
                } else {
                    btime
                };
                if let Some(mt) = movetime {
                    self.engine.allow_for(mt);
                } else if let Some(tc) = tc {
                    self.engine.time_control(movestogo, tc);
                } else {
                    self.engine.allow_for(Duration::MAX);
                }

                let mov = self.best_move(target_depth);
                if self.debug_mode {
                    let full = 1000 * self.engine.tt_used() / self.engine.tt_size();

                    println!("info hashfull {full}");
                    self.engine.dump_debug();
                }
                println!("bestmove {mov}");
            },
            Some(uci::UciCommand::D) => print!("{:#}", self.engine.game.read()),
            Some(uci::UciCommand::Eval) => println!(
                "{:#}Eval: {}",
                self.engine.game.read(),
                evaluate_static(self.engine.game.read().board()),
            ),
            Some(uci::UciCommand::Bench) => {
                self.engine.allow_for(Duration::MAX);
                self.engine.best_move(|_, (_, _, depth)| depth < 16);

                let nodes = self.engine.nodes();
                let elapsed = self.engine.elapsed();
                let nps = (nodes as f64 / elapsed.as_secs_f64()).round();

                println!("{nodes} nodes {nps} nps");
            },
            None => {},
        }
    }

    fn best_move(&mut self, target_depth: Option<usize>) -> chess::ChessMove {
        self.engine.best_move(|engine, (best, eval, depth)| {
            let time = engine.elapsed();
            let nodes = engine.nodes();

            println!(
                "info score {eval:#} depth {depth} nodes {nodes} time {} nps {} pv {}",
                time.as_millis(),
                (nodes as f64 / time.as_secs_f64()) as u64,
                engine.find_pv(best, if self.debug_mode { 100 } else { 20 }).into_iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            );
            target_depth.map_or(true, |td| td > depth)
        }).0
    }
}
