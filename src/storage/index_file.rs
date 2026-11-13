use std::{collections::HashMap, fs, os::unix::fs::MetadataExt, path::Path};

use crate::{
    domain::{
        branch::BabyEntry,
        commit::{CommitEntry, CommitId},
        index::{IndexEntries, IndexEntry},
        object::ObjectId,
        repo_path::RepoPath,
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub fn add_to_index(repo: &Repository, path: &Path) -> Result<()> {
    let c_path = fs::canonicalize(path)?;
    let rel_path = RepoPath::new(c_path.strip_prefix(repo.root())?);

    let idx_content = fsutil::read_file_to_string(&repo.index()).unwrap_or_default();
    let mut idx = IndexEntries::from_string(idx_content)?;

    let baby_content = fsutil::read_file_to_string(&repo.baby()).unwrap_or_default();
    let baby = BabyEntry::from_string(baby_content);

    let head_entries =
        if let Ok(id) = fsutil::read_file_to_string(&repo.head(&baby.path_name().path())) {
            let head_id = CommitId::new(id)?;
            let head_content = fsutil::read_file_to_string(&repo.commit_entry_file(&head_id))?;
            let head = CommitEntry::from_string(head_content, head_id.hash())?;
            head.entries()
        } else {
            IndexEntries::new(HashMap::default())
        };

    let tracked_entries = head_entries.extend(idx.clone());

    if let Some(e_idx) = tracked_entries.contains(&rel_path) {
        let f_content = fsutil::read_file_to_string(path)?;
        let f_metadata = fs::metadata(path)?;

        let f_idx = IndexEntry::new(
            f_metadata.size(),
            rel_path,
            ObjectId::from_bytes(f_content.as_bytes()),
            f_metadata.ctime_nsec(),
            f_metadata.mtime_nsec(),
        );
        if e_idx.is_equal(&f_idx) {
            return Ok(());
        } else {
            if idx.contains(&e_idx.path).is_some() {
                idx.modify(&f_idx);
            } else {
                idx.add(f_idx.clone().path, f_idx);
            }
            fsutil::write_atomic(&repo.index(), &idx.serialize())?;

            return Ok(());
        }
    }

    let f_content = fsutil::read_file_to_string(path)?;
    let f_metadata = fs::metadata(path)?;
    let obj_id = ObjectId::from_bytes(f_content.as_bytes());

    let idx_entry = IndexEntry::new(
        f_metadata.size(),
        rel_path,
        obj_id,
        f_metadata.ctime_nsec(),
        f_metadata.mtime_nsec(),
    );

    fsutil::append_atomic(&repo.index(), &idx_entry.serialize())?;

    Ok(())
}

pub fn get_index(repo: &Repository) -> Result<IndexEntries> {
    let idx_content = fsutil::read_file_to_string(&repo.index()).unwrap_or_default();
    IndexEntries::from_string(idx_content)
}
