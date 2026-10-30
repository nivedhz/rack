use crate::{hasher, repository};
use anyhow::{Context, Result, bail};
use std::{
    fs,
    io::Write,
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
        handle_dir(&path)?
    } else {
        create_goods_entry(&path)?;
        create_panty_list_entry(&path)?;
    }
    Ok(())
}
fn handle_dir(path: &Path) -> Result<()> {
    let entries = fs::read_dir(path)?;
    let rack_ignore_entries = repository::read_rack_ignore()?;
    for entry in entries {
        let entry = entry?;
        let recursive_path = entry.path();
        if rack_ignore_entries.iter().any(|entry| {
            let relative_path_from_root =
                repository::get_relative_path_from_root(&recursive_path).unwrap();
            relative_path_from_root.starts_with(entry.trim())
        }) {
            continue;
        }
        if recursive_path.is_dir() {
            handle_dir(&recursive_path)?;
        } else {
            create_goods_entry(&recursive_path)?;
            create_panty_list_entry(&recursive_path)?;
        }
    }

    Ok(())
}
fn create_goods_entry(path: &Path) -> Result<()> {
    let rack_repo_dir = repository::find_rack_dir().context("fatal: not in a rack repository")?;
    let file_content = fs::read(path)?;
    let hash = hasher::get_hash(&file_content);
    let (dir_name, file_name) = hash.split_at(2);
    let good_entry_dir_path = rack_repo_dir.join("goods").join(dir_name);
    if !good_entry_dir_path.exists() {
        fs::create_dir_all(&good_entry_dir_path)
            .context("fatal: could not create/locate rack directory")?;
    }
    let file_type = b"boob\0".to_vec();
    // Header: <type>0x00
    // Body: <compressed file contents>
    let compressed_file_contents = zstd::stream::encode_all(
        &file_type
            .into_iter()
            .chain(file_content)
            .collect::<Vec<u8>>()[..],
        3,
    )?;
    let goods_entry_file_path = good_entry_dir_path.join(file_name);
    fs::write(&goods_entry_file_path, compressed_file_contents)?;
    Ok(())
}

fn create_panty_list_entry(path: &Path) -> Result<()> {
    let relative_path_from_root = repository::get_relative_path_from_root(path)?;
    let head_commit_entries = repository::read_head_commit_entry()?;

    if head_commit_entries.iter().any(|entry| {
        let filename = entry.split_whitespace().nth(1).unwrap_or_default();
        let file_hash = entry.split_whitespace().nth(2).unwrap_or_default();
        relative_path_from_root == Path::new(filename)
            && hasher::get_hash(&fs::read(path).unwrap()) == file_hash
    }) {
        return Ok(());
    }

    let file_metadata = fs::metadata(path)?;
    let file_content = fs::read(path)?;
    let hash = hasher::get_hash(&file_content);

    let new_entry = get_new_panty_list_entry(&relative_path_from_root, file_metadata, &hash);
    let panty_list_path = repository::find_rack_dir()
        .context("fatal: could not find rack repository")?
        .join("panty-list");

    let baby_bonkers_entry = repository::read_baby_bonkers_file()?;
    if !baby_bonkers_entry.trim().is_empty() {
        let head_commit_entries = repository::read_head_commit_entry()?;
        for entry in head_commit_entries {
            let (file_hash, file_name) = get_file_details(&entry);
            if Path::new(file_name) == path && file_hash == hash {
                println!("info: file {} has no changes", file_name);
                return Ok(());
            }
        }
    }

    let panty_list_entries = repository::read_panty_list_file()?;
    for entry in &panty_list_entries {
        let (entry_file_hash, entry_file_name) = get_file_details(entry);
        if Path::new(entry_file_name) == relative_path_from_root {
            if entry_file_hash != hash {
                println!("info: file {} already added", entry_file_name);
                return Ok(());
            } else {
                let new_content = panty_list_entries
                    .iter()
                    .find(|x| x.contains(&hash) && x.contains(path.to_str().unwrap_or_default()))
                    .unwrap_or(&new_entry);
                fs::write(panty_list_path, new_content.as_bytes())?;
                return Ok(());
            }
        }
    }
    fs::File::options()
        .append(true)
        .open(panty_list_path)?
        .write_all(new_entry.as_bytes())?;
    Ok(())
}

fn get_file_details(entry: &str) -> (&str, &str) {
    let entry_file_hash = entry.split_whitespace().nth(2).unwrap_or_default();
    let entry_file_name = entry.split_whitespace().nth(1).unwrap_or_default();
    (entry_file_hash, entry_file_name)
}

// Format:
// <size>0x20<path>0x20<hash>0x20<ctime>0x20<mtime>
fn get_new_panty_list_entry(
    relative_path_from_root: &Path,
    file_metadata: fs::Metadata,
    hash: &String,
) -> String {
    let entry = format!(
        "{} {} {} {} {}\n",
        file_metadata.len(),
        relative_path_from_root.to_str().unwrap_or_default(),
        hash,
        file_metadata.ctime_nsec(),
        file_metadata.mtime_nsec()
    );
    entry
}
