use std::path::PathBuf;

use anyhow::Result;

pub fn run(path: PathBuf, staged: bool) -> Result<()> {
    dbg!(path, staged);
    Ok(())
}
