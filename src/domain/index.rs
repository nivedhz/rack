use core::fmt;
use std::collections::BTreeMap;

use crate::domain::{object_id::ObjectId, repo_path::RepoPath};

pub struct IndexEntry {
    pub size: u64,
    pub path: RepoPath,
    pub object_id: ObjectId,
    pub ctime: u64,
    pub mtime: u64,
}
pub struct IndexEntries {
    entries: BTreeMap<String, IndexEntry>,
}

impl fmt::Display for IndexEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.size, self.path, self.object_id, self.ctime, self.mtime
        )
    }
}
