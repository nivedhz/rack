use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

use sha3::{Digest, Sha3_256};

use crate::constants::DIR_PATH;

pub struct AddCommand {
    path: PathBuf,
}

impl AddCommand {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn init(&self) {
        if self.path.to_str().unwrap().trim().is_empty() {
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
        let file_content = fs::read(file_name).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&file_content);
        let hash = hasher.finalize();
        let (dir_name, file_name) = hash.split_at(4);
        let dir_path = Path::new(DIR_PATH).join(Path::new("goods").join(hex::encode(dir_name)));
        if !dir_path.exists() {
            fs::create_dir(&dir_path).unwrap();
        }
        let file_path = dir_path.join(hex::encode(file_name));
        fs::write(file_path, file_content).unwrap();
    }
}
