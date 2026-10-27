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
    Seed {
        #[arg(default_value = ".")]
        dir_path: PathBuf,
    },
    /// Add file to the rack
    Insert { path: PathBuf },
    /// View the current status of the rack repository
    Flash,
    /// Commit the added files
    Bust {
        #[arg(short, long)]
        message: String,
    },
    /// Set a config value
    Kinks { key: String, value: String },
    /// Logs the commits
    Stains {
        #[arg(long)]
        oneline: bool,
    },
}
