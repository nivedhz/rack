use crate::{
    domain::{
        branch::{BranchEntry, BranchIndexEntries},
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub fn get_entries(repo: &Repository) -> Result<BranchIndexEntries> {
    let branch_idx_content = fsutil::read_file_to_string(&repo.branch_index()).unwrap_or_default();

    BranchIndexEntries::from_string(branch_idx_content)
}

pub fn remove_entry(repo: &Repository, entry: BranchEntry) -> Result<()> {
    let mut branch_idx = get_entries(repo)?;
    branch_idx.remove(entry).unwrap_or_default();

    fsutil::write_atomic(&repo.branch_index(), &branch_idx.serialize())?;

    Ok(())
}
