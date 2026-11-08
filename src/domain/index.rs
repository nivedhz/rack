use core::fmt;
use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    domain::{object_id::ObjectId, repo_path::RepoPath},
    error::Result,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IndexEntry {
    pub size: u64,
    pub path: RepoPath,
    pub object_id: ObjectId,
    pub ctime: i64,
    pub mtime: i64,
}
impl IndexEntry {
    pub fn new(size: u64, path: RepoPath, object_id: ObjectId, ctime: i64, mtime: i64) -> Self {
        Self {
            size,
            path,
            object_id,
            ctime,
            mtime,
        }
    }
    pub fn parse(
        size: &str,
        path: &str,
        object_id: &str,
        ctime: &str,
        mtime: &str,
    ) -> Result<IndexEntry> {
        let size: u64 = size.parse()?;
        let path = RepoPath::new(path);
        let object_id = ObjectId::new(object_id.to_string());
        let ctime: i64 = ctime.parse()?;
        let mtime: i64 = mtime.parse()?;

        Ok(IndexEntry {
            size,
            path,
            object_id,
            ctime,
            mtime,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        // <size> <relative path> <hash> <ctime nsec> <mtime nsec>\n
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.size.to_string().into_bytes());
        bytes.push(0x20);
        bytes.extend_from_slice(&self.path.to_string().into_bytes());
        bytes.push(0x20);
        bytes.extend_from_slice(&self.object_id.hash().into_bytes());
        bytes.push(0x20);
        bytes.extend_from_slice(&self.ctime.to_string().into_bytes());
        bytes.push(0x20);
        bytes.extend_from_slice(&self.mtime.to_string().into_bytes());
        bytes.push(0x0A);

        bytes
    }
    pub fn is_equal(&self, other: &IndexEntry) -> bool {
        self.path == other.path && self.object_id == other.object_id
    }
}
#[derive(Debug, Clone)]
pub struct IndexEntries {
    entries: HashMap<RepoPath, IndexEntry>,
}
impl IndexEntries {
    pub fn entries(&self) -> HashMap<RepoPath, IndexEntry> {
        self.entries.clone()
    }
    pub fn parse(idx_content: String) -> Result<IndexEntries> {
        let mut entries: HashMap<RepoPath, IndexEntry> = HashMap::new();

        for entry in idx_content.lines() {
            let (size, path, object_id, ctime, mtime) =
                entry.split_whitespace().collect_tuple().unwrap_or_default();

            entries.insert(
                RepoPath::new(path),
                IndexEntry::parse(size, path, object_id, ctime, mtime)?,
            );
        }
        Ok(IndexEntries { entries })
    }
    pub fn serialize(&self) -> Vec<u8> {
        // <entries joined by \n>
        let mut bytes: Vec<u8> = Vec::new();

        for (_, e) in self.entries() {
            bytes.extend_from_slice(&e.serialize());
        }

        bytes
    }
    pub fn contains(&self, idx_path: &RepoPath) -> Option<IndexEntry> {
        self.entries.get(idx_path).cloned()
    }
    pub fn modify(&mut self, modified: &IndexEntry) {
        if let Some(e) = self.entries.get_mut(&modified.path) {
            *e = modified.to_owned();
        }
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn extend(&self, other: IndexEntries) -> IndexEntries {
        let extended: HashMap<RepoPath, IndexEntry> = self
            .entries
            .clone()
            .into_iter()
            .chain(other.entries)
            .collect();

        IndexEntries { entries: extended }
    }
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
