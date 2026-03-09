use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

use crate::daemon::{start_daemon, status_daemon, stop_daemon};

mod daemon;
mod highlighter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the highlighter daemon
    Start,

    /// Stop the highlighter daemon
    Stop,

    /// Check whether the highlighter daemon is running
    Status,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// For performance reasons, highlighting is disabled for very long lines.
    /// This option specifies maximum length of a line (in bytes) up to which
    /// highlighting is applied.
    max_line_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_line_length: 20000,
        }
    }
}

fn main() -> Result<()> {
    let home = PathBuf::from(env::var("HOME").context("$HOME not set")?);
    let config_dir = home.join(".config/zsh-patina");
    let data_dir = home.join(".local/share/zsh-patina");

    // parse arguments
    let args = Args::parse();

    // load config file
    let config_file_path = config_dir.join("config.toml");
    let config = if fs::exists(&config_file_path)? {
        Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(&config_file_path))
            .extract()
            .with_context(|| format!("Unable to read config file {config_file_path:?}"))?
    } else {
        Config::default()
    };

    match args.command {
        Command::Start => start_daemon(&data_dir, &config),
        Command::Stop => stop_daemon(&data_dir),
        Command::Status => status_daemon(&data_dir),
    }
}
