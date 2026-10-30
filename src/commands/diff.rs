use std::{
    env,
    fs::{self, canonicalize},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;
use colored::Colorize;
use itertools::Itertools;

use crate::{hasher, repository};

// TODO: implemented own diffing algorithm from scratch
// The Myers Diffing algorithm looks promising
pub fn run(path: PathBuf) -> anyhow::Result<()> {
    let path = get_diff_path(path)?;
    let diffed_files = get_diffed_files(path)?;
    print_diffed_entries(diffed_files)?;
    Ok(())
}

fn get_diff_path(path: PathBuf) -> anyhow::Result<PathBuf> {
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

fn print_diffed_entries(file_entry: Vec<FileEntry>) -> anyhow::Result<()> {
    for fe in file_entry {
        let prev_file_entry = repository::find_file_entry_in_head(&fe.path.to_string_lossy())?;
        let prev_hash = prev_file_entry
            .split_whitespace()
            .nth(2)
            .unwrap_or_default();
        let prev_goods_entry = repository::get_goods_entry(prev_hash)?;
        let prev_file_state = prev_goods_entry.split("\0").last().unwrap_or_default();
        let curr_file_state = read_curr_file(&fe.path)?;

        let patch = diffy::create_patch(prev_file_state, &curr_file_state);
        let formatter = diffy::PatchFormatter::new().with_color();
        println!(
            "{}\n{}",
            fe.path.to_string_lossy().bold(),
            formatter.fmt_patch(&patch)
        );
    }
    Ok(())
}

fn read_curr_file(path: &Path) -> anyhow::Result<String> {
    let rack_repo_root_path =
        repository::find_repo_root().context("fatal: could not find rack repository")?;
    let curr_file = fs::File::options()
        .read(true)
        .open(rack_repo_root_path.join(path))
        .context("fatal: could not read file")?;
    let mut curr_file_state = String::new();
    let mut reader = BufReader::new(curr_file);
    reader.read_to_string(&mut curr_file_state)?;
    Ok(curr_file_state)
}

fn get_diffed_files(path: PathBuf) -> anyhow::Result<Vec<FileEntry>> {
    let arg_path = fs::canonicalize(&path).unwrap_or_default();
    let head_file_entries: Vec<FileEntry> =
        convert_to_file_entry(repository::read_head_commit_entry()?)
            .into_iter()
            .filter(|e| {
                let entry_path = fs::canonicalize(&e.path).unwrap_or_default();
                entry_path.starts_with(&arg_path)
            })
            .collect();
    let panty_list_entries: Vec<FileEntry> =
        convert_to_file_entry(repository::read_panty_list_file()?);

    let mut common_modified_entries: Vec<FileEntry> = Vec::new();
    for entry in &head_file_entries {
        let curr_file_content = fs::read(&entry.path).context("fatal: could not read file")?;
        let curr_file_hash = hasher::get_hash(&curr_file_content);
        if curr_file_hash != entry.hash
            && !panty_list_entries.contains(&FileEntry::new(entry.path.clone(), curr_file_hash))
        {
            common_modified_entries.push(entry.clone());
        }
    }

    Ok(common_modified_entries)
}

fn convert_to_file_entry(file_entry: Vec<String>) -> Vec<FileEntry> {
    file_entry
        .iter()
        .map(|s| {
            let (_, path, hash, _, _) = s.split_whitespace().collect_tuple().unwrap();

            FileEntry::new(PathBuf::from(path), String::from(hash))
        })
        .collect()
}
