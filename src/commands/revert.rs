use std::env;

use crate::{
    domain::{
        commit::{CommitEntry, CommitId, CommitParent},
        repository::Repository,
    },
    error::Result,
    storage::{baby_file, commit_file, fsutil, head_file, ignore_file, repository},
};

pub fn run(commit: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let ig = ignore_file::get_ignored(&repo)?;
    let baby = baby_file::get_baby(&repo);
    let head_file = repo.head(&baby.path_name().path());

    let commit = CommitId::new(commit)?;
    let commit_entry = commit_file::get_entry(&repo, &commit)?;
    let commit_parent = commit_entry.parent();

    let curr_head_id = head_file::get_commit(&head_file)?;
    let curr_head = commit_file::get_entry(&repo, &curr_head_id)?;

    if let CommitParent::Parent(id) = commit_parent {
        let prev_commit = commit_file::get_entry(&repo, &id)?;
        let new_msg = format!(
            "Revert \"{}\"\nThis reverts commit {}",
            commit_entry.message(),
            commit_entry.id().hash()
        );
        let new_commit = CommitEntry::new(
            CommitId::random(),
            CommitParent::Parent(curr_head.id()),
            new_msg,
            prev_commit.author(),
            prev_commit.timestamp(),
            prev_commit.idx(),
        );
        commit_file::write_commit(&repo, &new_commit, &new_commit.id(), &head_file)?;
        fsutil::materialize_commit(&new_commit.id(), &repo, &ig)?;
    } else {
        let new_msg = format!(
            "Revert \"{}\"\nThis reverts commit {}",
            commit_entry.message(),
            commit_entry.id().hash()
        );
        let new_commit = CommitEntry::new(
            CommitId::random(),
            CommitParent::Parent(curr_head.id()),
            new_msg,
            commit_entry.author(),
            commit_entry.timestamp(),
            commit_entry.idx(),
        );
        commit_file::write_commit(&repo, &new_commit, &new_commit.id(), &head_file)?;
        repository::clear_root(&repo, &ig)?;
    }
    Ok(())
}
