use anyhow::Result;
use clap::Parser;

use crate::commands::{add, commit, new, status};

mod args;
mod commands;
mod constants;
mod hasher;
mod repository;

pub fn run() -> Result<()> {
    let args = args::Cli::parse();

    match args.command {
        args::Commands::Deflower { dir_path } => new::run(dir_path)?,
        args::Commands::Insert { path } => add::run(path)?,
        args::Commands::Expose => status::run()?,
        args::Commands::Bang { message } => commit::run(message)?,
    }
    Ok(())
}
