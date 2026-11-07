use core::fmt;
use std::path::{Path, PathBuf};

use crate::error::Result;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct RepoPath {
    path: PathBuf,
}

impl RepoPath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        RepoPath { path }
    }
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn strip_prefix(&self, pref: &Path) -> Result<Self> {
        Ok(RepoPath {
            path: self.path.strip_prefix(pref)?.to_path_buf(),
        })
    }
}
impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
