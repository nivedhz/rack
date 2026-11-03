use std::path::PathBuf;

use crate::{domain::repository, error::Result};

pub fn run(path: PathBuf) -> Result<()> {
    let repo = repository::Repository::discover(path.clone())?;
    dbg!(repo.root());
    Ok(())
}
