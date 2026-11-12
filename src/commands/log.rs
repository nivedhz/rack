use std::env;

use crate::{
    domain::{
        commit::{CommitEntry, CommitId},
        repository::Repository,
    },
    error::{Error, Result},
    storage::{baby_file, commit_file, fsutil},
};

pub fn run() -> Result<Vec<CommitEntry>> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let baby = baby_file::get_baby(&repo);

    if !repo.head(&baby.path_name().path()).exists() {
        return Err(Error::NoCommitsFound {
            message: String::from("use rack commit to add commits!"),
        });
    }
    let mut curr_head = fsutil::read_file_to_string(&repo.head(&baby.path_name().path()))?;

    let mut entries: Vec<CommitEntry> = Vec::new();

    while !curr_head.trim().is_empty() {
        let curr_head_id = CommitId::new(curr_head)?;

        let commit_entry = commit_file::get_entry(&repo, &curr_head_id)?;

        curr_head = commit_entry.parent().get_hash();
        entries.push(commit_entry);
    }

    Ok(entries)
}
