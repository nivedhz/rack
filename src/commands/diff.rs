use std::{collections::HashMap, env, fs, path::PathBuf};

use crate::{
    domain::{
        branch::BabyEntry,
        commit::{CommitEntry, CommitId},
        index::IndexEntries,
        object::{ObjectEntry, ObjectId},
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

#[derive(Debug)]
pub struct DiffedContent {
    pub prev: String,
    pub curr: String,
}

pub fn run(path: PathBuf) -> Result<Vec<DiffedContent>> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let baby_content = fsutil::read_file_to_string(&repo.baby()).unwrap_or_default();
    let baby = BabyEntry::parse(baby_content);

    let head_entries = if let Ok(head_content) =
        fsutil::read_file_to_string(&repo.head(&baby.path_name().path()))
    {
        let head_id = CommitId::new(head_content)?;
        let head_commit_content = fsutil::read_file_to_string(&repo.commit_entry_file(&head_id))?;
        let head = CommitEntry::parse(head_commit_content, head_id.hash())?;
        head.entries()
    } else {
        IndexEntries::new(HashMap::default())
    };

    let idx = if let Ok(idx_content) = fsutil::read_file_to_string(&repo.index()) {
        IndexEntries::parse(idx_content)?
    } else {
        IndexEntries::new(HashMap::default())
    };

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

        let obj_content = fsutil::read_file_to_bytes(&repo.object_entry_file(&entry.object_id))?;
        let obj = ObjectEntry::from_bytes(obj_content, entry.object_id)?;
        let prev_content = String::from_utf8(obj.data).unwrap_or_default();

        entries.push(DiffedContent {
            prev: prev_content,
            curr: curr_content,
        });
    }

    Ok(entries)
}
