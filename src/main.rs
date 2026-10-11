use clap::Parser;

use crate::commands::new;

mod args;
mod commands;
mod constants;

fn main() {
    let args = args::Cli::parse();

    match args.command {
        args::Commands::New { flags, dir_path } => new::initialize_rev_dir(dir_path, flags),
    }
}
