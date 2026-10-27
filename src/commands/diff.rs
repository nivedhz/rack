use std::{
    env,
    fs::{self, canonicalize},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use itertools::Itertools;

use crate::{hasher::get_hash, repository};

pub fn run(path: PathBuf) -> Result<()> {
    let _path = get_diff_path(path)?;
    compute_diff()?;
    Ok(())
}

fn get_diff_path(path: PathBuf) -> Result<PathBuf> {
    if path == Path::new(".") {
        Ok(env::current_dir().context("fatal: could not get current directory")?)
    } else {
        Ok(canonicalize(path).context("fatal: path not found in current directory")?)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct FileEntry {
    path: PathBuf,
    hash: String,
}
impl FileEntry {
    fn new(path: PathBuf, hash: String) -> FileEntry {
        FileEntry { path, hash }
    }
}

fn compute_diff() -> Result<()> {
    let head_file_entries: Vec<FileEntry> =
        convert_from_file_entry(repository::read_head_commit_entry()?);
    let panty_list_entries: Vec<FileEntry> =
        convert_from_file_entry(repository::read_panty_list_file()?);

    let mut common_modified_entries: Vec<FileEntry> = Vec::new();
    for entry in &head_file_entries {
        let curr_file_content = fs::read(&entry.path).context("fatal: could not read file")?;
        let curr_file_hash = get_hash(&curr_file_content);
        if curr_file_hash != entry.hash
            && !panty_list_entries.contains(&FileEntry::new(entry.path.clone(), curr_file_hash))
        {
            common_modified_entries.push(entry.clone());
        }
    }
    for e in common_modified_entries {
        println!("File Path: {:?}", e.path);
        println!("File Hash: {:?}", e.hash)
    }

    Ok(())
}

fn convert_from_file_entry(file_entry: Vec<String>) -> Vec<FileEntry> {
    file_entry
        .iter()
        .map(|s| {
            let (_, path, hash, _, _) = s.split_whitespace().collect_tuple().unwrap();

            FileEntry::new(PathBuf::from(path), String::from(hash))
        })
        .collect()
}
