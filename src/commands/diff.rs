use std::{collections::HashMap, env, fs, path::PathBuf};

use crate::{
    domain::{index::IndexEntries, object::ObjectId, repository::Repository},
    error::Result,
    storage::{baby_file, commit_file, fsutil, head_file, index_file, object_file},
};

#[derive(Debug)]
pub struct DiffedContent {
    pub prev: String,
    pub curr: String,
}

pub fn run(path: PathBuf) -> Result<Vec<DiffedContent>> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let baby = baby_file::get_baby(&repo);

    let head_file = repo.head(&baby.path_name().path());
    let head_entries = if head_file.exists() {
        let head_commit_id = head_file::get_commit(&head_file)?;
        let commit_entry = commit_file::get_entry(&repo, &head_commit_id)?;

        commit_entry.entries()
    } else {
        IndexEntries::new(HashMap::new())
    };

    let idx = index_file::get_index(&repo)?;

    let diff_entries = head_entries.extend(idx);
    let mut entries: Vec<DiffedContent> = Vec::new();

    for (e_path, entry) in diff_entries.entries() {
        let curr_content = fsutil::read_file_to_string(&e_path.path()).unwrap_or_default();
        if curr_content.trim().is_empty() {
            continue;
        }

        let curr_id = ObjectId::from_bytes(curr_content.as_bytes());
        let e_path = fs::canonicalize(e_path.path())?;

        if curr_id == entry.object_id || !e_path.starts_with(fs::canonicalize(&path)?) {
            continue;
        }

        let obj = object_file::get_object_entry(&repo, entry.object_id)?;
        let prev_content = String::from_utf8(obj.data).unwrap_or_default();

        entries.push(DiffedContent {
            prev: prev_content,
            curr: curr_content,
        });
    }

    Ok(entries)
}
