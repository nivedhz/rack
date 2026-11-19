use anyhow::Result;
use clap::Parser;

use crate::commands::{
    add, branch, commit, config, diff, init, log, mv, restore, rm, show, status, switch,
};

mod cli;
mod commands;
mod domain;
mod error;
mod output;
mod storage;

pub fn run() -> Result<()> {
    let args = cli::Cli::parse();

    match args.command {
        // rack init <path>
        cli::Commands::Init { dir_path } => init::run(dir_path)?,
        // rack add <path>
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
        // rack log |--oneline|
        cli::Commands::Log { oneline } => {
            let log_res = log::run()?;

            output::log(oneline, log_res)?;
        }
        // rack diff |<path>| |--cached|
        cli::Commands::Diff { path, cached } => {
            let diff_res = diff::run(path, cached)?;

            output::diff(diff_res)?;
        }
        // rack restore <path> |--staged | --commit|
        cli::Commands::Restore {
            path,
            staged,
            commit,
        } => restore::run(path, staged, commit)?,
        // rack branch |-d| |<name>|
        cli::Commands::Branch { name, delete } => branch::run(name, delete)?,
        // rack switch |-c| <branch>
        cli::Commands::Switch { branch, create } => switch::run(branch, create)?,
        // rack rm <path> |--cached|
        cli::Commands::Rm { path, cached } => rm::run(path, cached)?,
        // rack mv <src> <dst>
        cli::Commands::Mv { src, dst } => mv::run(src, dst)?,
        // rack show |commit|
        cli::Commands::Show { commit } => {
            let show_res = show::run(commit)?;

            output::log(false, vec![show_res.log])?;
            output::diff(show_res.diff)?;
        }
    }
    Ok(())
}
