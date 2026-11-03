use std::fmt;

pub struct ObjectId {
    hash: String,
}
impl ObjectId {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let hex = hex::encode(bytes);
        ObjectId { hash: hex }
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}
