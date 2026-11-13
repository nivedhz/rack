use clap::{Parser, Subcommand};
use std::path::PathBuf;

const LONG_ABOUT: &str = "A VCS, but in Rust. So automatically 1000x better than Git.";
#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create an empty repository or reinitialize an existing one
    Init {
        #[arg(default_value = ".")]
        dir_path: PathBuf,
    },
    /// Add the file to staging area
    Add { path: Vec<PathBuf> },
    /// Show the working tree status
    Status,
    /// Record changes to the repository
    Commit {
        #[arg(short, long)]
        message: String,
    },
    /// Set repository config options
    Config { key: String, value: String },
    /// Show commit logs
    Log {
        #[arg(long)]
        oneline: bool,
    },
    /// Show changes between HEAD and current state
    Diff {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Restore working tree files
    Restore {
        #[arg(long)]
        staged: bool,
        #[arg(long)]
        commit: Option<String>,
        path: PathBuf,
    },
    /// List, create, or delete branches
    Branch {
        name: Option<String>,
        #[arg(short, long)]
        delete: Option<String>,
    },
    /// Switch to another branch
    Switch {
        branch: String,
        #[arg(short, long)]
        create: bool,
    },
}
