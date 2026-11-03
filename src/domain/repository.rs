use std::path::PathBuf;

use crate::storage::layout;

pub struct Repository {
    pub root: PathBuf,
}

impl Repository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
    pub fn rack_dir(&self) -> PathBuf {
        layout::rack_dir(&self.root)
    }
}
