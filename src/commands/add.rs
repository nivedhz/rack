use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    domain::{
        ignore::IgnoreEntries,
        repository::{self, Repository},
    },
    error::Result,
    storage::{fsutil, index_file, object_file},
};

pub fn run(path: PathBuf) -> Result<()> {
    let repo = repository::Repository::discover(&path)?;
    let path = fs::canonicalize(path)?;

    let ig_file_content = fsutil::read_file_to_string(&path)?;
    let ig = IgnoreEntries::parse(ig_file_content);

    if path.is_dir() {
        handle_dir(&path, &ig, &repo)?;
    } else {
        handle_file(&path, &ig, &repo)?;
    }

    Ok(())
}
fn handle_dir(path: &Path, ig: &IgnoreEntries, repo: &Repository) -> Result<()> {
    if ig.is_ignored(repo, path)? {
        return Ok(());
    }
    let dirs = fs::read_dir(path)?;
    for dir in dirs {
        let rec_path = dir?.path();

        if ig.is_ignored(repo, &rec_path)? {
            continue;
        }
        if rec_path.is_dir() {
            handle_dir(&rec_path, ig, repo)?;
        } else {
            handle_file(&rec_path, ig, repo)?;
        }
    }
    Ok(())
}
fn handle_file(path: &Path, ig: &IgnoreEntries, repo: &Repository) -> Result<()> {
    if ig.is_ignored(repo, path)? {
        return Ok(());
    }

    object_file::write_blob(repo, path)?;
    index_file::write_index(repo, path)?;
    Ok(())
}
