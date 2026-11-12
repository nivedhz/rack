use std::{env, fs};

use crate::{
    commands::branch,
    domain::{branch::BabyEntry, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{commit_file, fsutil, head_file, ignore_file, object_file},
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
    let mut branch_head_entry = commit_file::get_entry(&repo, &branch_id)?.entries();

    let curr_file_state: Vec<RepoPath> = fsutil::collect_files(&repo.root(), &repo, &ig)?;

    for file in &curr_file_state {
        if let Some(e) = branch_head_entry.contains(&file.strip_prefix(&repo.root())?) {
            let f_obj_id = object_file::get_object_id(&file.path())?;
            branch_head_entry.remove(&e.path);

            if e.object_id == f_obj_id {
                continue;
            } else {
                let prev_obj = object_file::get_object_entry(&repo, e.object_id)?;
                let prev_content = String::from_utf8(prev_obj.data).unwrap_or_default();

                fsutil::write_atomic(&file.path(), prev_content.as_bytes())?;
            }
        } else {
            fs::remove_file(file.path())?;
        }
    }

    // the entries in `branch_head_entry` but not in `curr_file_state` will be dealt with in here
    for (repo_path, entry) in branch_head_entry.entries() {
        let object = object_file::get_object_entry(&repo, entry.object_id)?;
        fsutil::write_atomic(&repo_path.path(), &object.data)?;
    }

    Ok(())
}
