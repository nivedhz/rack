use std::collections::HashMap;

use crate::{
    domain::{commit::CommitId, repo_path::RepoPath},
    error::Result,
};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct BranchEntry {
    name: RepoPath,
}
impl BranchEntry {
    fn new(name: RepoPath) -> Self {
        BranchEntry { name }
    }
    pub fn name(&self) -> RepoPath {
        self.name.clone()
    }
}

#[derive(Debug)]
pub struct BranchEntries {
    entries: HashMap<BranchEntry, CommitId>,
}

impl BranchEntries {
    pub fn parse(file_content: String) -> Result<BranchEntries> {
        let lines = file_content.lines();
        let mut entries: HashMap<BranchEntry, CommitId> = HashMap::new();

        for l in lines {
            let (tail, name) = l.split_once(" ").unwrap_or_default();
            let tail = CommitId::new(tail.to_string())?;
            let name = RepoPath::new(name);

            entries.insert(BranchEntry::new(name), tail);
        }

        Ok(BranchEntries { entries })
    }
    pub fn entries(&self) -> HashMap<BranchEntry, CommitId> {
        self.entries.clone()
    }
}

pub struct BranchIndexEntry {
    head: CommitId,
    name: RepoPath,
}
impl BranchIndexEntry {
    pub fn new(head: CommitId, name: RepoPath) -> Self {
        BranchIndexEntry { head, name }
    }
    pub fn head(&self) -> CommitId {
        self.head.clone()
    }
    pub fn name(&self) -> RepoPath {
        self.name.clone()
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.head.serialize());
        bytes.push(0x20);
        bytes.extend_from_slice(&self.name.serialize());
        bytes.push(0x0A);

        bytes
    }
}

pub struct BabyEntry {
    path: RepoPath,
}

impl BabyEntry {
    pub fn parse(file_content: String) -> BabyEntry {
        let path = file_content
            .trim()
            .split_once(" ")
            .unwrap_or_default()
            .1
            .to_string();

        BabyEntry {
            path: RepoPath::new(path),
        }
    }
    pub fn path_name(&self) -> RepoPath {
        self.path.clone()
    }
    pub fn name(&self) -> RepoPath {
        let name = self
            .path
            .to_string()
            .splitn(3, "/")
            .nth(2)
            .unwrap_or_default()
            .to_string();

        RepoPath::new(name)
    }
}
