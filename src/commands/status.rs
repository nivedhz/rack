use colored::*;
use std::{
    fs,
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
        let current_dir = match std::env::current_dir() {
            Ok(current_dir) => current_dir,
            Err(_) => {
                eprintln!("fatal: could not get current directory");
                process::exit(1)
            }
        };
        let rack_folder = match self.find_upwards(&current_dir, DIR_PATH) {
            Some(rack_folder) => rack_folder,
            None => {
                eprintln!("fatal: not inside a rack repository");
                process::exit(1)
            }
        };
        let rack_repository_dir = match rack_folder.parent() {
            Some(rack_repository_dir) => rack_repository_dir,
            None => {
                eprintln!("fatal: could not find rack repository");
                process::exit(1)
            }
        };
        let mut files: Vec<FileStatus> = Vec::new();
        let mut ignore_dirs: Vec<PathBuf> =
            self.read_rack_ignore().iter().map(PathBuf::from).collect();
        ignore_dirs.push(PathBuf::from(DIR_PATH));
        let indexed_files: Vec<PathBuf> =
            self.get_indexed_files().iter().map(PathBuf::from).collect();
        self.get_file_recursively(rack_repository_dir, &mut files, ignore_dirs, indexed_files);
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
        }
        println!();
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
            Ok(entries) => entries,
            Err(_) => {
                eprintln!("fatal: could not read directory");
                process::exit(1)
            }
        };
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    eprintln!("fatal: could not read directory");
                    process::exit(1)
                }
            };
            let path = entry.path();
            if path == Path::new("target") || path == Path::new(".git") {
                continue;
            } else {
                if path.is_dir() {
                    if let Some(dir_name) = path.file_name()
                        && ignore_dirs.contains(&PathBuf::from(dir_name))
                    {
                        continue;
                    }
                    self.get_file_recursively(
                        &path,
                        files,
                        ignore_dirs.clone(),
                        indexed_files.clone(),
                    );
                } else {
                    let path_file_name = match path.file_name() {
                        Some(path_file_name) => path_file_name,
                        None => {
                            eprintln!("fatal: could not read directory");
                            process::exit(1)
                        }
                    };
                    files.push(FileStatus {
                        is_indexed: indexed_files.contains(&PathBuf::from(path_file_name)),
                        file_path: PathBuf::from(path_file_name),
                    });
                }
            }
        }
    }
    fn find_upwards(&self, start_dir: &Path, target_name: &str) -> Option<PathBuf> {
        for ancestor in start_dir.ancestors() {
            if ancestor == Path::new("/") {
                return None;
            }
            let path = ancestor.join(target_name);
            if path.exists() && path.is_dir() {
                return Some(path);
            }
        }
        None
    }
    fn read_rack_ignore(&self) -> Vec<String> {
        let value = match fs::read_to_string(".rackignore") {
            Ok(value) => value,
            Err(_) => return Vec::new(),
        };
        let entries: Vec<&str> = value.split("\n").collect();
        entries.iter().map(|s| s.to_string()).collect()
    }
    fn get_indexed_files(&self) -> Vec<String> {
        let value = match fs::read_to_string(Path::new(DIR_PATH).join("panty-list")) {
            Ok(value) => value,
            Err(_) => return Vec::new(),
        };
        value
            .trim()
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.split(" ").collect::<Vec<&str>>()[1].to_string())
            .collect::<Vec<String>>()
    }
}
