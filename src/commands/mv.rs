use std::{collections::HashMap, env, fs, path::PathBuf};

use crate::{
    domain::{index::IndexEntries, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{baby_file, commit_file, fsutil, head_file, index_file},
};

// TODO: refactor and cleanup
// This is a very sloppy impelemntation of `mv` and we need to work on everything once again
pub fn run(src: PathBuf, dst: PathBuf) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let src = RepoPath::new(src);
    let dst = RepoPath::new(dst);

    let idx = index_file::get_index(&repo)?;

    let baby = baby_file::get_baby(&repo);
    let head_path = &repo.head(&baby.path_name().path());

    let head_entries = if let Ok(head_commit) = head_file::get_commit(head_path) {
        let head = commit_file::get_entry(&repo, &head_commit)?;
        head.entries()
    } else {
        IndexEntries::new(HashMap::default())
    };
    let tracked_entries = head_entries.extend(idx);

    // not handling dirs currently
    if src.path().is_dir() {
        return Err(Error::PathIsDir {
            path: src.to_string(),
        });
    }

    if !tracked_entries.contains(&src).is_some() {
        return Err(Error::PathNotVersionControlled {
            path: src.to_string(),
        });
    }

    // this is a very sloppy implementation and I don't know if it will work properly in every
    // condition
    let dst_canonical = fsutil::get_canonical(&dst.path())?;
    if dst_canonical.starts_with(repo.root()) {
        if dst.path().is_dir() {
            if !dst.path().exists() {
                return Err(Error::PathNotFound {
                    path: dst.to_string(),
                });
            }
            fs::rename(src.path(), dst.path().join(src.file_name()))?;
        } else {
            let dst_parent = dst
                .path()
                .parent()
                .ok_or(Error::NoParent {
                    path: dst.to_string(),
                })?
                .to_path_buf();

            if !dst_parent.exists() {
                return Err(Error::PathNotFound {
                    path: dst.to_string(),
                });
            }
            fs::rename(src.path(), dst.path())?;
        }
    } else {
        return Err(Error::PathNotInRepo {
            path: dst.to_string(),
            repo: repo.root().to_str().unwrap_or_default().to_string(),
        });
    }

    Ok(())
}
