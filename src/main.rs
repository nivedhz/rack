use clap::Parser;

use crate::commands::new::NewCommand;

mod args;
mod commands;
mod constants;

fn main() {
    let args = args::Cli::parse();
    tracing_subscriber::fmt::init();

    match args.command {
        args::Commands::New { flags, dir_path } => {
            NewCommand::new(flags, dir_path).init();
        }
    }
}
