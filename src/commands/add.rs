use std::{
    path::{Path, PathBuf},
    process,
};

use crate::constants::DIR_PATH;

pub struct AddCommand {
    path: PathBuf,
}

impl Default for AddCommand {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
        }
    }
}
impl AddCommand {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn init(&self) {
        if !Path::new(DIR_PATH).exists() {
            eprintln!("fatal: not a rack repository");
            process::exit(1);
        }
        println!("added {}", self.path.display());
    }
}
