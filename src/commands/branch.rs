use std::env;

use crate::{
    domain::{
        branch::{BabyEntry, BranchEntries, BranchIndexEntry},
        commit::CommitId,
        repo_path::RepoPath,
        repository::Repository,
    },
    error::{Error, Result},
    output,
    storage::fsutil,
};

pub fn run(name: Option<String>) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let baby_content = fsutil::read_file_to_string(&repo.baby())?;
    let baby = BabyEntry::parse(baby_content);

    let head_content =
        fsutil::read_file_to_string(&repo.head(&baby.path_name().path())).unwrap_or_default();
    let head_commit = CommitId::new(head_content)?;

    let branch_idx_content = fsutil::read_file_to_string(&repo.branch_index()).unwrap_or_default();
    let branch_idx = BranchEntries::parse(branch_idx_content)?;

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
