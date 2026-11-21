use std::fs;

use crate::{
    domain::{ignore::IgnoreEntries, repository::Repository},
    error::Result,
    storage::fsutil,
};

pub fn clear_root(repo: &Repository, ig: &IgnoreEntries) -> Result<()> {
    let files = fsutil::collect_paths(&repo.root(), repo, ig)?;
    for entry in files {
        if entry.path() == repo.ignore_file() {
            continue;
        }
        if entry.path().is_dir() {
            fs::remove_dir_all(entry.path())?;
        } else {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}
