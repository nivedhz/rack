use std::env;

use crate::{
    domain::repository::Repository,
    error::Result,
    storage::config_file::{self, write_config},
};

pub fn run(key: String, value: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let mut cfg = config_file::get_confit(&repo)?;

    cfg.change_config(key, value)?;
    write_config(&repo, cfg)?;
    Ok(())
}
