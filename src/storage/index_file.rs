use std::{fs, os::unix::fs::MetadataExt, path::Path};

use crate::{
    domain::{
        index::IndexEntries, object_id::ObjectId, repo_path::RepoPath, repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub fn write_index(repo: &Repository, path: &Path) -> Result<()> {
    let c_path = fs::canonicalize(path)?;
    let rel_path = c_path.strip_prefix(repo.root())?;

    let idx_content = fs::read_to_string(repo.index()).unwrap_or_default();
    let idx_entries = IndexEntries::parse(idx_content)?;
    if idx_entries.contains(&RepoPath::new(rel_path)) {
        return Ok(());
    }

    let f_content = fsutil::read_file_to_string(path)?;
    let f_metadata = fs::metadata(path)?;
    let obj_id = ObjectId::from_bytes(&f_content.into_bytes());

    let idx_data = format!(
        "{} {} {} {} {}\n",
        f_metadata.size(),
        rel_path.display(),
        obj_id.hash(),
        f_metadata.ctime_nsec(),
        f_metadata.mtime_nsec()
    );
    fsutil::append_to_file(&repo.index(), &idx_data.into_bytes())?;

    Ok(())
}
