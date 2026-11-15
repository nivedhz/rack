use std::{collections::HashMap, env, path::PathBuf};

use crate::{
    domain::{index::IndexEntries, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{baby_file, commit_file, head_file, index_file},
};

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
        return Err(Error::PathNotFound {
            path: src.to_string(),
        });
    }

    dbg!(tracked_entries);
    dbg!(src, dst);
    Ok(())
}
