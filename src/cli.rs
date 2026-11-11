use clap::{Parser, Subcommand};
use std::path::PathBuf;

const LONG_ABOUT: &str = "A dirtier version of git which is sure to get you fired.";
#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initilalize a new rack repository
    Init {
        #[arg(default_value = ".")]
        dir_path: PathBuf,
    },
    /// Add file to the rack
    Add { path: Vec<PathBuf> },
    /// View the current status of the rack repository
    Status,
    /// Commit the added files
    Commit {
        #[arg(short, long)]
        message: String,
    },
    /// Set a config value
    Config { key: String, value: String },
    /// Logs the commits
    Log {
        #[arg(long)]
        oneline: bool,
    },
    /// Show the diff
    Diff {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Restore a file
    Restore {
        #[arg(long)]
        staged: bool,
        #[arg(long)]
        commit: Option<String>,
        path: PathBuf,
    },
    /// List all branches or create a branch
    Branch { name: Option<String> },
    /// Switch to another branch
    Switch {
        branch: String,
        #[arg(short, long)]
        create: bool,
    },
}
