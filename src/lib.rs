use anyhow::Result;
use clap::Parser;

use crate::commands::{add, commit, config, diff, init, log, restore, status};

mod cli;
mod commands;
mod constants;
mod domain;
mod error;
mod hasher;
mod repository;
mod storage;

pub fn run() -> Result<()> {
    let args = cli::Cli::parse();

    match args.command {
        // rack init <path | .>
        cli::Commands::Init { dir_path } => init::run(dir_path)?,
        // rack add <file | dir>
        cli::Commands::Add { path } => add::run(path)?,
        // rack status
        cli::Commands::Status => status::run()?,
        // rack commit -m <message>
        cli::Commands::Commit { message } => commit::run(message)?,
        // rack config <key> <value>
        cli::Commands::Config { key, value } => config::run(key, value)?,
        // rack log <--oneline>
        cli::Commands::Log { oneline } => log::run(oneline)?,
        // rack diff
        cli::Commands::Diff { path } => diff::run(path)?,
        // rack restore <path>
        cli::Commands::Restore { path } => restore::run(path)?,
    }
    Ok(())
}
