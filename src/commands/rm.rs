use std::{collections::HashMap, env, fs, path::PathBuf};

use crate::{
    domain::{index::IndexEntries, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{baby_file, commit_file, fsutil, head_file, index_file},
};

// TODO: refactor and make it logically correct
pub fn run(path: Vec<PathBuf>, cached: bool) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let mut idx = index_file::get_index(&repo)?;
    let baby = baby_file::get_baby(&repo);

    let commit_entries = if let Ok(head) = head_file::get_commit(&baby.path_name().path()) {
        commit_file::get_entry(&repo, &head)?.entries()
    } else {
        IndexEntries::new(HashMap::default())
    };
    let tracked_entries = commit_entries.extend(idx.clone());

    for p in path {
        let repo_path = RepoPath::new(p);

        if repo_path.path().is_dir() {
            return Err(Error::PathIsDir {
                path: repo_path.to_string(),
            });
        }

        if !repo_path.path().exists() || !tracked_entries.contains(&repo_path).is_some() {
            return Err(Error::PathNotFound {
                path: repo_path.to_string(),
            });
        }

        if cached {
            if idx.contains(&repo_path).is_some() {
                idx.remove(&repo_path);
                fsutil::write_atomic(&repo.index(), &idx.serialize())?;

                println!("rm '{}'", repo_path);
            }

            continue;
        }

        fs::remove_file(repo_path.path())?;
        println!("rm '{}'", repo_path);
    }

    Ok(())
}
