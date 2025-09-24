use once_cell::sync::Lazy;

pub mod config;

use config::Config;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::init());
