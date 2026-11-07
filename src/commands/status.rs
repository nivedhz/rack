use std::env;

use crate::{
    domain::{
        ignore::IgnoreEntries, index::IndexEntries, repo_path::RepoPath, repository::Repository,
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

        // TODO: take the current state of the file into consideration `idx.is_equal(lhs, rhs)`
        if idx.contains(&f).is_some() {
            idx_entries.push(f);
        } else {
            un_idx_entries.push(f);
        }
    }

    Ok(StatusResult {
        idx_entries,
        un_idx_entries,
    })
}
