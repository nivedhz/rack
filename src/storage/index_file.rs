use std::{fs, os::unix::fs::MetadataExt, path::Path};

use crate::{
    domain::{
        index::{IndexEntries, IndexEntry},
        object_id::ObjectId,
        repo_path::RepoPath,
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub fn write_index(repo: &Repository, path: &Path) -> Result<()> {
    let c_path = fs::canonicalize(path)?;
    let rel_path = RepoPath::new(c_path.strip_prefix(repo.root())?);

    let idx_content = fs::read_to_string(repo.index()).unwrap_or_default();
    let idx_entries = IndexEntries::parse(idx_content)?;

    if let Some(e_idx) = idx_entries.contains(&rel_path) {
        // TODO: this is recurring so extract into a function of some sorts
        let f_content = fsutil::read_file_to_string(path)?;
        let f_metadata = fs::metadata(path)?;

        let f_idx = IndexEntry::new(
            f_metadata.size(),
            rel_path,
            ObjectId::from_bytes(&f_content.into_bytes()),
            f_metadata.ctime_nsec(),
            f_metadata.mtime_nsec(),
        );
        if e_idx.is_equal(&f_idx) {
            return Ok(());
        } else {
            // TODO: replace the current idx entry with the new one
            dbg!(idx_entries.entries());
            todo!("replace the current idx entry with the new one");
        }
    }

    let f_content = fsutil::read_file_to_string(path)?;
    let f_metadata = fs::metadata(path)?;
    let obj_id = ObjectId::from_bytes(&f_content.into_bytes());

    let idx_data = IndexEntry::new(
        f_metadata.size(),
        rel_path,
        obj_id,
        f_metadata.ctime_nsec(),
        f_metadata.mtime_nsec(),
    );

    fsutil::append_atomic(&repo.index(), &idx_data.serialize())?;

    Ok(())
}
