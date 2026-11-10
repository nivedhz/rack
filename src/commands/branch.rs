use colored::Colorize;
use std::env;

use crate::{
    domain::{repo_path::RepoPath, repository::Repository},
    error::Result,
    storage::fsutil,
};

// TODO: refactor and cleanup, implement the needed data structures.
pub fn run(name: Option<String>) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let baby = fsutil::read_file_to_string(&repo.baby())?;
    let curr_head = baby.split_once(" ").unwrap_or_default().1.to_string();
    let curr_head = RepoPath::new(repo.rack_dir().join(curr_head.trim()));
    let head = fsutil::read_file_to_string(&curr_head.path()).unwrap_or_default();

    let branches: Vec<String> = fsutil::read_file_to_string(&repo.branch_index())?
        .lines()
        .map(|l| l.split_once(" ").unwrap_or_default().1.to_string())
        .collect();

    if let Some(name) = name {
        fsutil::write_atomic(&repo.head_dir().join(&name), head.as_bytes())?;
        fsutil::append_atomic(
            &repo.branch_index(),
            format!("{} {}\n", head, name).as_bytes(),
        )?;
    } else {
        for branch in branches {
            if RepoPath::new(&branch) == curr_head.strip_prefix(&repo.head_dir())? {
                print!("* {}    ", branch.green());
            } else {
                print!("{}    ", branch);
            }
        }
        println!();
    }

    Ok(())
}
