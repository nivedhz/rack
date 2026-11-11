use std::{env, fs};

use crate::{
    commands::branch,
    domain::{
        branch::BabyEntry,
        commit::{CommitEntry, CommitId},
        ignore::IgnoreEntries,
        object::{ObjectEntry, ObjectId},
        repo_path::RepoPath,
        repository::Repository,
    },
    error::{Error, Result},
    storage::fsutil,
};

pub fn run(branch: String, create: bool) -> Result<()> {
    let branch = RepoPath::new(branch);
    let repo = Repository::discover(&env::current_dir()?)?;
    let ig_content = fsutil::read_file_to_string(&repo.ignore_file())?;

    let ig = IgnoreEntries::parse(ig_content);

    if create {
        if repo.branch(&branch.path()).exists() {
            return Err(Error::BranchExists {
                name: branch.to_string(),
            });
        }

        branch::run(Some(branch.to_string()))?;
    }

    if !repo.branch(&branch.path()).exists() {
        return Err(Error::BranchNotExists {
            name: branch.to_string(),
        });
    }

    let baby_entry =
        BabyEntry::new(RepoPath::new(repo.branch(&branch.path())).strip_prefix(&repo.rack_dir())?);
    fsutil::write_atomic(&repo.baby(), &baby_entry.serialize())?;

    let branch_content = fsutil::read_file_to_string(&repo.branch(&branch.path()))?;
    let branch_id = CommitId::new(branch_content)?;

    let branch_head_content = fsutil::read_file_to_string(&repo.commit_entry_file(&branch_id))?;
    let branch_head_entry = CommitEntry::parse(branch_head_content, branch_id.hash())?.entries();

    let curr_file_state: Vec<RepoPath> = fsutil::collect_files(&repo.root(), &repo, &ig)?;

    for file in &curr_file_state {
        if let Some(e) = branch_head_entry.contains(&file.strip_prefix(&repo.root())?) {
            let f_content = fsutil::read_file_to_string(&file.path())?;
            let f_obj_id = ObjectId::from_bytes(f_content.as_bytes());

            if e.object_id == f_obj_id {
                continue;
            } else {
                let prev_file_obj =
                    fsutil::read_file_to_bytes(&repo.object_entry_file(&e.object_id))?;
                let prev_obj = ObjectEntry::from_bytes(prev_file_obj, e.object_id)?;
                let prev_content = String::from_utf8(prev_obj.data).unwrap_or_default();

                fsutil::write_atomic(&file.path(), prev_content.as_bytes())?;
            }
        } else {
            fs::remove_file(file.path())?;
        }
    }

    Ok(())
}
