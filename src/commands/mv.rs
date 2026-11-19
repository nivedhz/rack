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

    let mut idx = index_file::get_index(&repo)?;

    let baby = baby_file::get_baby(&repo);
    let head_path = &repo.head(&baby.path_name().path());

    let head_entries = if let Ok(head_commit) = head_file::get_commit(head_path) {
        let head = commit_file::get_entry(&repo, &head_commit)?;
        head.idx()
    } else {
        IndexEntries::new(HashMap::default())
    };
    let tracked_entries = head_entries.extend(idx.clone());

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

    // TODO:
    // - return error on same dst and src - rack mv README.md . should return SamePathError
    let dst_canonical = if dst.path().is_absolute() {
        fsutil::get_canonical(&dst.path())?
    } else {
        fsutil::get_canonical(&env::current_dir()?.join(dst.path()))?
    };
    dbg!(&dst_canonical);
    if dst_canonical.starts_with(repo.root()) {
        let dst_parent = dst_canonical
            .parent()
            .ok_or(Error::NoParent {
                path: dst.to_string(),
            })?
            .to_path_buf();

        if !dst_parent.exists() {
            return Err(Error::PathNotFound {
                path: dst_parent.display().to_string(),
            });
        }

        let entry_path = RepoPath::new(fs::canonicalize(src.path())?.strip_prefix(repo.root())?);
        let mut prev_entry = match tracked_entries.contains(&entry_path) {
            Some(e) => e,
            None => {
                return Err(Error::PathNotVersionControlled {
                    path: dst.to_string(),
                });
            }
        };

        if dst.path().is_dir() {
            if !dst.path().exists() {
                return Err(Error::PathNotFound {
                    path: dst.to_string(),
                });
            }
            let new_path = dst.path().join(src.file_name());
            fs::rename(src.path(), &new_path)?;
            prev_entry.modify_path(
                RepoPath::new(fs::canonicalize(new_path)?).strip_prefix(&repo.root())?,
            );
        } else {
            fs::rename(src.path(), dst.path())?;
            prev_entry.modify_path(
                RepoPath::new(fs::canonicalize(dst.path())?).strip_prefix(&repo.root())?,
            );
        }

        if idx.contains(&entry_path).is_some() {
            idx.modify(&prev_entry);
        } else {
            idx.add(prev_entry.path.clone(), prev_entry);
        }

        fsutil::write_atomic(&repo.index(), &idx.serialize())?;
    } else {
        return Err(Error::PathNotInRepo {
            path: dst.to_string(),
            repo: repo.root().to_str().unwrap_or_default().to_string(),
        });
    }

    Ok(())
}
