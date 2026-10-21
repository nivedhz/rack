use anyhow::{Context, Result};
use colored::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{constants::DIR_PATH, hasher, repository};
#[derive(Debug)]
struct FileStatus {
    file_path: PathBuf,
    is_indexed: bool,
}

pub fn run() -> Result<()> {
    get_all_files()?;
    Ok(())
}
fn get_all_files() -> Result<()> {
    let rack_repo_root_path =
        repository::find_repo_root().context("fatal: could not find rack repository")?;

    let ignore_dirs: Vec<PathBuf> =
        repository::read_rack_ignore(&rack_repo_root_path.join(DIR_PATH))?
            .iter()
            .map(PathBuf::from)
            .collect();

    let indexed_files: Vec<PathBuf> = get_indexed_files()?.iter().map(PathBuf::from).collect();

    let files: Vec<FileStatus> =
        get_file_recursively(&rack_repo_root_path, ignore_dirs, indexed_files)?;

    let (indexed_files, unindexed_files): (Vec<&FileStatus>, Vec<&FileStatus>) =
        files.iter().partition(|f| f.is_indexed);

    println!();
    if !indexed_files.is_empty() {
        println!("Changes to be commited:");
        for file in indexed_files {
            println!(
                "      {}    {}",
                "modified:".green(),
                file.file_path.display().to_string().green()
            );
        }
        println!();
    }
    if !unindexed_files.is_empty() {
        println!("Changes not staged for commit:");
        println!("(use rack insert <file> to update what will be banged)");
        for file in unindexed_files {
            println!(
                "      {}    {}",
                "modified:".red(),
                file.file_path.display().to_string().red()
            );
        }
    }
    Ok(())
}
fn get_file_recursively(
    dir: &Path,
    ignore_dirs: Vec<PathBuf>,
    indexed_files: Vec<PathBuf>,
) -> Result<Vec<FileStatus>> {
    let entries = fs::read_dir(dir)?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let relative_path = repository::get_relative_path_from_root(&path)?;
        let prev_commit_entry: Vec<(String, String)> = repository::read_head_commit_entry()?
            .iter()
            .map(|e| {
                (
                    e.split_whitespace().nth(1).unwrap_or_default().to_string(),
                    e.split_whitespace().nth(2).unwrap_or_default().to_string(),
                )
            })
            .collect();
        if let Some(path_name) = path.file_name()
            && (ignore_dirs.contains(&PathBuf::from(path_name))
                || ignore_dirs.contains(&relative_path))
        {
            continue;
        } else {
            if path.is_dir() {
                files.extend(get_file_recursively(
                    &path,
                    ignore_dirs.clone(),
                    indexed_files.clone(),
                )?);
            } else {
                let file_content = fs::read(&path).unwrap_or_default();
                let hash = hasher::get_hash(&file_content);
                let prev_commit_hash = prev_commit_entry
                    .iter()
                    .find(|e| e.0 == relative_path.display().to_string())
                    .map(|e| e.1.to_owned())
                    .unwrap_or_default();
                if prev_commit_hash == hash {
                    continue;
                }
                files.push(FileStatus {
                    is_indexed: indexed_files.contains(&relative_path),
                    file_path: relative_path,
                });
            }
        }
    }
    Ok(files)
}
fn get_indexed_files() -> Result<Vec<String>> {
    let rack_dir = repository::find_rack_dir().context("fatal: could not find rack repository")?;
    if !rack_dir.join("panty-list").exists() {
        fs::File::create(rack_dir.join("panty-list"))?;
        return Ok(Vec::new());
    }
    let entries = repository::read_panty_list_file()?;
    if entries.is_empty() {
        return Ok(Vec::new());
    }
    Ok(entries
        .iter()
        .map(|s| s.split(" ").collect::<Vec<&str>>()[1].to_string())
        .collect::<Vec<String>>())
}
