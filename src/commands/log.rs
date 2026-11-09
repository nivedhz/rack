use std::env;

use crate::{
    domain::{
        commit::{CommitEntry, CommitId},
        repository::Repository,
    },
    error::{Error, Result},
    storage::fsutil,
};

pub fn run() -> Result<Vec<CommitEntry>> {
    let repo = Repository::discover(&env::current_dir()?)?;
    if !repo.head_file().exists() {
        return Err(Error::NoCommitsFound {
            message: String::from("use rack commit to add commits!"),
        });
    }
    let mut curr_head = fsutil::read_file_to_string(&repo.head_file())?;

    let mut entries: Vec<CommitEntry> = Vec::new();

    while !curr_head.trim().is_empty() {
        let curr_head_id = CommitId::new(curr_head);

        let commit_content = fsutil::read_file_to_string(&repo.commit_entry_file(&curr_head_id))?;
        let commit_entry = CommitEntry::parse(commit_content, curr_head_id.hash())?;

        curr_head = commit_entry.parent().get_hash();
        entries.push(commit_entry);
    }

    Ok(entries)
}
