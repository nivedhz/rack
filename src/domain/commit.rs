use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;

use crate::{
    domain::{index::IndexEntries, user::User},
    error::Result,
};

#[derive(Debug, Clone)]
pub struct CommitId {
    hash: String,
}
impl CommitId {
    pub fn new(hash: String) -> CommitId {
        CommitId { hash }
    }
    pub fn hash(&self) -> String {
        self.hash.clone()
    }
    pub fn random() -> CommitId {
        let mut bytes = [0u8; 32];
        rand::fill(&mut bytes);
        let hex = hex::encode(bytes);
        CommitId { hash: hex }
    }
    pub fn shard(&self) -> String {
        self.hash[..2].to_string()
    }
    pub fn rest(&self) -> String {
        self.hash[2..].to_string()
    }
}
impl std::str::FromStr for CommitId {
    type Err = ParseError;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(CommitId {
            hash: s.to_string(),
        })
    }
}
#[derive(Debug)]
pub enum CommitParent {
    Orphan,
    Parent(CommitId),
}
impl CommitParent {
    pub fn new(hash: String) -> CommitParent {
        if hash.trim().is_empty() {
            CommitParent::Orphan
        } else {
            CommitParent::Parent(CommitId::new(hash))
        }
    }
    fn get_hash(&self) -> String {
        match self {
            CommitParent::Orphan => String::new(),
            CommitParent::Parent(commit_id) => commit_id.hash.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CommitEntry {
    parent: CommitParent,
    message: String,
    author: User,
    timestamp: i64,
    entries: IndexEntries,
}

impl CommitEntry {
    pub fn new(
        parent: CommitParent,
        message: String,
        author: User,
        timestamp: i64,
        entries: IndexEntries,
    ) -> CommitEntry {
        CommitEntry {
            parent,
            message,
            author,
            timestamp,
            entries,
        }
    }
    pub fn parse(file_content: String) -> Result<CommitEntry> {
        let (header, body) = file_content.rsplit_once("\0").unwrap_or_default();
        let (parent, message, author_name, author_email, timestamp) = header
            .split("\0")
            .map(String::from)
            .collect_tuple()
            .unwrap_or_default();
        let parent = if parent.trim().is_empty() {
            CommitParent::Orphan
        } else {
            CommitParent::Parent(CommitId::from_str(parent.trim())?)
        };
        let author = User::from_str(&author_name, &author_email);
        let timestamp: i64 = timestamp.trim().parse()?;

        let entries = IndexEntries::parse(body.trim().to_string())?;
        Ok(CommitEntry {
            parent,
            message,
            author,
            timestamp,
            entries,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        // - Header: <parent hash>0x20<message>0x20<author name>0x20<author email>0x20<commit
        // timestamp>0x00\n
        // - Body: <prev commit panty-list entries> + <panty-list entries joined by \n>
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.parent.get_hash().into_bytes());
        bytes.push(0x00);
        bytes.extend_from_slice(&self.message.to_string().into_bytes());
        bytes.push(0x00);
        bytes.extend_from_slice(&self.author.name.clone().unwrap_or_default().into_bytes());
        bytes.push(0x00);
        bytes.extend_from_slice(&self.author.email.clone().unwrap_or_default().into_bytes());
        bytes.push(0x00);
        bytes.extend_from_slice(&self.timestamp.to_string().into_bytes());
        bytes.push(0x00);
        bytes.push(0x0A);
        bytes.extend_from_slice(&self.entries.serialize());
        bytes
    }
    pub fn entries(&self) -> IndexEntries {
        self.entries.clone()
    }
}
