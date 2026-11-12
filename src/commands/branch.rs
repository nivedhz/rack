use std::env;

use crate::{
    domain::{branch::BranchIndexEntry, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    output,
    storage::{baby_file, branch_index, fsutil, head_file},
};

pub fn run(name: Option<String>) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let baby = baby_file::get_baby(&repo);

    let head_commit = head_file::get_commit(&repo.head(&baby.path_name().path()))?;

    let branch_idx = branch_index::get_entries(&repo)?;

    if let Some(n) = name {
        let name = RepoPath::new(n);
        if repo.branch(&name.path()).exists() {
            return Err(Error::BranchExists {
                name: name.to_string(),
            });
        }

        let branch_entry = BranchIndexEntry::new(head_commit, name);
        fsutil::write_atomic(
            &repo.branch(&branch_entry.name().path()),
            &branch_entry.head().serialize(),
        )?;
        fsutil::append_atomic(&repo.branch_index(), &branch_entry.serialize())?;
        return Ok(());
    }

    output::branch(baby, branch_idx);

    Ok(())
}
