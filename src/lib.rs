use anyhow::Result;
use clap::Parser;

use crate::commands::{add, commit, config, new, status};

mod args;
mod commands;
mod constants;
mod hasher;
mod repository;

pub fn run() -> Result<()> {
    let args = args::Cli::parse();

    match args.command {
        // rack deflower <path | .>
        args::Commands::Deflower { dir_path } => new::run(dir_path)?,
        // rack insert <file | dir>
        args::Commands::Insert { path } => add::run(path)?,
        // rack expose
        args::Commands::Expose => status::run()?,
        // rack bang -m <message>
        args::Commands::Bang { message } => commit::run(message)?,
        args::Commands::Config { key, value } => config::run(key, value)?,
    }
    Ok(())
}
