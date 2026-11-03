use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    error::{self, Result},
    storage::layout,
};

pub struct Repository {
    pub root: PathBuf,
}

impl Repository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn discover(root: PathBuf) -> Result<Self> {
        let curr_path = env::current_dir()?.join(root);
        for ancestor in curr_path.ancestors() {
            let rack_path = layout::rack_dir(ancestor);
            if rack_path.exists() && rack_path.is_dir() {
                return Ok(Repository::new(ancestor.to_path_buf()));
            }
            if ancestor == Path::new("/") {
                return Err(error::Error::NotInRackRepo);
            }
        }
        Err(error::Error::NotInRackRepo)
    }
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
    pub fn rack_dir(&self) -> PathBuf {
        layout::rack_dir(&self.root)
    }
    pub fn index(&self) -> PathBuf {
        layout::index(&self.root)
    }
    pub fn head(&self) -> PathBuf {
        layout::head(&self.root)
    }
    pub fn baby(&self) -> PathBuf {
        layout::baby(&self.root)
    }
    pub fn ignore_file(&self) -> PathBuf {
        layout::ignore_file(&self.root)
    }
    pub fn objects_dir(&self) -> PathBuf {
        layout::objects_dir(&self.root)
    }
    pub fn commits_dir(&self) -> PathBuf {
        layout::commits_dir(&self.root)
    }
    pub fn config(&self) -> PathBuf {
        layout::config(&self.root)
    }
}
