use std::{collections::HashMap, env, fs, path::PathBuf};

use anyhow::Result;

use crate::{
    domain::{
        commit::{CommitEntry, CommitId},
        index::IndexEntries,
        object::ObjectEntry,
        repo_path::RepoPath,
        repository::Repository,
    },
    storage::fsutil,
};

pub fn run(path: PathBuf, staged: bool) -> Result<()> {
    let repo_path = RepoPath::new(&path);
    let repo = Repository::discover(&env::current_dir()?)?;

    let idx_content = fsutil::read_file_to_string(&repo.index()).unwrap_or_default();
    let mut idx = IndexEntries::parse(idx_content)?;

    let head = if let Ok(e) = fsutil::read_file_to_string(&repo.head_file()) {
        let head_id = CommitId::new(e);
        let head_content = fsutil::read_file_to_string(&repo.commit_entry_file(&head_id))?;
        let head = CommitEntry::parse(head_content, head_id.hash())?;
        head.entries()
    } else {
        IndexEntries::new(HashMap::default())
    };

    if staged {
        if idx.contains(&repo_path).is_some() {
            idx.remove(repo_path);
            fsutil::write_atomic(&repo.index(), &idx.serialize())?;
        }
        return Ok(());
    }
    if let Some(e) = head.contains(&repo_path) {
        let obj_content = fsutil::read_file_to_bytes(&repo.object_entry_file(&e.object_id))?;
        let obj = ObjectEntry::from_bytes(obj_content, e.object_id)?;

        fsutil::write_atomic(&repo_path.path(), &obj.data)?;
    } else {
        fs::remove_file(repo_path.path())?;
    }

    Ok(())
}
