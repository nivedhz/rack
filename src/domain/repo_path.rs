use core::fmt;
use std::path::PathBuf;

use crate::error::Result;

#[derive(Clone)]
pub struct RepoPath {
    path: PathBuf,
}

impl RepoPath {
    fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        Ok(RepoPath { path })
    }
}
impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
