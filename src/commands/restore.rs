use std::path::PathBuf;

use anyhow::Result;

pub fn run(path: PathBuf) -> Result<()> {
    println!("{}", path.display());
    Ok(())
}
