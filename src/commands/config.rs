use std::env;

use crate::{
    domain::{config::Config, repository::Repository},
    error::Result,
    storage::config_file::write_config,
    storage::fsutil,
};

pub fn run(key: String, value: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let conf_content = fsutil::read_file_to_string(&repo.config())?;
    let mut conf_toml: Config = toml::from_str(&conf_content)?;

    conf_toml.change_config(key, value)?;
    write_config(&repo, conf_toml)?;
    Ok(())
}
