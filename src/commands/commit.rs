use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
};

use anyhow::{Context, Result, bail};
use chrono::Utc;

use crate::{hasher, repository};

pub fn run(message: String) -> Result<()> {
    let hash = hasher::get_random_hash();
    let baby_bonkers_entry = repository::read_baby_bonkers_file()?;
    handle_commit(&hash, &message, !baby_bonkers_entry.trim().is_empty())?;
    Ok(())
}

fn handle_commit(hash: &str, message: &str, has_head: bool) -> Result<()> {
    let prev_commit_hash = repository::read_baby_bonkers_file()?;
    let curr_file_entries = repository::read_panty_list_file()?;
    if curr_file_entries.is_empty() {
        println!("use rack insert <file> to insert a file");
        bail!("fatal: no file inserted");
    }
    if has_head {
        let prev_file_entries: Vec<String> = get_prev_commit_entry(prev_commit_hash.trim())?;
        let diff_entries = get_diff_entries(&prev_file_entries, &curr_file_entries)?;
        create_commit_entry_with_head(diff_entries, message, hash, &prev_commit_hash)?;
    } else {
        create_commit_dir().context("fatal: could not create commit dir")?;
        create_headless_commit_entry(hash, message, curr_file_entries)
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
    diffed_entry: Vec<String>,
    message: &str,
    hash: &str,
    prev_commit_hash: &str,
) -> Result<()> {
    let user_detais = repository::get_user_details()?;
    if let Some(username) = user_detais.name
        && let Some(user_email) = user_detais.email
    {
        let rack_dir_path =
            repository::find_rack_dir().context("fatal: could not find rack dir")?;
        let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
        fs::create_dir_all(&entry_dir_path).context("fatal: could not create commit entry")?;
        let mut entry_file = fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(entry_dir_path.join(&hash[2..]))?;
        let commit_entry = get_commit_entry(
            Some(prev_commit_hash),
            message,
            &username,
            &user_email,
            diffed_entry,
        )?;
        entry_file
            .write_all(commit_entry.as_bytes())
            .context("fatal: could not write commit entry")?;
        Ok(())
    } else {
        println!("use rack config <key> <value> to set user details");
        bail!("fatal: user details not found");
    }
}

fn create_headless_commit_entry(
    hash: &str,
    message: &str,
    curr_file_entries: Vec<String>,
) -> Result<()> {
    let user_details = repository::get_user_details()?;
    if let Some(username) = user_details.name
        && let Some(user_email) = user_details.email
    {
        let rack_dir_path =
            repository::find_rack_dir().context("fatal: could not find rack dir")?;
        let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
        fs::create_dir_all(&entry_dir_path).context("fatal: could not create commit entry")?;
        let mut entry_file = fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(entry_dir_path.join(&hash[2..]))?;
        let commit_entry =
            get_commit_entry(None, message, &username, &user_email, curr_file_entries)?;
        entry_file
            .write_all(commit_entry.as_bytes())
            .context("fatal: could not write commit entry")?;
    } else {
        println!("use rack config <key> <value> to set user details");
        bail!("fatal: user details not found");
    }
    Ok(())
}

fn get_commit_entry(
    prev_commit_hash: Option<&str>,
    message: &str,
    username: &str,
    user_email: &str,
    entries: Vec<String>,
) -> Result<String> {
    let commit_timestamp = get_current_time();
    // Header: <prev head hash>0x20<message>0x20<author name>0x20<author email>0x20<commit timestamp>\n
    // Body: <panty-list entries joined by \n>
    let commit_entry = format!(
        "{}\0{}\0{}\0{}\0{}\0\n{}",
        prev_commit_hash.unwrap_or_default().trim(),
        message,
        username,
        user_email,
        commit_timestamp,
        entries.join("\n").as_str()
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

fn get_prev_commit_entry(hash: &str) -> Result<Vec<String>> {
    let rack_dir_path = repository::find_rack_dir().context("fatal: could not find rack dir")?;
    let entry_dir_path = rack_dir_path.join("commit").join(&hash[..2]);
    let entry_file = fs::File::open(entry_dir_path.join(&hash[2..]))
        .context("fatal: could not open entry file")?;
    let mut reader = std::io::BufReader::new(entry_file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let entries: Vec<String> = buf.split("\0").map(String::from).collect();
    if let Some(commit_file_entry) = entries.last() {
        let commit_file_entry: Vec<String> =
            commit_file_entry.trim().lines().map(String::from).collect();
        return Ok(commit_file_entry);
    };
    Ok(Vec::new())
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

fn get_current_time() -> i64 {
    let now = Utc::now();
    now.timestamp_millis()
}
