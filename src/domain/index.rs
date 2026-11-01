use std::collections::BTreeMap;

use crate::domain::{object_id::ObjectId, repo_path::RepoPath, timestamp::TimeStamp};

pub struct IndexEntry {
    pub size: u64,
    pub path: RepoPath,
    pub hash: ObjectId,
    pub ctime: TimeStamp,
    pub mtime: TimeStamp,
}

pub struct Index(BTreeMap<RepoPath, IndexEntry>);
