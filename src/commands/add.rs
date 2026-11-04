use std::path::PathBuf;

use crate::{domain::repository, error::Result, storage::object_file};

pub fn run(path: PathBuf) -> Result<()> {
    let repo = repository::Repository::discover(&path)?;
    object_file::write_blob(&repo, &path)?;

    Ok(())
}
