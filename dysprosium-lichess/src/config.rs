use std::{path::Path, sync::{Arc, RwLockReadGuard}};

use notify::{PollWatcher, RecursiveMode, Watcher};
use serde::Deserialize;

use crate::{api::Speed, info, LichessClient};

const CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default = "_false")]
    pub allow_rated: bool,
    #[serde(default = "_true")]
    pub allow_casual: bool,
    #[serde(default)]
    pub tc_blacklist: Vec<Speed>,
    #[serde(default)]
    pub superusers: Vec<String>,

    #[serde(default = "_1")]
    pub threads_per_game: usize,
    #[serde(default)]
    pub max_games: Option<usize>,
}

fn _true() -> bool { true }
fn _false() -> bool { false }
fn _1() -> usize { 1 }

impl Default for Config {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

pub fn load_config() -> Config {
    let src = std::fs::read_to_string(CONFIG_PATH).unwrap_or_else(|_| String::new());
    toml::from_str(&src).unwrap()
}

impl LichessClient {
    pub fn config(&self) -> RwLockReadGuard<Config> {
        self.config.read().unwrap()
    }

    pub fn listen_config(self: Arc<Self>) {
        *self.config.write().unwrap() = load_config();

        std::thread::spawn(move || {
            let mut watcher = PollWatcher::new(move |_| {
                *self.config.write().unwrap() = load_config();
                info!("config reloaded");
            }, notify::Config::default()).unwrap();

            let path = Path::new(CONFIG_PATH);
            if watcher.watch(&path, RecursiveMode::NonRecursive).is_err() {
                return;
            }

            loop {
                core::hint::spin_loop();
            }
        });
    }
}
