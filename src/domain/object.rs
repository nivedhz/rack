use zstd::DEFAULT_COMPRESSION_LEVEL;

use crate::{domain::object_id::ObjectId, error::Result};

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
    fn from_str(s: &str) -> Option<ObjectType> {
        match s {
            "blob" => Some(ObjectType::Blob),
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

        let obj_type = String::from_utf8(obj_type.to_owned()).unwrap_or_default();
        let obj_type = ObjectType::from_str(&obj_type).unwrap_or_default();

        let obj_data = zstd::decode_all(obj_data)?;

        Ok(ObjectEntry {
            id: obj_id,
            object_type: obj_type,
            data: obj_data,
        })
    }
}
