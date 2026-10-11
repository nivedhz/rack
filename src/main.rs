use clap::Parser;

use crate::commands::init;

mod args;
mod commands;

fn main() {
    let args = args::Cli::parse();

    match args.command {
        args::Commands::Init { dir_path } => init::initialize_rev_dir(dir_path),
    }
}
