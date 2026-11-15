use std::path::PathBuf;

use crate::error::Result;

pub fn run(src: PathBuf, dst: PathBuf) -> Result<()> {
    dbg!(src, dst);
    Ok(())
}
