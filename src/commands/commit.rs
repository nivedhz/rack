use crate::repository;
use anyhow::{Context, Result};
use rand::RngExt;
use std::fs;
use std::io::Write;

pub fn run(message: String) -> Result<()> {
    create_commit_dir(&message)?;
    create_commit_entry(&message)?;
    Ok(())
}

fn create_commit_dir(message: &str) -> Result<()> {
    let rack_dir_path =
        repository::find_rack_dir().context("fatal: could not find rack repository")?;
    fs::create_dir_all(rack_dir_path.join("commit"))
        .context("fatal: could not create commit dir")?;
    println!("bang message: {}", message);
    Ok(())
}

fn create_commit_entry(message: &str) -> Result<()> {
    let hash = generate_random_hash();
    let rack_dir_path =
        repository::find_rack_dir().context("fatal: could not find rack repository")?;
    let dir_name = hash[..2].to_string();
    let file_name = hash[2..].to_string();
    fs::create_dir_all(rack_dir_path.join("commit").join(&dir_name))?;
    let mut commit_entry_file = fs::File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(rack_dir_path.join("commit").join(dir_name).join(file_name))?;
    commit_entry_file.write_all(message.as_bytes())?;
    Ok(())
}
fn generate_random_hash() -> String {
    let mut rng = rand::rng();
    let mut bytes = [0; 32];
    rng.fill(&mut bytes);
    hex::encode(bytes)
}
