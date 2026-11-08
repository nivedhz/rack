use crate::{
    domain::{config::Config, repository::Repository},
    error::Result,
    storage::fsutil,
};

pub fn write_config(repo: &Repository, conf: Config) -> Result<()> {
    let output = toml::to_string_pretty(&conf)?;

    fsutil::write_atomic(&repo.config(), output.as_bytes())?;
    Ok(())
}
