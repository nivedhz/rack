use std::{collections::HashMap, env, fs, path::PathBuf};

use crate::{
    domain::{index::IndexEntries, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{baby_file, commit_file, fsutil, head_file, index_file},
};

// TODO: refactor and cleanup
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

    // TODO:
    // we need to handle paths which doesn't exist and edit the IndexEntries to the new path
    // A lot of fucking improvement is needed here as `is_dir()` check doesn't work on paths that
    // doesn't exist and every single path from `fsutil::get_canonical(&dst.path())` returns with '/' at the end so we cannot check if '/' is at the end to determine if it's directory or a file
    // Git handles it the same way so we ain't gotta worry about whether it is a directory or not
    // if we pass in a directory name in Git then it would make that the name of the file rather
    // than a 'dirname/curr_filename'
    let dst_canonical = fsutil::get_canonical(&dst.path())?;
    if dst_canonical.starts_with(repo.root()) {
        if dst.path().is_dir() {
            fs::rename(src.path(), dst.path().join(src.file_name()))?;
        } else {
            fs::rename(src.path(), dst.path())?;
        }
    } else {
        return Err(Error::PathNotInRepo {
            path: dst.to_string(),
        });
    }

    Ok(())
}
