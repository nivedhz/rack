use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
};

use anyhow::{Context, Result, bail};

use crate::{hasher, repository};

pub fn run(message: String) -> Result<()> {
    let hash = hasher::get_random_hash();
    let baby_bonkers_entry = repository::read_baby_bonkers_file()?;
    handle_commit(&hash, &message, !baby_bonkers_entry.trim().is_empty())?;
    Ok(())
}

fn handle_commit(hash: &str, message: &str, has_head: bool) -> Result<()> {
    let prev_commit_hash = repository::read_baby_bonkers_file()?;
    if has_head {
        let prev_commit_entry: Vec<String> = get_prev_commit_entry(prev_commit_hash.trim())?
            .trim()
            .lines()
            .map(String::from)
            .collect();
        let curr_file_entry = repository::read_panty_list_file()?;
        if curr_file_entry.is_empty() {
            println!("use rack insert <file> to insert a file");
            bail!("fatal: no file inserted");
        }
        let diff_entries = get_diff_entries(&prev_commit_entry, &curr_file_entry)?;
        create_commit_entry_with_head(&diff_entries.join("\n"), message, hash, &prev_commit_hash)?;
    } else {
        create_commit_dir().context("fatal: could not create commit dir")?;
        create_headless_commit_entry(hash, message)
            .context("fatal: could not create commit entry")?;
    }
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

fn create_commit_entry_with_head(
    diffed_entry: &str,
    message: &str,
    hash: &str,
    prev_commit_hash: &str,
) -> Result<()> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
    fs::create_dir_all(&entry_dir_path).context("fatal: could not create commit entry")?;
    let mut entry_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(entry_dir_path.join(&hash[2..]))?;
    entry_file
        .write_all(format!("{}\0{}\0\n{}", prev_commit_hash, message, diffed_entry).as_bytes())
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
    let commit_entry = get_commit_entry(message);
    entry_file
        .write_all(commit_entry?.as_bytes())
        .context("fatal: could not write commit entry")?;

    Ok(())
}
// TODO: make the header <prev head hash>0x20<message>0x20<author name>0x20<author email>0x20<commit timestamp>\n
// Header: <hash>0x20<message>0x00\n
// Body: <panty-list entries joined by \n>
fn get_commit_entry(message: &str) -> Result<String> {
    let panty_list_entries =
        repository::read_panty_list_file().context("fatal: could not read panty list")?;
    if panty_list_entries.is_empty() {
        println!("use rack insert <file> to insert a file");
        bail!("fatal: no file inserted");
    }
    let commit_entry = format!(
        "\0{}\0\n{}",
        message,
        panty_list_entries.join("\n").as_str()
    );

    Ok(commit_entry)
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

fn get_prev_commit_entry(hash: &str) -> Result<String> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
    let entry_file = fs::File::open(entry_dir_path.join(&hash[2..]))
        .context("fatal: could not open entry file")?;
    let mut reader = std::io::BufReader::new(entry_file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let entries: Vec<String> = buf.split("\0").map(String::from).collect();
    if let Some(commit_file_entry) = entries.last() {
        return Ok(commit_file_entry.to_string());
    };
    Ok(String::new())
}
fn get_diff_entries(prev_entries: &Vec<String>, curr_entries: &Vec<String>) -> Result<Vec<String>> {
    let mut result: HashMap<String, (Option<String>, Option<String>)> = HashMap::new();
    let mut result_vec: Vec<String> = Vec::new();

    for entry in prev_entries {
        let filename = entry.split_whitespace().nth(1).unwrap().to_string();
        result.entry(filename).or_insert((None, None)).0 = Some(entry.clone());
    }

    for entry in curr_entries {
        let filename = entry.split_whitespace().nth(1).unwrap().to_string();
        result.entry(filename).or_insert((None, None)).1 = Some(entry.clone());
    }
    for (_, (prev, curr)) in result {
        result_vec.push(curr.unwrap_or(prev.unwrap_or_default()));
    }

    Ok(result_vec)
}
