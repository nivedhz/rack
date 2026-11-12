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
pub fn get_confit(repo: &Repository) -> Result<Config> {
    let cfg_content = fsutil::read_file_to_string(&repo.config())?;

    Config::parse(cfg_content)
}
