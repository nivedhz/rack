use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::constants::DIR_PATH;

pub fn read_rack_ignore(rack_dir: &Path) -> Result<Vec<String>> {
    let rack_repo_root_path = rack_dir
        .parent()
        .context("fatal: could not find rack repository")?
        .to_path_buf();
    let value = fs::read_to_string(rack_repo_root_path.join(".rackignore")).unwrap_or_default();
    let mut entries: Vec<&str> = value.trim().lines().collect();
    entries.push(DIR_PATH);
    Ok(entries.iter().map(|s| s.to_string()).collect())
}
