use std::{env, fs, path::PathBuf};

use crate::{
    domain::{repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
    storage::{fsutil, index_file},
};

pub fn run(path: Vec<PathBuf>, cached: bool) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    for p in path {
        let repo_path = RepoPath::new(p);

        if !repo_path.path().exists() {
            return Err(Error::PathNotFound {
                path: repo_path.to_string(),
            });
        }

        if cached {
            let mut idx = index_file::get_index(&repo)?;
            if idx.contains(&repo_path).is_some() {
                idx.remove(&repo_path);

                println!("rm '{}'", repo_path);
                fsutil::write_atomic(&repo.index(), &idx.serialize())?;
            }

            continue;
        }

        if repo_path.path().is_dir() {
            fs::remove_dir(repo_path.path())?;
        } else {
            fs::remove_file(repo_path.path())?;
        }
    }

    Ok(())
}
