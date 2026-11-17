use std::{collections::HashMap, env, fs, path::PathBuf};

use crate::{
    domain::{index::IndexEntries, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{baby_file, commit_file, fsutil, head_file, index_file},
};

pub fn run(path: Vec<PathBuf>, cached: bool) -> Result<()> {
    if path.is_empty() {
        return Err(Error::NoPathProvided);
    }

    let repo = Repository::discover(&env::current_dir()?)?;
    let mut idx = index_file::get_index(&repo)?;
    let baby = baby_file::get_baby(&repo);
    let head_path = repo.head(&baby.path_name().path());

    let commit_entries = if let Ok(head) = head_file::get_commit(&head_path) {
        commit_file::get_entry(&repo, &head)?.idx()
    } else {
        IndexEntries::new(HashMap::default())
    };
    let tracked_entries = commit_entries.extend(idx.clone());

    for p in path {
        if !p.exists() {
            return Err(Error::PathNotFound {
                path: p.display().to_string(),
            });
        }

        let repo_path = RepoPath::new(&p);

        let path = fs::canonicalize(&p)?;
        let rel_path = RepoPath::new(&path).strip_prefix(&repo.root())?;

        // directories are currently not handled
        if repo_path.path().is_dir() {
            return Err(Error::PathIsDir {
                path: repo_path.to_string(),
            });
        }

        if !tracked_entries.contains(&rel_path).is_some() {
            return Err(Error::PathNotVersionControlled {
                path: repo_path.to_string(),
            });
        }

        if cached && idx.contains(&rel_path).is_some() {
            idx.remove(&rel_path);
            fsutil::write_atomic(&repo.index(), &idx.serialize())?;
        }

        fs::remove_file(repo_path.path())?;
        println!("rm '{}'", repo_path);
    }

    Ok(())
}
