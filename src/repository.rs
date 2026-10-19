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
        let line = line
            .context("fatal: could not read line")?
            .trim()
            .to_string();

        if line.is_empty() {
            continue;
        }
        entries.push(line);
    }
    Ok(entries)
}

pub fn read_rack_ignore(rack_dir: &Path) -> Result<Vec<String>> {
    let rack_repo_root_path = rack_dir
        .parent()
        .context("fatal: could not find rack repository")?
        .to_path_buf();
    let value = fs::read_to_string(rack_repo_root_path.join(".rackignore")).unwrap_or_default();
    let mut entries: Vec<&str> = value.trim().lines().collect();
    entries.push(DIR_PATH);
    Ok(entries.iter().map(|s| s.trim().to_string()).collect())
}

pub fn read_baby_bonkers_file() -> Result<String> {
    let rack_repo_root_path = find_rack_dir().context("fatal: could not find rack repository")?;
    if !rack_repo_root_path.join("hump/baby/bonkers").exists() {
        return Ok(String::new());
    }
    let head_file = fs::File::open(rack_repo_root_path.join("hump/baby/bonkers"))?;
    let mut reader = std::io::BufReader::new(head_file);
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    Ok(buf)
}
