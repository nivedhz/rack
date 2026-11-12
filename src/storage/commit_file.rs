use std::{fs, path::Path};

use crate::{
    domain::{
        commit::{CommitEntry, CommitId},
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub fn get_entry(repo: &Repository, commit: &CommitId) -> Result<CommitEntry> {
    let commit_content =
        fsutil::read_file_to_string(&repo.commit_entry_file(commit)).unwrap_or_default();

    CommitEntry::parse(commit_content, commit.hash())
}

pub fn write_commit(
    repo: &Repository,
    commit: &CommitEntry,
    id: &CommitId,
    head_file: &Path,
) -> Result<()> {
    fs::create_dir(repo.commit_entry_dir(id))?;
    fsutil::write_atomic(&repo.commit_entry_file(id), &commit.serialize())?;
    fsutil::write_atomic(head_file, id.hash().as_bytes())?;

    Ok(())
}
