use anyhow::{Context, Result};

use crate::{args::NewFlags, constants::DIR_PATH};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf, absolute},
};

pub struct DeflowerCommand {
    flags: NewFlags,
    dir_path: PathBuf,
}
impl DeflowerCommand {
    pub fn new(flags: NewFlags, dir_path: PathBuf) -> Self {
        Self { flags, dir_path }
    }
    fn get_path(&self) -> Result<PathBuf> {
        if self.flags.default || self.dir_path == Path::new(".") {
            Ok(env::current_dir()?)
        } else {
            Ok(absolute(&self.dir_path)?)
        }
    }
    pub fn run(&self) -> Result<()> {
        println!();
        RackFolder::new(self.get_path()?.join(DIR_PATH)).init()?;
        Ok(())
    }
}

struct RackFolder {
    path: PathBuf,
}
impl RackFolder {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
    fn init(&self) -> Result<()> {
        self.create_rack_dir()?;
        self.create_baby_file()?;
        self.create_hump_dir()?;
        self.create_goods_dir()?;
        self.create_rack_ignore_file()?;
        Ok(())
    }
    fn create_rack_dir(&self) -> Result<()> {
        let path = &self.path;
        let exist_status = fs::exists(path)? && path.is_dir();

        fs::create_dir_all(path)?;
        if exist_status {
            println!("Reinitialized a rack repository in {}", path.display());
        } else {
            println!("Initialized a new rack repository in {}", path.display());
        }
        Ok(())
    }
    fn create_baby_file(&self) -> Result<()> {
        let path = &self.path;
        let mut file = File::create(path.join("BABY"))?;
        file.write_all(b"hump: hump/baby/bonkers")?;
        Ok(())
    }
    fn create_hump_dir(&self) -> Result<()> {
        let path = &self.path;
        if fs::create_dir_all(path.join("hump/baby")).is_ok() {
            let mut file = File::create(path.join("hump/baby/bonkers"))?;
            file.write_all(b"bonkers")?;
        }
        Ok(())
    }
    fn create_goods_dir(&self) -> Result<()> {
        let path = &self.path;
        fs::create_dir_all(path.join("goods"))?;
        Ok(())
    }
    fn create_rack_ignore_file(&self) -> Result<()> {
        let path = self
            .path
            .parent()
            .context("could not find rack repository")?;
        if path.join(".rackignore").exists() {
            return Ok(());
        }
        let mut file = File::create(path.join(".rackignore"))?;
        file.write_all(b"# Enter the relative or absolute paths of the files or directories you want to ignore.\n")?;
        Ok(())
    }
}
