use std::{collections::HashMap, env, path::PathBuf};

use anyhow::Result;

use crate::{
    domain::{
        commit::{CommitEntry, CommitId},
        index::IndexEntries,
        repo_path::RepoPath,
        repository::Repository,
    },
    storage::fsutil,
};

pub fn run(path: PathBuf, staged: bool) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let idx_content = fsutil::read_file_to_string(&repo.index()).unwrap_or_default();
    let mut idx = IndexEntries::parse(idx_content)?;

    let _head = if let Ok(e) = fsutil::read_file_to_string(&repo.head_file()) {
        let head_id = CommitId::new(e);
        let head_content = fsutil::read_file_to_string(&repo.commit_entry_file(&head_id))?;
        let head = CommitEntry::parse(head_content, head_id.hash())?;
        head.entries()
    } else {
        IndexEntries::new(HashMap::default())
    };

    if staged {
        if idx.contains(&RepoPath::new(&path)).is_some() {
            idx.remove(RepoPath::new(&path));
            fsutil::write_atomic(&repo.index(), &idx.serialize())?;
        }
        return Ok(());
    }

    Ok(())
}
