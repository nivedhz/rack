use core::fmt;
use std::path::PathBuf;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct RepoPath {
    path: PathBuf,
}

impl RepoPath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        RepoPath { path }
    }
}
impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
