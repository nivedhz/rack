use std::{
    env, fs,
    io::BufRead,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::constants::DIR_PATH;

pub fn find_rack_dir() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    for ancestor in current_dir.ancestors() {
        if ancestor == Path::new("/") {
            return None;
        }
        let path = ancestor.join(DIR_PATH);
        if path.exists() && path.is_dir() {
            return Some(path);
        }
    }
    None
}

pub fn read_panty_list_file() -> Result<Vec<String>> {
    let mut entries: Vec<String> = Vec::new();
    let rack_repo_root_path = find_rack_dir().context("fatal: could not find rack repository")?;
    let panty_list_file =
        fs::File::open(rack_repo_root_path.join("panty-list")).context("fatal: no file added")?;
    let reader = std::io::BufReader::new(panty_list_file);
    for line in reader.lines() {
        entries.push(line?);
    }
    Ok(entries)
}
