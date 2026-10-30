use std::{
    env, fs,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::Deserialize;
use zstd::Decoder;

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
pub fn find_repo_root() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    for ancestor in current_dir.ancestors() {
        if ancestor == Path::new("/") {
            return None;
        }
        let path = ancestor.join(DIR_PATH);
        if path.exists() && path.is_dir() {
            return Some(path.parent()?.to_path_buf());
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

pub fn read_rack_ignore() -> Result<Vec<String>> {
    let rack_repo_root_path = find_repo_root().context("fatal: could not find rack repository")?;
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
    let head_file = fs::File::open(rack_repo_root_path.join("hump/baby/bonkers"))
        .context("fatal: no commits available")?;
    let mut reader = std::io::BufReader::new(head_file);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .context("fatal: could not read baby bonkers file")?;
    Ok(buf)
}

pub fn read_head_commit_entry() -> Result<Vec<String>> {
    let baby_bonkers_entry = read_baby_bonkers_file()?;
    if baby_bonkers_entry.trim().is_empty() {
        return Ok(Vec::new());
    }
    let rack_repo_root_path = find_repo_root().context("fatal: could not find rack repository")?;

    let commit_entry_file = fs::File::options().read(true).open(
        rack_repo_root_path
            .join(DIR_PATH)
            .join("commit")
            .join(&baby_bonkers_entry.trim()[..2])
            .join(&baby_bonkers_entry.trim()[2..]),
    )?;
    let mut buf = String::new();
    let mut reader = BufReader::new(commit_entry_file);
    reader.read_to_string(&mut buf)?;
    let commit_entry: Vec<String> = buf.trim().split("\0").map(|s| s.to_string()).collect();
    if let Some(commit_file_entries) = commit_entry.last() {
        let entries: Vec<String> = commit_file_entries
            .trim()
            .lines()
            .map(String::from)
            .collect();
        Ok(entries)
    } else {
        Ok(Vec::new())
    }
}

pub fn get_relative_path_from_root(path: &Path) -> Result<PathBuf> {
    let rack_repo_root_path = find_rack_dir()
        .context("fatal: could not find rack repository")?
        .parent()
        .context("fatal: could not find rack repository")?
        .to_path_buf();
    let canonical_file_name = fs::canonicalize(path)?;
    let relative_path_from_root = canonical_file_name.strip_prefix(&rack_repo_root_path)?;
    Ok(relative_path_from_root.to_path_buf())
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub user: UserDetails,
}
#[derive(Deserialize, Debug)]
pub struct UserDetails {
    pub name: Option<String>,
    pub email: Option<String>,
}
pub fn get_user_details() -> Result<UserDetails> {
    let config_file_path = find_rack_dir()
        .context("fatal: could not find rack repository")?
        .join("config.toml");
    let file_content =
        fs::read_to_string(config_file_path).context("fatal: could not read config file")?;
    let config: Config = toml::from_str(&file_content).context("fatal: could not parse config")?;

    Ok(config.user)
}

pub fn find_file_entry_in_head(filename: &str) -> Result<String> {
    let curr_head_hash = read_baby_bonkers_file()?;
    let rack_repo_path = find_rack_dir().context("fatal: could not find rack repository")?;
    let commit_entry_file = fs::File::options().read(true).open(
        rack_repo_path
            .join("commit")
            .join(&curr_head_hash[..2])
            .join(&curr_head_hash[2..]),
    )?;
    let mut buf = String::new();
    let mut reader = BufReader::new(commit_entry_file);
    reader.read_to_string(&mut buf)?;
    let file_entries = buf.split("\0").last().unwrap_or_default().to_string();
    for entry in file_entries.lines() {
        let entry_file_name = entry.split_whitespace().nth(1).unwrap_or_default();
        if entry_file_name == filename {
            return Ok(entry.to_string());
        }
    }
    Ok(String::new())
}

pub fn get_goods_entry(hash: &str) -> Result<String> {
    let rack_repo_path = find_rack_dir().context("fatal: could not find rack repository")?;
    let entry_file = fs::File::options().read(true).open(
        rack_repo_path
            .join("goods")
            .join(&hash[..2])
            .join(&hash[2..]),
    )?;
    let mut decoder = Decoder::new(entry_file)?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok(String::from_utf8_lossy(&decompressed).to_string())
}
