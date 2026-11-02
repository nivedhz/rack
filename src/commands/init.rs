use anyhow::Result;
use std::path::PathBuf;

use crate::storage;

pub fn run(dir_path: PathBuf) -> Result<()> {
    storage::bootstrap::initialize(&dir_path)?;
    Ok(())
}
