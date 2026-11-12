use crate::{
    domain::{branch::BranchEntries, repository::Repository},
    error::Result,
    storage::fsutil,
};

pub fn get_entries(repo: &Repository) -> Result<BranchEntries> {
    let branch_idx_content = fsutil::read_file_to_string(&repo.branch_index()).unwrap_or_default();

    BranchEntries::from_string(branch_idx_content)
}
