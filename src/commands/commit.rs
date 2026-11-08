use std::env;

use crate::{
    domain::{config::Config, index::IndexEntries, repository::Repository},
    error::{Error, Result},
    storage::fsutil,
};

// TODO:
// Ensure that index is not empty >>
// Ensure that there is user info >>
// Create a commit entry based on the indexed files and prev commit
// Add those entries to the commit entry dir
// - Header: <parent hash>0x20<message>0x20<author name>0x20<author email>0x20<commit
// timestamp>0x00\n
// - Body: <prev commit panty-list entries> + <panty-list entries joined by \n>
// Clear the panty-list
// Update the current branch head
pub fn run(message: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let cfg_content = fsutil::read_file_to_string(&repo.config())?;
    let cfg = Config::parse(cfg_content)?;
    if !cfg.user.name.is_some() || !cfg.user.email.is_some() {
        return Err(Error::ConfigNotFound {
            message: String::from("use rack config <key> <value> to set config details!"),
        });
    }

    if !repo.index().exists() {
        return Err(Error::NoFileAdded {
            message: String::from("use rack add <file> to insert a file"),
        });
    }
    let idx_content = fsutil::read_file_to_string(&repo.index())?;
    let idx = IndexEntries::parse(idx_content)?;
    if idx.is_empty() {
        return Err(Error::NoFileAdded {
            message: String::from("use rack add <file> to insert a file"),
        });
    }

    dbg!(&idx, &idx.is_empty(), message);
    Ok(())
}
