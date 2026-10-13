use clap::Parser;

use crate::commands::{add::InsertCommand, new::DeflowerCommand, status::ExposeCommand};

mod args;
mod commands;
mod constants;

fn main() {
    let args = args::Cli::parse();
    tracing_subscriber::fmt::init();

    match args.command {
        args::Commands::Deflower { flags, dir_path } => {
            DeflowerCommand::new(flags, dir_path).init();
        }
        args::Commands::Insert { path } => {
            InsertCommand::new(path).init();
        }
        args::Commands::Expose => ExposeCommand::new().init(),
    }
}
