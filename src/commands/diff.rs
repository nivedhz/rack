use std::path::PathBuf;

use crate::error::Result;

// TODO:
// Get the diff of the current state of file with the latest change of the file (panty-list entries
// have more priority over HEAD entries)
pub fn run(path: PathBuf) -> Result<Vec<String>> {
    dbg!(path);
    Ok(Vec::new())
}
