use anyhow::Result;
use clap::Parser;

use crate::commands::{add, commit, config, log, new, status};

mod args;
mod commands;
mod constants;
mod hasher;
mod repository;

pub fn run() -> Result<()> {
    let args = args::Cli::parse();

    match args.command {
        // rack init <path | .>
        args::Commands::Seed { dir_path } => new::run(dir_path)?,
        // rack add <file | dir>
        args::Commands::Insert { path } => add::run(path)?,
        // rack status
        args::Commands::Flash => status::run()?,
        // rack commit -m <message>
        args::Commands::Bust { message } => commit::run(message)?,
        // rack config <key> <value>
        args::Commands::Kinks { key, value } => config::run(key, value)?,
        // rack log <--oneline>
        args::Commands::Stains { oneline } => log::run(oneline)?,
    }
    Ok(())
}
