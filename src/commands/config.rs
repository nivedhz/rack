use crate::repository;
use anyhow::{Context, Result, bail};
use std::fs;
use toml_edit::{DocumentMut, Item, Table, value};

pub fn run(key: String, input_value: String) -> Result<()> {
    let rack_dir = repository::find_rack_dir().context("fatal: could not find rack repository")?;
    let config_path = rack_dir.join("config.toml");
    let file_content = fs::read_to_string(&config_path)?;
    let mut rack_config_file_toml: DocumentMut = file_content
        .parse()
        .context("fatal: could not parse toml")?;
    if let Some(item) = rack_config_file_toml.get_mut("user")
        && let Some(inline_table) = item.as_inline_table()
    {
        let mut table = Table::new();

        for (k, v) in inline_table {
            table[k] = v.clone().into()
        }
        *item = Item::Table(table)
    }
    let (section, section_key) = key.split_once('.').context(format!(
        "valid keys: user.name, user.email\nfatal: {} is not a valid key",
        key
    ))?;
    match section {
        "user" => match section_key {
            "name" => {
                rack_config_file_toml["user"]["name"] = value(input_value.trim());
            }
            "email" => {
                rack_config_file_toml["user"]["email"] = value(input_value.trim());
            }
            _ => {
                println!("valid keys: user.name, user.email");
                bail!("fatal: {} is not a valid key", key);
            }
        },
        _ => {
            println!("valid keys: user.name, user.email");
            bail!("fatal: {} is not a valid key", key);
        }
    }
    println!("{} ", rack_config_file_toml);
    fs::write(&config_path, rack_config_file_toml.to_string())?;
    Ok(())
}
