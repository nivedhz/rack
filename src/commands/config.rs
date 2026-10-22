use std::{fs, io::Write};

use anyhow::{Context, Result, bail};
use toml_edit::{DocumentMut, value};

use crate::repository;

pub fn run(key: String, input_value: String) -> Result<()> {
    let base_path = repository::find_rack_dir().context("fatal: could not find rack repository")?;
    let mut rack_config_file = fs::File::options()
        .read(true)
        .write(true)
        .open(base_path.join("config.toml"))?;
    let file_content = fs::read_to_string(base_path.join("config.toml"))?;
    let mut toml_parse: DocumentMut = file_content
        .parse()
        .context("fatal: could not parse toml")?;
    let (top_level, bottom_level) = key.split_once('.').context("fatal: invalid key")?;
    match top_level {
        "user" => match bottom_level {
            "name" => {
                toml_parse["user"]["name"] = value(input_value);
                rack_config_file
                    .write(toml_parse.to_string().as_bytes())
                    .context("fatal: could not write toml")?;
            }
            "email" => {
                toml_parse["user"]["email"] = value(input_value);
                rack_config_file
                    .write(toml_parse.to_string().as_bytes())
                    .context("fatal: could not write toml")?;
            }
            _ => {
                println!("valid keys: user.name, user.email");
                bail!("fatal: {} is not a valid key", key);
            }
        },
        "core" => match bottom_level {
            "editor" => {
                toml_parse["core"]["editor"] = value(input_value);
                rack_config_file
                    .write(toml_parse.to_string().as_bytes())
                    .context("fatal: could not write toml")?;
            }
            _ => {
                println!("valid keys: core.editor");
                bail!("fatal: {} is not a valid key", key);
            }
        },
        _ => {
            println!("valid keys: user.name, user.email, core.editor");
            bail!("fatal: {} is not a valid key", key);
        }
    }
    Ok(())
}
