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

fn print_diffed_entries(file_entry: Vec<FileEntry>) -> Result<()> {
    for fe in file_entry {
        let prev_file_entry = repository::find_file_entry_in_head(&fe.path.to_string_lossy())?;
        let prev_hash = prev_file_entry
            .split_whitespace()
            .nth(2)
            .unwrap_or_default();
        let prev_goods_entry = repository::get_goods_entry(prev_hash)?;
        let prev_file_state = prev_goods_entry.split("\0").last().unwrap_or_default();
        let rack_repo_root_path =
            repository::find_repo_root().context("fatal: could not find rack repository")?;
        let curr_file = fs::File::options()
            .read(true)
            .open(rack_repo_root_path.join(&fe.path))?;
        let mut curr_file_state = String::new();
        let mut reader = BufReader::new(curr_file);
        reader.read_to_string(&mut curr_file_state)?;

        let patch = diffy::create_patch(prev_file_state, &curr_file_state);
        let f = PatchFormatter::new().with_color();
        println!("{}", f.fmt_patch(&patch));
    }
    Ok(())
}
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
