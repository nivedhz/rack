use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        #[command(flatten)]
        flags: NewFlags,
    },
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct NewFlags {
    #[arg(short, long)]
    pub default: bool,
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}
