use std::{
    fs,
    io::{BufRead, Write},
};

use anyhow::{Context, Result};
use rand::RngExt;

use crate::repository;

pub fn run(message: String) -> Result<()> {
    let hash = create_commit_hash();
    let baby_bonkers_entry = read_baby_bonkers_file()?;
    if baby_bonkers_entry.trim().is_empty() {
        create_commit_dir().context("fatal: could not create commit dir")?;
        create_commit_entry(&hash, &message).context("fatal: could not create commit entry")?;
        write_baby_bonkers_file(&hash).context("fatal: could not write baby bonkers file")?;
        empty_panty_list().context("fatal: could not empty panty list")?;
    }
    Ok(())
}

fn create_commit_dir() -> Result<()> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    fs::create_dir_all(rack_dir_path.join("commit"))
        .context("fatal: could not create commit dir")?;
    Ok(())
}

fn create_commit_entry(hash: &str, message: &str) -> Result<()> {
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
fn generate_random_hash() -> String {
    let mut rng = rand::rng();
    let mut bytes = [0; 32];
    rng.fill(&mut bytes);
    hex::encode(bytes)
}
