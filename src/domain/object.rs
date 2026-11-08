use zstd::DEFAULT_COMPRESSION_LEVEL;

use crate::{domain::object_id::ObjectId, error::Result};

#[derive(Debug)]
pub enum ObjectType {
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
}

#[derive(Debug)]
pub struct Object {
    pub id: ObjectId,
    pub object_type: ObjectType,
    pub data: Vec<u8>,
}

impl Object {
    pub fn new(id: ObjectId, object_type: ObjectType, data: Vec<u8>) -> Result<Self> {
        let data = zstd::encode_all(&data[..], DEFAULT_COMPRESSION_LEVEL)?;
        Ok(Object {
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
}
