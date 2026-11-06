use core::fmt;
use std::{collections::BTreeMap, path::Path};

use itertools::Itertools;

use crate::{
    domain::{object_id::ObjectId, repo_path::RepoPath},
    error::Result,
};

pub struct IndexEntry {
    pub size: u64,
    pub path: RepoPath,
    pub object_id: ObjectId,
    pub ctime: u64,
    pub mtime: u64,
}
impl IndexEntry {
    fn parse(
        size: &str,
        path: &str,
        object_id: &str,
        ctime: &str,
        mtime: &str,
    ) -> Result<IndexEntry> {
        let size: u64 = size.parse()?;
        let path = RepoPath::new(path);
        let object_id = ObjectId::from_bytes(&object_id.to_string().into_bytes());
        let ctime: u64 = ctime.parse()?;
        let mtime: u64 = mtime.parse()?;

        Ok(IndexEntry {
            size,
            path,
            object_id,
            ctime,
            mtime,
        })
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
