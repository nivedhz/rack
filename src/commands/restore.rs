use std::{env, fs, path::PathBuf};

use crate::{
    domain::{
        branch::BabyEntry,
        commit::{CommitEntry, CommitId},
        index::IndexEntries,
        object::ObjectEntry,
        repo_path::RepoPath,
        repository::Repository,
    },
    error::{Error, Result},
    storage::fsutil,
};

pub fn run(path: PathBuf, staged: bool, commit: Option<String>) -> Result<()> {
    let repo_path = RepoPath::new(&path);
    let repo = Repository::discover(&env::current_dir()?)?;

    let idx_content = fsutil::read_file_to_string(&repo.index()).unwrap_or_default();
    let mut idx = IndexEntries::parse(idx_content)?;

    if staged {
        if idx.contains(&repo_path).is_some() {
            idx.remove(repo_path);
            fsutil::write_atomic(&repo.index(), &idx.serialize())?;
        }

        return Ok(());
    }

    let commit_id = match commit {
        Some(id) => CommitId::new(id)?,
        None => {
            let baby_content = fsutil::read_file_to_string(&repo.baby()).unwrap_or_default();
            let baby = BabyEntry::parse(baby_content);

            let id = fsutil::read_file_to_string(&repo.head(&baby.path_name().path()))
                .unwrap_or_default();
            if id.trim().is_empty() {
                return Err(Error::NoCommitsFound {
                    message: String::from("use rack commit to add commits!"),
                });
            }

            CommitId::new(id)?
        }
    };

    if !repo.commit_entry_file(&commit_id).exists() {
        return Err(Error::CommitNotFound {
            commit: commit_id.hash(),
        });
    }
    let commit_content = fsutil::read_file_to_string(&repo.commit_entry_file(&commit_id))?;
    let commit = CommitEntry::parse(commit_content, commit_id.hash())?.entries();

    if let Some(e) = commit.contains(&repo_path) {
        let obj_content = fsutil::read_file_to_bytes(&repo.object_entry_file(&e.object_id))?;
        let obj = ObjectEntry::from_bytes(obj_content, e.object_id)?;

        fsutil::write_atomic(&repo_path.path(), &obj.data)?;
    } else {
        fs::remove_file(repo_path.path())?;
    }

    Ok(())
}
