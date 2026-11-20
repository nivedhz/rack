use std::env;

use crate::{
    commands::branch,
    domain::{branch::BabyEntry, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{fsutil, head_file, ignore_file},
};

pub fn run(branch: String, create: bool) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let ig = ignore_file::get_ignored(&repo)?;

    if branch.trim().is_empty() {
        return Err(Error::InvalidBranch { name: branch });
    }

    let branch = RepoPath::new(branch);
    if create {
        if repo.branch(&branch.path()).exists() {
            return Err(Error::BranchExists {
                name: branch.to_string(),
            });
        }

        branch::run(Some(branch.to_string()), None)?;
    }

    if !repo.branch(&branch.path()).exists() {
        return Err(Error::BranchNotExists {
            name: branch.to_string(),
        });
    }

    let baby_entry =
        BabyEntry::new(RepoPath::new(repo.branch(&branch.path())).strip_prefix(&repo.rack_dir())?);
    fsutil::write_atomic(&repo.baby(), &baby_entry.serialize())?;

    let branch_id = head_file::get_commit(&repo.branch(&branch.path()))?;
    fsutil::materialize_commit(&branch_id, &repo, &ig)?;

    Ok(())
}
