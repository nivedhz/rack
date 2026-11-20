use std::env;

use crate::{
    domain::{branch::BranchEntry, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{baby_file, branch_index, fsutil, head_file, ignore_file},
};

pub fn run(branch: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let baby = baby_file::get_baby(&repo);
    let branch_idx = branch_index::get_entries(&repo)?;
    let ig = ignore_file::get_ignored(&repo)?;

    if branch.trim().is_empty() {
        return Err(Error::InvalidBranch { name: branch });
    }

    let branch = RepoPath::new(branch);
    if !repo.branch(&branch.path()).exists() {
        return Err(Error::BranchNotExists {
            name: branch.to_string(),
        });
    }
    if baby.name() == branch {
        return Err(Error::BranchInUse {
            name: branch.to_string(),
        });
    }

    let curr_head = head_file::get_commit(&repo.branch(&baby.name().path()))?;

    let branch_head = head_file::get_commit(&repo.branch(&branch.path()))?;
    let branch_anc = branch_idx.get(BranchEntry::new(branch)).unwrap_or_default();

    if curr_head == branch_anc {
        fsutil::write_atomic(&repo.branch(&baby.name().path()), &branch_head.serialize())?;
        fsutil::materialize_commit(&branch_head, &repo, &ig)?;
        return Ok(());
    }
    println!("performing tw merge");

    Ok(())
}
