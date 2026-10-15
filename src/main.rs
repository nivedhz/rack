use clap::Parser;

use crate::commands::{add::InsertCommand, new::DeflowerCommand, status::ExposeCommand};

mod args;
mod commands;
mod constants;

fn main() {
    let args = args::Cli::parse();

    match args.command {
        args::Commands::Deflower { flags, dir_path } => {
            DeflowerCommand::new(flags, dir_path).run();
        }
        args::Commands::Insert { path } => {
            InsertCommand::new(path).run();
        }
        args::Commands::Expose => ExposeCommand::new().run(),
    }
}
