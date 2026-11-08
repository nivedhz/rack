use std::env;

use crate::{
    domain::{config::Config, repository::Repository},
    error::Result,
    storage::config_file::write_config,
    storage::fsutil,
};

pub fn run(key: String, value: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let cfg_content = fsutil::read_file_to_string(&repo.config())?;
    let mut cfg = Config::parse(cfg_content)?;

    cfg.change_config(key, value)?;
    write_config(&repo, cfg)?;
    Ok(())
}
