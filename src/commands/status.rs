use colored::*;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
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
    pub fn run(&self) {
        self.get_all_files();
    }
    fn get_all_files(&self) {
        let rack_folder = match self.find_rack_dir() {
            Some(f) => match f.parent() {
                Some(p) => &p.to_path_buf(),
                None => {
                    eprintln!("fatal: could not find rack repository");
                    process::exit(1)
                }
            },
            None => {
                eprintln!("fatal: not inside a rack repository");
                process::exit(1)
            }
        };

        let mut ignore_dirs: Vec<PathBuf> =
            self.read_rack_ignore().iter().map(PathBuf::from).collect();
        ignore_dirs.push(PathBuf::from(DIR_PATH));

        let indexed_files: Vec<PathBuf> =
            self.get_indexed_files().iter().map(PathBuf::from).collect();

        let mut files: Vec<FileStatus> = Vec::new();
        self.get_file_recursively(rack_folder, &mut files, ignore_dirs, indexed_files);

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
    }
    fn get_file_recursively(
        &self,
        dir: &Path,
        files: &mut Vec<FileStatus>,
        ignore_dirs: Vec<PathBuf>,
        indexed_files: Vec<PathBuf>,
    ) {
        let entries = match fs::read_dir(dir) {
            Ok(r) => r,
            Err(_) => {
                eprintln!("fatal: could not read directory");
                process::exit(1)
            }
        };
        for entry in entries {
            let entry = match entry {
                Ok(de) => de,
                Err(_) => {
                    eprintln!("fatal: could not read directory");
                    process::exit(1)
                }
            };
            let path = entry.path();
            let base_path = match self.find_rack_dir() {
                Some(p) => match p.parent() {
                    Some(p) => p.to_path_buf(),
                    None => {
                        eprintln!("fatal: could not find rack repository");
                        process::exit(1)
                    }
                },
                None => {
                    eprintln!("fatal: could not find rack repository");
                    process::exit(1)
                }
            };
            let relative_path = match path.strip_prefix(base_path) {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("fatal: coult not get relative path");
                    process::exit(1)
                }
            };
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
                    );
                } else {
                    files.push(FileStatus {
                        is_indexed: indexed_files.contains(&PathBuf::from(relative_path)),
                        file_path: PathBuf::from(relative_path),
                    });
                }
            }
        }
    }
    fn find_rack_dir(&self) -> Option<PathBuf> {
        let current_dir = match env::current_dir() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("fatal: could not get current directory");
                process::exit(1)
            }
        };
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
    fn read_rack_ignore(&self) -> Vec<String> {
        let rack_repo_root_path = match self.find_rack_dir() {
            Some(f) => match f.parent() {
                Some(p) => p.to_path_buf(),
                None => {
                    eprintln!("fatal: could not find rack repository");
                    process::exit(1)
                }
            },
            None => {
                eprintln!("fatal: not inside a rack repository");
                process::exit(1)
            }
        };
        let value = match fs::read_to_string(rack_repo_root_path.join(".rackignore")) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let entries: Vec<&str> = value.trim().split("\n").collect();
        entries.iter().map(|s| s.to_string()).collect()
    }
    fn get_indexed_files(&self) -> Vec<String> {
        let rack_dir = match self.find_rack_dir() {
            Some(f) => f,
            None => {
                eprintln!("fatal: not inside a rack repository");
                process::exit(1)
            }
        };
        let value = match fs::read_to_string(rack_dir.join("panty-list")) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        if value.trim().is_empty() {
            return Vec::new();
        }
        value
            .trim()
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.split(" ").collect::<Vec<&str>>()[1].to_string())
            .collect::<Vec<String>>()
    }
}
