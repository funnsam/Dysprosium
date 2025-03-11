#![warn(clippy::future_not_send)]

use core::str::FromStr;
use std::sync::{atomic::*, Arc, RwLock};
use api::{move_from_uci, Challenge, Direction, Event, GameEvent, GameState, LichessApi, Player, Variant};
use chess::{Board, BoardStatus, Color};
use config::Config;
use dysprosium::Game;

mod api;
mod config;
mod log;

pub struct LichessClient {
    api: LichessApi,
    pub config: RwLock<Config>,
    pub active_games: AtomicUsize,
}

impl LichessClient {
    pub fn new(api: LichessApi) -> Self {
        Self {
            api,
            config: RwLock::default(),
            active_games: AtomicUsize::new(0),
        }
    }

    pub fn listen(self: Arc<Self>) {
        Arc::clone(&self).listen_config();

        self.api.listen(|event| match event {
            Event::Challenge { challenge: Challenge { direction, id, challenger: Player { name: Some(challenger), .. }, variant: Variant { key: variant }, speed, rated } } => {
                if direction == Some(Direction::Out) { return };

                let config = self.config();

                let is_su = config.superusers.iter().any(|i| i == challenger);

                info!("user `{challenger}` challenged bot (id: `{id}`, variant: {variant:?}, time control: {speed:?}, rated: {rated})");

                let max_games = config.max_games.unwrap_or(usize::MAX);
                if !is_su && self.active_games.load(Ordering::Relaxed) >= max_games {
                    self.api.decline_challenge(id, "later");
                } else if !is_su && variant != "standard" {
                    self.api.decline_challenge(id, "standard");
                } else if !is_su && config.tc_blacklist.contains(&speed) {
                    self.api.decline_challenge(id, "timeControl");
                } else if !is_su && !config.allow_rated && rated {
                    self.api.decline_challenge(id, "casual");
                } else if !is_su && !config.allow_casual && !rated {
                    self.api.decline_challenge(id, "rated");
                } else {
                    self.api.accept_challenge(id);
                }
            },
            Event::GameStart { game: api::Game { id, color, fen, opponent, .. } } => {
                let game = dysprosium::Game::from_str(fen).unwrap();
                self.active_games.fetch_add(1, Ordering::Relaxed);

                info!("started a game with `{}` (id: `{id}`, fen: `{fen}`)", opponent.username.unwrap());

                let arc = Arc::clone(&self);
                let id = id.to_string();
                std::thread::spawn(move || arc.play_game(id, game, color.0));
            },
            Event::GameFinish { .. } => {
                self.active_games.fetch_sub(1, Ordering::Relaxed);
            },
            _ => dbg!("{event:?}"),
        });
    }

    fn play_game(self: Arc<Self>, game_id: String, game: dysprosium::Game, color: Color) {
        let mut engine = dysprosium::Engine::new(game, 64 * 1024 * 1024);
        engine.start_smp(self.config().threads_per_game - 1);

        self.api.listen_game(&game_id, |event| match event {
            GameEvent::GameFull { initial_fen, state } => {
                let mut game = engine.game.write();
                *game = Game::new(Board::from_str(initial_fen).unwrap_or_default());
                for m in state.moves.split_whitespace() {
                    *game = game.make_move(move_from_uci(m));
                }

                if game.board().side_to_move() == color {
                    drop(game);
                    self.play(&game_id, color, state, &mut engine);
                }
            },
            GameEvent::GameState { state } => {
                let mut game = engine.game.write();
                if let Some(m) = state.moves.split_whitespace().last() {
                    *game = game.make_move(move_from_uci(m));
                }

                if game.board().side_to_move() == color {
                    drop(game);
                    self.play(&game_id, color, state, &mut engine);
                }
            },
            _ => dbg!("{event:?}"),
        });

        info!("stream ended (id: `{}`)", game_id);
    }

    fn play(&self, game_id: &str, color: Color, state: GameState<'_>, engine: &mut dysprosium::Engine) {
        {
            let game = engine.game.read();
            if game.can_declare_draw() || game.board().status() != BoardStatus::Ongoing {
                return;
            }
        }

        engine.time_control(None, match color {
            Color::White => dysprosium::TimeControl {
                time_left: state.wtime,
                time_incr: state.winc,
            },
            Color::Black => dysprosium::TimeControl {
                time_left: state.btime,
                time_incr: state.binc,
            },
        });

        let (next, _, _) = engine.best_move(|engine, (best, eval, depth)| {
            let nodes = engine.nodes();
            let time = engine.elapsed().as_secs_f64();

            info!(
                "searched {nodes} nodes at {depth}-ply deep in {time:.2}s ({:.2} MN/s), PV: {} ({eval})",
                nodes as f64 / time / 1_000_000.0,
                engine.find_pv(best, 20).into_iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            );
            true
        });
        self.api.send_move(game_id, next);
    }
}

fn main() {
    let api_key = std::fs::read_to_string("api_key.txt").unwrap().trim().to_string();
    Arc::new(LichessClient::new(LichessApi::new(api_key))).listen();
}
