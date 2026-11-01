use rand::RngExt;
use sha3::Digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ObjectId([u8; 32]);

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}
impl std::str::FromStr for ObjectId {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).unwrap();
        Ok(ObjectId(bytes.try_into().unwrap()))
    }
}
impl ObjectId {
    pub fn hash(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    pub fn random() -> Self {
        let random_bytes: [u8; 32] = rand::rng().random();
        Self(random_bytes)
    }
    pub fn shard(&self) -> String {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(self.0);
        hex::encode(hasher.finalize())[..2].to_string()
    }
    pub fn rest(&self) -> String {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(self.0);
        hex::encode(hasher.finalize())[2..].to_string()
    }
    pub fn short(&self) -> String {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(self.0);
        hex::encode(hasher.finalize())[..10].to_string()
    }
}
