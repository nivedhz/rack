use std::{collections::HashMap, env, os::unix::fs::MetadataExt};

use crate::{
    domain::{
        index::{IndexEntries, IndexEntry},
        repo_path::RepoPath,
        repository::Repository,
    },
    error::Result,
    storage::{baby_file, commit_file, fsutil, head_file, ignore_file, index_file, object_file},
};

pub struct StatusResult {
    pub idx_entries: Vec<RepoPath>,
    pub un_idx_entries: Vec<RepoPath>,
    pub untracked_entries: Vec<RepoPath>,
}

pub fn run() -> Result<StatusResult> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let ig = ignore_file::get_ignored(&repo)?;
    let idx = index_file::get_index(&repo)?;
    let baby = baby_file::get_baby(&repo);

    let head_file = repo.head(&baby.path_name().path());
    let head_entries = if head_file.exists() {
        let head_commit_id = head_file::get_commit(&head_file)?;
        let commit_entry = commit_file::get_entry(&repo, &head_commit_id)?;

        commit_entry.entries()
    } else {
        IndexEntries::new(HashMap::new())
    };

    let tracked_entries = head_entries.extend(idx.clone());
    let files = fsutil::collect_files(&repo.root(), &repo, &ig)?;

    let mut idx_entries: Vec<RepoPath> = Vec::new();
    let mut un_idx_entries: Vec<RepoPath> = Vec::new();
    let mut untracked_entries: Vec<RepoPath> = Vec::new();
    for f in files {
        let f_object_id = object_file::get_object_id(&f.path())?;

        let rel_f = f.strip_prefix(&repo.root())?;
        if let Some(e) = head_entries.contains(&rel_f)
            && e.object_id.hash() == f_object_id.hash()
        {
            continue;
        }

        if let Some(e) = idx.contains(&rel_f) {
            idx_entries.push(rel_f.clone());

            let f_metadata = std::fs::File::open(f.path())?.metadata()?;
            let f_idx = IndexEntry::new(
                f_metadata.size(),
                rel_f,
                f_object_id,
                f_metadata.ctime_nsec(),
                f_metadata.mtime_nsec(),
            );
            if !e.is_equal(&f_idx) {
                un_idx_entries.push(f_idx.path);
            }
        } else if !tracked_entries.contains(&rel_f).is_some() {
            untracked_entries.push(rel_f);
        } else {
            un_idx_entries.push(rel_f);
        }
    }

    Ok(StatusResult {
        idx_entries,
        un_idx_entries,
        untracked_entries,
    })
}
