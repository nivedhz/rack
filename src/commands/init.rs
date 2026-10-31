use crate::constants::DIR_PATH;
use anyhow::{Context, Result};
use std::{
    env,
    fs::{self, File, canonicalize},
    io::Write,
    path::{Path, PathBuf},
};

pub fn run(dir_path: PathBuf) -> Result<()> {
    let path = get_current_dir_path(dir_path)
        .context("fatal: could parse arg path")?
        .join(DIR_PATH);
    create_rack_dir(&path)?;
    create_baby_file(&path)?;
    create_hump_dir(&path)?;
    create_goods_dir(&path)?;
    create_rack_ignore_file(&path)?;
    create_rack_config_file(&path)?;
    Ok(())
}
fn get_current_dir_path(dir_path: PathBuf) -> Result<PathBuf> {
    if dir_path == Path::new(".") {
        Ok(env::current_dir()?)
    } else {
        Ok(canonicalize(dir_path)?)
    }
}
fn create_rack_dir(path: &Path) -> Result<()> {
    let dir_exists = fs::exists(path)? && path.is_dir();

    fs::create_dir_all(path)?;
    if dir_exists {
        println!("Reinitialized a rack repository in {}", path.display());
    } else {
        println!("Initialized a new rack repository in {}", path.display());
    }
    Ok(())
}
fn create_baby_file(path: &Path) -> Result<()> {
    let mut f = File::create(path.join("BABY"))?;
    f.write_all(b"hump: hump/baby/bonkers")?;
    Ok(())
}
fn create_hump_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path.join("hump/baby")).context("fatal: could not create hump dir")?;
    Ok(())
}
fn create_goods_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path.join("goods"))?;
    Ok(())
}
fn create_rack_ignore_file(path: &Path) -> Result<()> {
    let p = path.parent().context("could not find rack repository")?;
    const RACK_IGNORE_CONTENT: &str = "# Enter the relative or absolute paths of the files or directories you want to ignore.\ntmp/\ntarget\n.git/";
    if p.join(".rackignore").exists() {
        return Ok(());
    }
    let mut f = File::create(p.join(".rackignore"))?;
    f.write_all(RACK_IGNORE_CONTENT.as_bytes())?;
    Ok(())
}
fn create_rack_config_file(path: &Path) -> Result<()> {
    const RACK_CONFIG_CONTENT: &str = "[user]\n";
    if path.join("config.toml").exists() {
        return Ok(());
    }
    let mut file = File::create(path.join("config.toml"))?;
    file.write_all(RACK_CONFIG_CONTENT.as_bytes())?;
    Ok(())
}
