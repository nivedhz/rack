use crate::{
    domain::{ignore::IgnoreEntries, repository::Repository},
    error::Result,
    storage::fsutil,
};

pub fn get_ignored(repo: &Repository) -> Result<IgnoreEntries> {
    let ig_file_content = fsutil::read_file_to_string(&repo.ignore_file())?;
    Ok(IgnoreEntries::from_string(ig_file_content))
}
