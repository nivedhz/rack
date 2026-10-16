use crate::constants::DIR_PATH;
use anyhow::{Context, Result, bail};
use sha3::{Digest, Sha3_256};
use std::{
    env,
    fs::{self},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub struct InsertCommand {
    path: PathBuf,
}

impl InsertCommand {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn run(&self) -> Result<()> {
        let rack_dir = self
            .find_rack_dir()
            .context("fatal: could not find rack repository")?;
        if let Some(path_string) = self.path.to_str()
            && path_string.trim().is_empty()
        {
            bail!("fatal: empty path")
        }
        if !rack_dir.exists() {
            bail!("fatal: not a rack repository")
        }
        if self.path.is_dir() {
            self.handle_dir(self.path.to_path_buf())?
        } else {
            self.create_goods_entry(None)?;
            self.create_panty_list_entry(None)?;
        }
        Ok(())
    }
    fn find_rack_dir(&self) -> Option<PathBuf> {
        let current_dir = env::current_dir().ok()?;
        for ancestor in current_dir.ancestors() {
            if ancestor == Path::new("/") {
                return None;
            }
            let path = ancestor.join(DIR_PATH);
            if path.exists() && path.is_dir() {
                return Some(path);
            }
        }
        None
    }
    fn handle_dir(&self, path: PathBuf) -> Result<()> {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.handle_dir(path)?;
            } else {
                self.create_goods_entry(Some(path.clone()))?;
                self.create_panty_list_entry(Some(path))?;
            }
        }

        Ok(())
    }
    fn create_goods_entry(&self, path: Option<PathBuf>) -> Result<()> {
        let path = path.unwrap_or(self.path.clone());
        let file_content = fs::read(path)?;
        let mut hasher = Sha3_256::new();
        hasher.update(&file_content);
        let hash = hex::encode(hasher.finalize());
        let (dir_name, file_name) = hash.split_at(2);
        let dir_path = self
            .find_rack_dir()
            .context("fatal: could not find rack repository")?
            .join("goods")
            .join(dir_name);
        if !dir_path.exists() {
            fs::create_dir(&dir_path)?;
        }
        let compressed_file_contents = zstd::stream::encode_all(&file_content[..], 3)?;
        let file_path = dir_path.join(file_name);
        fs::write(&file_path, compressed_file_contents)?;
        Ok(())
    }
    fn create_panty_list_entry(&self, file_name: Option<PathBuf>) -> Result<()> {
        let rack_repo_dir = self
            .find_rack_dir()
            .context("fatal: could not find rack repository")?
            .parent()
            .context("fatal: could not find rack repository")?
            .to_path_buf();
        let current_dir = env::current_dir()?;
        let relative_file_name = file_name.unwrap_or(self.path.clone());
        let canonical_file_name = fs::canonicalize(&relative_file_name)?;
        let absolute_path = current_dir.join(canonical_file_name);
        let relative_path_from_root = absolute_path.strip_prefix(rack_repo_dir)?;
        let file_metadata = fs::metadata(&relative_file_name)?;
        let file_content = fs::read(&relative_file_name)?;
        let mut hasher = Sha3_256::new();
        hasher.update(&file_content);
        let hash = hex::encode(hasher.finalize());

        let entry = format!(
            "{} {} {} {} {}\n",
            file_metadata.len(),
            relative_path_from_root.to_str().unwrap_or_default(),
            hash,
            file_metadata.ctime_nsec(),
            file_metadata.mtime_nsec()
        );
        let panty_list_path = self
            .find_rack_dir()
            .context("fatal: could not find rack repository")?
            .join("panty-list");
        let mut panty_list_file = fs::File::options()
            .create(true)
            .read(true)
            .append(true)
            .truncate(false)
            .open(&panty_list_path)?;
        let mut read_buf: Vec<u8> = Vec::new();
        panty_list_file.read_to_end(&mut read_buf)?;
        let file_content = String::from_utf8_lossy(&read_buf[..]).to_string();
        for contents in file_content.trim().lines() {
            let file_name = contents.split_whitespace().nth(1).unwrap_or_default();
            let m_time = contents.split_whitespace().nth(4).unwrap_or_default();
            if file_name == relative_file_name.to_str().unwrap_or_default() {
                if m_time == file_metadata.mtime_nsec().to_string() {
                    println!("info: file already added");
                    return Ok(());
                } else {
                    let new_content = file_content.trim().replace(contents, entry.as_str());
                    fs::write(panty_list_path, new_content.as_bytes())?;
                    return Ok(());
                }
            }
        }
        panty_list_file.write_all(entry.as_bytes())?;
        Ok(())
    }
}
