use core::fmt;
use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{
    domain::{object_id::ObjectId, repo_path::RepoPath},
    error::Result,
};

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
        let object_id = ObjectId::from_bytes(&object_id.to_string().into_bytes());
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
}
pub struct IndexEntries {
    entries: BTreeMap<RepoPath, IndexEntry>,
}
impl IndexEntries {
    pub fn parse(idx_content: String) -> Result<IndexEntries> {
        let mut entries: BTreeMap<RepoPath, IndexEntry> = BTreeMap::new();
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
    pub fn contains(&self, idx_path: &RepoPath) -> bool {
        self.entries.contains_key(idx_path)
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
