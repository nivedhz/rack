use crate::{constants::DIR_PATH, repository};
use anyhow::{Context, Result, bail};
use sha3::{Digest, Sha3_256};
use std::{
    fs::{self},
    io::{BufReader, Read, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub fn run(path: PathBuf) -> Result<()> {
    let rack_dir = repository::find_rack_dir().context("fatal: could not find rack repository")?;
    if let Some(path_string) = path.to_str()
        && path_string.trim().is_empty()
    {
        bail!("fatal: empty path")
    }
    if !rack_dir.exists() {
        bail!("fatal: not a rack repository")
    }
    if path.is_dir() {
        handle_dir(path.to_path_buf())?
    } else {
        create_goods_entry(&path, None)?;
        create_panty_list_entry(&path, None)?;
    }
    Ok(())
}
fn handle_dir(path: PathBuf) -> Result<()> {
    let entries = fs::read_dir(&path)?;
    for entry in entries {
        let entry = entry?;
        let recursion_path = entry.path();
        if recursion_path.is_dir() {
            handle_dir(recursion_path)?;
        } else {
            create_goods_entry(&path, Some(&recursion_path))?;
            create_panty_list_entry(&path, Some(recursion_path))?;
        }
    }

    Ok(())
}
fn create_goods_entry(path: &Path, recursive_path: Option<&Path>) -> Result<()> {
    let path = recursive_path.unwrap_or(path);
    let rack_repo_dir = repository::find_rack_dir().context("fatal: not in a rack repository")?;
    let rack_repo_root_path = rack_repo_dir
        .parent()
        .context("fatal: could not find parent")?
        .to_path_buf();
    let full_path = fs::canonicalize(path)?;
    let relative_path_from_root = full_path.strip_prefix(&rack_repo_root_path)?;
    let rack_ignore_entries = repository::read_rack_ignore(&rack_repo_dir)?;
    if rack_ignore_entries
        .iter()
        .any(|entry| relative_path_from_root.starts_with(entry.trim()))
    {
        return Ok(());
    }
    let file_content = fs::read(path)?;
    let hash = get_hash(&file_content);
    let (dir_name, file_name) = hash.split_at(2);
    let dir_path = rack_repo_dir.join("goods").join(dir_name);
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).context("fatal: could not create/locate rack directory")?;
    }
    let mut file_type = b"boob".to_vec();
    file_type.push(0x00);
    let total_content = [file_type, file_content].concat();
    let compressed_file_contents = zstd::stream::encode_all(&total_content[..], 3)?;
    let file_path = dir_path.join(file_name);
    fs::write(&file_path, compressed_file_contents)?;
    Ok(())
}

fn get_hash(file_content: &Vec<u8>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(file_content);
    hex::encode(hasher.finalize())
}
fn create_panty_list_entry(path: &Path, file_name: Option<PathBuf>) -> Result<()> {
    let rack_repo_root_path = repository::find_rack_dir()
        .context("fatal: could not find rack repository")?
        .parent()
        .context("fatal: could not find rack repository")?
        .to_path_buf();
    let relative_file_name = file_name.unwrap_or(path.to_path_buf());
    let canonical_file_name = fs::canonicalize(&relative_file_name)?;
    let relative_path_from_root = canonical_file_name.strip_prefix(&rack_repo_root_path)?;
    let rack_ignore_entries = repository::read_rack_ignore(&rack_repo_root_path.join(DIR_PATH))?;
    if rack_ignore_entries
        .iter()
        .any(|entry| relative_path_from_root.starts_with(entry.trim()))
    {
        return Ok(());
    }

    let file_metadata = fs::metadata(&relative_file_name)?;
    let file_content = fs::read(&relative_file_name)?;
    let hash = get_hash(&file_content);

    let entry = format!(
        "{} {} {} {} {}\n",
        file_metadata.len(),
        relative_path_from_root.to_str().unwrap_or_default(),
        hash,
        file_metadata.ctime_nsec(),
        file_metadata.mtime_nsec()
    );
    let panty_list_path = repository::find_rack_dir()
        .context("fatal: could not find rack repository")?
        .join("panty-list");
    let mut panty_list_file = fs::File::options()
        .create(true)
        .read(true)
        .append(true)
        .truncate(false)
        .open(&panty_list_path)?;
    let mut read_buf: Vec<u8> = Vec::new();
    panty_list_file.read_to_end(&mut read_buf)?;
    let file_content = String::from_utf8_lossy(&read_buf[..]).to_string();
    let baby_bonkers_entry = repository::read_baby_bonkers_file()?;
    if !baby_bonkers_entry.trim().is_empty() {
        let commit_entry_file = fs::File::options().read(true).open(
            rack_repo_root_path
                .join(DIR_PATH)
                .join("commit")
                .join(&baby_bonkers_entry.trim()[..2])
                .join(&baby_bonkers_entry.trim()[2..]),
        )?;
        let mut commit_entry = String::new();
        let mut reader = BufReader::new(commit_entry_file);
        reader.read_to_string(&mut commit_entry)?;
        let entries = commit_entry.trim().split("\0").nth(1).unwrap_or_default();
        for line_entry in entries.lines() {
            let file_name = line_entry.split_whitespace().nth(1).unwrap_or_default();
            let file_hash = line_entry.split_whitespace().nth(2).unwrap_or_default();
            if file_name == relative_file_name.to_str().unwrap_or_default() && file_hash == hash {
                println!("info: file {} has no changes", file_name);
                return Ok(());
            }
        }
    }
    for contents in file_content.trim().lines() {
        let file_hash = contents.split_whitespace().nth(2).unwrap_or_default();
        let file_name = contents.split_whitespace().nth(1).unwrap_or_default();
        if file_name == relative_file_name.to_str().unwrap_or_default() {
            if file_hash == hash {
                println!("info: file {} already added", file_name);
                return Ok(());
            } else {
                let new_content = file_content.trim().replace(contents, entry.as_str());
                fs::write(panty_list_path, new_content.as_bytes())?;
                return Ok(());
            }
        }
    }
    panty_list_file.write_all(entry.as_bytes())?;
    Ok(())
}
