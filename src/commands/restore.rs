use std::{env, fs, path::PathBuf};

use crate::{
    domain::{commit::CommitId, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{baby_file, commit_file, fsutil, index_file, object_file},
};

pub fn run(path: PathBuf, staged: bool, commit: Option<String>) -> Result<()> {
    let repo_path = RepoPath::new(&path);
    let repo = Repository::discover(&env::current_dir()?)?;

    let mut idx = index_file::get_index(&repo)?;

    if staged {
        if idx.contains(&repo_path).is_some() {
            idx.remove(&repo_path);
            fsutil::write_atomic(&repo.index(), &idx.serialize())?;
        }

        return Ok(());
    }

    let commit_id = match commit {
        Some(id) => CommitId::new(id)?,
        None => {
            let baby = baby_file::get_baby(&repo);

            let commit_id = fsutil::read_file_to_string(&repo.head(&baby.path_name().path()))
                .unwrap_or_default();
            if commit_id.trim().is_empty() {
                return Err(Error::NoCommitsFound {
                    message: String::from("use rack commit to add commits!"),
                });
            }

            CommitId::new(commit_id)?
        }
    };

    if !repo.commit_entry_file(&commit_id).exists() {
        return Err(Error::CommitNotFound {
            commit: commit_id.hash(),
        });
    }
    let commit = commit_file::get_entry(&repo, &commit_id)?.entries();

    if let Some(e) = commit.contains(&repo_path) {
        let obj = object_file::get_object_entry(&repo, e.object_id)?;

        fsutil::write_atomic(&repo_path.path(), &obj.data)?;
    } else {
        fs::remove_file(repo_path.path())?;
    }

    Ok(())
}
