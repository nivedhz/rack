use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    domain::{
        ignore_file::IgnoreFile,
        repository::{self, Repository},
    },
    error::Result,
    storage::object_file,
};

pub fn run(path: PathBuf) -> Result<()> {
    let repo = repository::Repository::discover(&path)?;
    let ig_file_content = fs::read_to_string(repo.ignore_file())?;
    let ig = IgnoreFile::parse(ig_file_content);
    if path.is_dir() {
        handle_dir(&path, &ig, &repo)?;
    } else {
        handle_file(&path, &ig, &repo)?;
    }

    Ok(())
}
fn handle_dir(path: &Path, ig: &IgnoreFile, repo: &Repository) -> Result<()> {
    if ig.is_ignored(repo, path)? {
        return Ok(());
    }
    let dirs = fs::read_dir(path)?;
    for dir in dirs {
        let re_path = dir?.path();
        if ig.is_ignored(repo, &re_path)? {
            return Ok(());
        }
        if re_path.is_dir() {
            handle_dir(&re_path, ig, repo)?;
        } else {
            handle_file(&re_path, ig, repo)?;
        }
    }
    Ok(())
}
fn handle_file(path: &Path, ig: &IgnoreFile, repo: &Repository) -> Result<()> {
    if ig.is_ignored(repo, path)? {
        return Ok(());
    }
    object_file::write_blob(repo, path)?;
    Ok(())
}
