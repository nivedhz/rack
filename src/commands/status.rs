use anyhow::{Context, Result};
use colored::*;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::constants::DIR_PATH;
#[derive(Debug)]
struct FileStatus {
    file_path: PathBuf,
    is_indexed: bool,
}

pub struct ExposeCommand;
impl ExposeCommand {
    pub fn new() -> Self {
        ExposeCommand
    }
    pub fn run(&self) -> Result<()> {
        self.get_all_files()?;
        Ok(())
    }
    fn get_all_files(&self) -> Result<()> {
        let rack_folder = self
            .find_rack_dir()
            .context("fatal: could not find rack repository")?
            .parent()
            .context("fatal: could not find rack repository")?
            .to_path_buf();

        let ignore_dirs: Vec<PathBuf> =
            self.read_rack_ignore()?.iter().map(PathBuf::from).collect();

        let indexed_files: Vec<PathBuf> = self
            .get_indexed_files()?
            .iter()
            .map(PathBuf::from)
            .collect();

        let mut files: Vec<FileStatus> = Vec::new();
        self.get_file_recursively(&rack_folder, &mut files, ignore_dirs, indexed_files)?;

        let unindexed_files = files
            .iter()
            .filter(|f| !f.is_indexed)
            .collect::<Vec<&FileStatus>>();
        let indexed_files = files
            .iter()
            .filter(|f| f.is_indexed)
            .collect::<Vec<&FileStatus>>();

        println!();
        if !indexed_files.is_empty() {
            println!("Changes to be commited:");
            for file in indexed_files {
                println!(
                    "      {}    {}",
                    "modified:".green(),
                    file.file_path.display().to_string().green()
                );
            }
            println!();
        }
        if !unindexed_files.is_empty() {
            println!("Changes not staged for commit:");
            for file in unindexed_files {
                println!(
                    "      {}    {}",
                    "modified:".red(),
                    file.file_path.display().to_string().red()
                );
            }
        }
        Ok(())
    }
    fn get_file_recursively(
        &self,
        dir: &Path,
        files: &mut Vec<FileStatus>,
        ignore_dirs: Vec<PathBuf>,
        indexed_files: Vec<PathBuf>,
    ) -> Result<()> {
        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let base_path = self
                .find_rack_dir()
                .context("fatal: could not find rack repository")?
                .parent()
                .context("fatal: could not find rack repository")?
                .to_path_buf();
            let relative_path = path.strip_prefix(base_path)?;
            if let Some(path_name) = path.file_name()
                && (ignore_dirs.contains(&PathBuf::from(path_name))
                    || ignore_dirs.contains(&PathBuf::from(relative_path)))
            {
                continue;
            } else {
                if path.is_dir() {
                    self.get_file_recursively(
                        &path,
                        files,
                        ignore_dirs.clone(),
                        indexed_files.clone(),
                    )?;
                } else {
                    files.push(FileStatus {
                        is_indexed: indexed_files.contains(&PathBuf::from(relative_path)),
                        file_path: PathBuf::from(relative_path),
                    });
                }
            }
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
    fn read_rack_ignore(&self) -> Result<Vec<String>> {
        let rack_repo_root_path = self
            .find_rack_dir()
            .context("fatal: could not find rack repository")?
            .parent()
            .context("fatal: could not find rack repository")?
            .to_path_buf();
        let value = fs::read_to_string(rack_repo_root_path.join(".rackignore")).unwrap_or_default();
        let mut entries: Vec<&str> = value.trim().lines().collect();
        entries.push(DIR_PATH);
        Ok(entries.iter().map(|s| s.to_string()).collect())
    }
    fn get_indexed_files(&self) -> Result<Vec<String>> {
        let rack_dir = self
            .find_rack_dir()
            .context("fatal: could not find rack repository")?;
        if !rack_dir.join("panty-list").exists() {
            fs::File::create(rack_dir.join("panty-list"))?;
            return Ok(Vec::new());
        }
        let value = fs::read_to_string(rack_dir.join("panty-list")).unwrap_or_default();
        if value.trim().is_empty() {
            return Ok(Vec::new());
        }
        Ok(value
            .trim()
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.split(" ").collect::<Vec<&str>>()[1].to_string())
            .collect::<Vec<String>>())
    }
}
