use anyhow::Result;
use clap::Parser;

use crate::commands::{add, commit, config, init, log, status};

mod args;
mod commands;
mod constants;
mod hasher;
mod repository;

pub fn run() -> Result<()> {
    let args = args::Cli::parse();

    match args.command {
        // rack init <path | .>
        args::Commands::Init { dir_path } => init::run(dir_path)?,
        // rack add <file | dir>
        args::Commands::Add { path } => add::run(path)?,
        // rack status
        args::Commands::Status => status::run()?,
        // rack commit -m <message>
        args::Commands::Commit { message } => commit::run(message)?,
        // rack config <key> <value>
        args::Commands::Config { key, value } => config::run(key, value)?,
        // rack log <--oneline>
        args::Commands::Log { oneline } => log::run(oneline)?,
        args::Commands::Diff => todo!(),
    }
    Ok(())
}
