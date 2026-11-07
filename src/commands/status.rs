use std::{env, os::unix::fs::MetadataExt};

use crate::{
    domain::{
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
}

pub fn run() -> Result<StatusResult> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let ig_content = fsutil::read_file_to_string(&repo.ignore_file())?;
    let ig = IgnoreEntries::parse(ig_content);

    let idx_content = fsutil::read_file_to_string(&repo.index())?;
    let idx = IndexEntries::parse(idx_content)?;

    let files = fsutil::collect_files(&repo.root(), &repo, &ig)?;

    let mut idx_entries: Vec<RepoPath> = Vec::new();
    let mut un_idx_entries: Vec<RepoPath> = Vec::new();
    for f in &files {
        let f = f.strip_prefix(&repo.root())?;

        if let Some(e) = idx.contains(&f) {
            idx_entries.push(f.clone());

            let f_content = fsutil::read_file_to_string(&f.path())?;
            let f_metadata = std::fs::File::open(f.path())?.metadata()?;
            let f_idx = IndexEntry::new(
                f_metadata.size(),
                f.clone(),
                ObjectId::from_bytes(&f_content.into_bytes()),
                f_metadata.ctime_nsec(),
                f_metadata.mtime_nsec(),
            );
            if !e.is_equal(&f_idx) {
                un_idx_entries.push(f);
            }
        } else {
            un_idx_entries.push(f);
        }
    }

    Ok(StatusResult {
        idx_entries,
        un_idx_entries,
    })
}
