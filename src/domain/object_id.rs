use std::fmt;

use sha3::{Digest, Sha3_256};

#[derive(Debug, Clone)]
pub struct ObjectId {
    hash: String,
}
impl ObjectId {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash = Sha3_256::new();
        hash.update(bytes);
        let hash = hash.finalize();
        let hex = hex::encode(hash);
        ObjectId { hash: hex }
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}
