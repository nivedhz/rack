use sha3::{Digest, Sha3_256};
use std::fmt;
use zstd::DEFAULT_COMPRESSION_LEVEL;

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectId {
    hash: String,
}
impl ObjectId {
    pub fn new(hash: String) -> Result<Self> {
        if hash.len() != 64 {
            return Err(Error::InvalidObject {
                message: format!("'{hash}' invalid object id"),
            });
        }
        Ok(ObjectId { hash })
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash = Sha3_256::new();
        hash.update(bytes);
        let hash = hash.finalize();
        let hex = hex::encode(hash);
        ObjectId { hash: hex }
    }
    pub fn hash(&self) -> String {
        self.hash.clone()
    }
    pub fn shard(&self) -> String {
        self.hash[..2].to_string()
    }
    pub fn rest(&self) -> String {
        self.hash[2..].to_string()
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

#[derive(Debug, Default)]
pub enum ObjectType {
    #[default]
    Blob,
}
impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
        }
    }
}
impl ObjectType {
    fn as_bytes(&self) -> &[u8] {
        match self {
            ObjectType::Blob => b"blob",
        }
    }
    fn from_bytes(s: &[u8]) -> Option<ObjectType> {
        match s {
            b"blob" => Some(ObjectType::Blob),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ObjectEntry {
    pub id: ObjectId,
    pub object_type: ObjectType,
    pub data: Vec<u8>,
}

impl ObjectEntry {
    pub fn new(id: ObjectId, object_type: ObjectType, data: Vec<u8>) -> Result<Self> {
        let data = zstd::encode_all(&data[..], DEFAULT_COMPRESSION_LEVEL)?;
        Ok(ObjectEntry {
            id,
            object_type,
            data,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        // <object_type>\0<compressed file content>
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(self.object_type.as_bytes());
        bytes.push(0x00);
        bytes.extend_from_slice(&self.data);
        bytes
    }
    pub fn from_bytes(bytes: Vec<u8>, obj_id: ObjectId) -> Result<ObjectEntry> {
        let idx = bytes.iter().position(|&b| b == 0x00).unwrap_or_default();
        let (obj_type, obj_data) = (&bytes[..idx], &bytes[idx + 1..]);

        let obj_type = ObjectType::from_bytes(obj_type).unwrap_or_default();
        let obj_data = zstd::decode_all(obj_data)?;

        Ok(ObjectEntry {
            id: obj_id,
            object_type: obj_type,
            data: obj_data,
        })
    }
}
