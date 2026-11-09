use anyhow::Result;
use clap::Parser;

use crate::commands::{add, commit, config, diff, init, log, restore, status};

mod cli;
mod commands;
mod domain;
mod error;
mod output;
mod storage;

pub fn run() -> Result<()> {
    let args = cli::Cli::parse();

    match args.command {
        // rack init <path | .>
        cli::Commands::Init { dir_path } => init::run(dir_path)?,
        // rack add <file | dir>
        cli::Commands::Add { path } => add::run(path)?,
        // rack status
        cli::Commands::Status => {
            let status_res = status::run()?;
            output::status(status_res)?;
        }
        // rack commit -m <message>
        cli::Commands::Commit { message } => commit::run(message)?,
        // rack config <key> <value>
        cli::Commands::Config { key, value } => config::run(key, value)?,
        // rack log <--oneline>
        cli::Commands::Log { oneline } => {
            let log_res = log::run()?;
            output::log(oneline, log_res)?;
        }
        // rack diff
        cli::Commands::Diff { path } => {
            let diff_res = diff::run(path)?;
            output::diff(diff_res)?;
        }
        // rack restore <path>
        cli::Commands::Restore { path } => restore::run(path)?,
    }
    Ok(())
}
