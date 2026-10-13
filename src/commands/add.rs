use crate::constants::DIR_PATH;
use sha3::{Digest, Sha3_256};
use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

pub struct AddCommand {
    path: PathBuf,
}

impl AddCommand {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn init(&self) {
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
    }
    fn create_goods_entry(&self) {
        let file_name = self.path.clone();
        let file_content = match fs::read(&file_name) {
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
}
