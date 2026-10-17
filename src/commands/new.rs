use crate::constants::DIR_PATH;
use anyhow::{Context, Result};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf, absolute},
};

pub fn run(dir_path: PathBuf) -> Result<()> {
    println!();
    let path = get_path(dir_path)
        .context("fatal: could parse arg path")?
        .join(DIR_PATH);
    create_rack_dir(&path).context("fatal: could not create rack directory")?;
    create_baby_file(&path).context("fatal: could not create baby file")?;
    create_hump_dir(&path).context("fatal: could not create hump directory")?;
    create_goods_dir(&path).context("fatal: could not create goods directory")?;
    create_rack_ignore_file(&path).context("fatal: could not create rackignore file")?;
    Ok(())
}
fn get_path(dir_path: PathBuf) -> Result<PathBuf> {
    if dir_path == Path::new(".") {
        Ok(env::current_dir()?)
    } else {
        Ok(absolute(dir_path)?)
    }
}
fn create_rack_dir(path: &Path) -> Result<()> {
    let exist_status = fs::exists(path)? && path.is_dir();

    fs::create_dir_all(path)?;
    if exist_status {
        println!("Reinitialized a rack repository in {}", path.display());
    } else {
        println!("Initialized a new rack repository in {}", path.display());
    }
    Ok(())
}
fn create_baby_file(path: &Path) -> Result<()> {
    let mut file = File::create(path.join("BABY"))?;
    file.write_all(b"hump: hump/baby/bonkers")?;
    Ok(())
}
fn create_hump_dir(path: &Path) -> Result<()> {
    if fs::create_dir_all(path.join("hump/baby")).is_ok() {
        let mut file = File::create(path.join("hump/baby/bonkers"))?;
        file.write_all(b"bonkers")?;
    }
    Ok(())
}
fn create_goods_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path.join("goods"))?;
    Ok(())
}
fn create_rack_ignore_file(path: &Path) -> Result<()> {
    let path = path.parent().context("could not find rack repository")?;
    if path.join(".rackignore").exists() {
        return Ok(());
    }
    let mut file = File::create(path.join(".rackignore"))?;
    file.write_all(b"# Enter the relative or absolute paths of the files or directories you want to ignore.\n# tmp/\n# target\n# .git/")?;
    Ok(())
}
