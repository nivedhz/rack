// TODO: refactor this as whole. Make the code more readable
// FIXME: fix the small bugs
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
};

use anyhow::{Context, Result};
use rand::RngExt;

use crate::repository;

pub fn run(message: String) -> Result<()> {
    let hash = create_commit_hash();
    let baby_bonkers_entry = repository::read_baby_bonkers_file()?;
    if baby_bonkers_entry.trim().is_empty() {
        handle_fresh_commit(&hash, &message)?;
    } else {
        handle_not_fresh_commit(&hash, &message)?;
    }
    Ok(())
}

fn handle_fresh_commit(hash: &str, message: &str) -> Result<()> {
    create_commit_dir().context("fatal: could not create commit dir")?;
    create_headless_commit_entry(hash, message).context("fatal: could not create commit entry")?;
    write_baby_bonkers_file(hash).context("fatal: could not write baby bonkers file")?;
    empty_panty_list().context("fatal: could not empty panty list")?;

    Ok(())
}

fn handle_not_fresh_commit(hash: &str, message: &str) -> Result<()> {
    let prev_commit_hash = repository::read_baby_bonkers_file()?;
    let prev_file_entry: Vec<String> = get_file_entries(prev_commit_hash.trim())?
        .trim()
        .lines()
        .map(String::from)
        .collect();
    let curr_file_entry = repository::read_panty_list_file()?;
    let diff_entries = get_diff_entries(&curr_file_entry, &prev_file_entry)?;
    create_commit_entry_with_head(&diff_entries, message, hash)?;
    write_baby_bonkers_file(hash).context("fatal: could not write baby bonkers file")?;
    empty_panty_list().context("fatal: could not empty panty list")?;
    Ok(())
}

fn create_commit_dir() -> Result<()> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    fs::create_dir_all(rack_dir_path.join("commit"))
        .context("fatal: could not create commit dir")?;
    Ok(())
}

fn create_commit_entry_with_head(diffed_entry: &str, message: &str, hash: &str) -> Result<()> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
    fs::create_dir_all(&entry_dir_path).context("fatal: could not create commit entry")?;
    let mut entry_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(entry_dir_path.join(&hash[2..]))?;
    entry_file
        .write_all(format!("{} {}\0\n{}", hash, message, diffed_entry).as_bytes())
        .context("fatal: could not write commit entry")?;

    Ok(())
}

fn create_headless_commit_entry(hash: &str, message: &str) -> Result<()> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
    fs::create_dir_all(&entry_dir_path).context("fatal: could not create commit entry")?;
    let mut entry_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(entry_dir_path.join(&hash[2..]))?;
    let commit_entry = get_commit_entry(hash, message);
    entry_file
        .write_all(commit_entry?.as_bytes())
        .context("fatal: could not write commit entry")?;

    Ok(())
}
fn get_commit_entry(hash: &str, message: &str) -> Result<String> {
    let panty_list_entries =
        repository::read_panty_list_file().context("fatal: could not read panty list")?;
    let mut commit_entry = format!("{} {}\0\n", hash, message);
    for entry in panty_list_entries {
        commit_entry.push_str(&format!("{}\n", entry));
    }

    Ok(commit_entry)
}

fn create_commit_hash() -> String {
    let random_bytes: [u8; 32] = rand::rng().random();
    hex::encode(random_bytes)
}
fn write_baby_bonkers_file(hash: &str) -> Result<()> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let mut file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(rack_dir_path.join("hump/baby/bonkers"))?;
    file.write_all(hash.as_bytes())?;
    Ok(())
}

fn empty_panty_list() -> Result<()> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let file = fs::File::options()
        .write(true)
        .truncate(true)
        .open(rack_dir_path.join("panty-list"))?;
    file.set_len(0)?;

    Ok(())
}

fn get_file_entries(hash: &str) -> Result<String> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
    let entry_file = fs::File::open(entry_dir_path.join(&hash[2..]))
        .context("fatal: could not open entry file")?;
    let mut reader = std::io::BufReader::new(entry_file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let entries = buf
        .split_once("\0")
        .context("fatal: could not read commit entry")?
        .1
        .to_string();

    Ok(entries)
}
struct DiffFileEntry {
    filename: String,
    hash: String,
}
impl DiffFileEntry {
    fn new(entry: String) -> Self {
        let filename = entry.split_whitespace().next().unwrap().to_string();
        let hash = entry.split_whitespace().nth(1).unwrap().to_string();
        DiffFileEntry { filename, hash }
    }
}
fn get_diff_entries(
    curr_file_entries: &Vec<String>,
    prev_file_entries: &Vec<String>,
) -> Result<String> {
    let mut diffed_result: HashMap<String, String> = std::collections::HashMap::new();
    for pe in prev_file_entries {
        let prev_file_entry = DiffFileEntry::new(pe.to_string());
        diffed_result.insert(prev_file_entry.filename, prev_file_entry.hash);
    }
    for ce in curr_file_entries {
        let curr_file_entry = DiffFileEntry::new(ce.to_string());
        diffed_result.insert(curr_file_entry.filename, curr_file_entry.hash);
    }
    let mut result = String::new();
    for (filename, hash) in diffed_result {
        result.push_str(&format!("{} {}\n", filename, hash));
    }
    Ok(result)
}
