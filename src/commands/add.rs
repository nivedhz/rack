use crate::constants::DIR_PATH;
use sha3::{Digest, Sha3_256};
use std::{
    fs::{self},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process,
};

pub struct InsertCommand {
    path: PathBuf,
}

impl InsertCommand {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn run(&self) {
        if let Some(path_string) = self.path.to_str()
            && path_string.trim().is_empty()
        {
            eprintln!("fatal: empty path");
            process::exit(1);
        }
        if !Path::new(DIR_PATH).exists() {
            eprintln!("fatal: not a rack repository");
            process::exit(1);
        }
        self.create_goods_entry();
        self.create_panty_list_entry();
    }
    fn create_goods_entry(&self) {
        let file_name = &self.path;
        let file_content = match fs::read(file_name) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("fatal: {}", e);
                process::exit(1);
            }
        };
        let mut hasher = Sha3_256::new();
        hasher.update(&file_content);
        let hash = hex::encode(hasher.finalize());
        let (dir_name, file_name) = hash.split_at(2);
        let dir_path = Path::new(DIR_PATH).join(Path::new("goods").join(dir_name));
        if !dir_path.exists() {
            fs::create_dir(&dir_path).unwrap_or_else(|e| {
                eprintln!("fatal: {}", e);
                process::exit(1);
            });
        }
        let compressed_file_contents = match zstd::stream::encode_all(&file_content[..], 3) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("fatal: {}", e);
                process::exit(1);
            }
        };
        let file_path = dir_path.join(file_name);
        fs::write(&file_path, compressed_file_contents).unwrap_or_else(|e| {
            eprintln!("fatal: {}", e);
            process::exit(1);
        });
    }
    fn create_panty_list_entry(&self) {
        let relative_file_name = &self.path;
        let file_metadata = fs::metadata(relative_file_name).unwrap_or_else(|e| {
            eprintln!("fatal: {}", e);
            process::exit(1);
        });
        let file_content = match fs::read(relative_file_name) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("fatal: {}", e);
                process::exit(1);
            }
        };
        let mut hasher = Sha3_256::new();
        hasher.update(&file_content);
        let hash = hex::encode(hasher.finalize());

        let entry = format!(
            "{} {} {} {} {}\n",
            file_metadata.len(),
            relative_file_name.to_str().unwrap_or_default(),
            hash,
            file_metadata.ctime_nsec(),
            file_metadata.mtime_nsec()
        );
        let mut panty_list_file = fs::File::options()
            .create(true)
            .read(true)
            .append(true)
            .truncate(false)
            .open(Path::new(DIR_PATH).join("panty-list"))
            .unwrap_or_else(|e| {
                eprintln!("fatal: {}", e);
                process::exit(1);
            });
        let mut read_buf: Vec<u8> = Vec::new();
        panty_list_file
            .read_to_end(&mut read_buf)
            .unwrap_or_else(|e| {
                eprintln!("fatal: {}", e);
                process::exit(1);
            });
        let file_content = String::from_utf8_lossy(&read_buf[..]).to_string();
        for contents in file_content.trim().split("\n") {
            let file_name = contents.split(" ").nth(1).unwrap_or_default();
            let m_time = contents.split(" ").nth(4).unwrap_or_default();
            if file_name == relative_file_name.to_str().unwrap_or_default() {
                if m_time == file_metadata.mtime_nsec().to_string() {
                    eprintln!("info: file already added");
                    return;
                } else {
                    let new_content = file_content.trim().replace(contents, entry.as_str());
                    fs::write(
                        Path::new(DIR_PATH).join("panty-list"),
                        new_content.as_bytes(),
                    )
                    .unwrap_or_else(|e| {
                        eprintln!("fatal: {}", e);
                        process::exit(1);
                    });
                    return;
                }
            }
        }
        panty_list_file
            .write_all(entry.as_bytes())
            .unwrap_or_else(|e| {
                eprintln!("fatal: {}", e);
                process::exit(1);
            });
    }
}
