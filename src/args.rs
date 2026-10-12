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
        #[arg(default_value = ".")]
        dir_path: PathBuf,
        #[command(flatten)]
        flags: NewFlags,
    },
    Add {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct NewFlags {
    #[arg(short, long)]
    pub default: bool,
}
