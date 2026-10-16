use crate::constants::LONG_ABOUT;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initilalize a new rack repository
    Deflower {
        #[arg(default_value = ".")]
        dir_path: PathBuf,
        #[command(flatten)]
        flags: NewFlags,
    },
    /// Add file to the rack
    Insert { path: PathBuf },
    /// View the current status of the rack repository
    Expose,
    /// Commit the added files
    Bang {
        #[arg(short, long)]
        message: String,
    },
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct NewFlags {
    #[arg(short, long)]
    pub default: bool,
}
