use std::{collections::HashMap, env, os::unix::fs::MetadataExt};

use crate::{
    domain::{
        commit::{CommitEntry, CommitId},
        ignore::IgnoreEntries,
        index::{IndexEntries, IndexEntry},
        object_id::ObjectId,
        repo_path::RepoPath,
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub struct StatusResult {
    pub idx_entries: Vec<RepoPath>,
    pub un_idx_entries: Vec<RepoPath>,
    pub untracked_entries: Vec<RepoPath>,
}

pub fn run() -> Result<StatusResult> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let ig_content = fsutil::read_file_to_string(&repo.ignore_file())?;
    let ig = IgnoreEntries::parse(ig_content);

    let idx_content = fsutil::read_file_to_string(&repo.index()).unwrap_or_default();
    let idx = IndexEntries::parse(idx_content)?;

    let head_content = fsutil::read_file_to_string(&repo.head_file()).unwrap_or_default();
    let head_entries = if !head_content.trim().is_empty() {
        let head_commit_id = CommitId::new(head_content);
        let head_commit_content =
            fsutil::read_file_to_string(&repo.commit_entry_file(&head_commit_id))
                .unwrap_or_default();
        CommitEntry::parse(head_commit_content, head_commit_id.hash())?.entries()
    } else {
        IndexEntries::new(HashMap::new())
    };

    let tracked_entries = head_entries.extend(idx.clone());
    let files = fsutil::collect_files(&repo.root(), &repo, &ig)?;

    let mut idx_entries: Vec<RepoPath> = Vec::new();
    let mut un_idx_entries: Vec<RepoPath> = Vec::new();
    let mut untracked_entries: Vec<RepoPath> = Vec::new();
    for f in &files {
        let f = f.strip_prefix(&repo.root())?;
        let f_content = fsutil::read_file_to_string(&f.path())?;
        let f_object_id = ObjectId::from_bytes(f_content.as_bytes());

        if let Some(e) = head_entries.contains(&f)
            && e.object_id.hash() == f_object_id.hash()
        {
            continue;
        }

        if let Some(e) = idx.contains(&f) {
            idx_entries.push(f.clone());

            let f_metadata = std::fs::File::open(f.path())?.metadata()?;
            let f_idx = IndexEntry::new(
                f_metadata.size(),
                f.clone(),
                f_object_id,
                f_metadata.ctime_nsec(),
                f_metadata.mtime_nsec(),
            );
            if !e.is_equal(&f_idx) {
                un_idx_entries.push(f);
            }
        } else if !tracked_entries.contains(&f).is_some() {
            untracked_entries.push(f);
        } else {
            un_idx_entries.push(f);
        }
    }

    Ok(StatusResult {
        idx_entries,
        un_idx_entries,
        untracked_entries,
    })
}
