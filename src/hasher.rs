use rand::RngExt;
use sha3::{Digest, Sha3_256};

pub fn get_hash(file_content: &Vec<u8>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(file_content);
    hex::encode(hasher.finalize())
}
pub fn get_random_hash() -> String {
    let random_bytes: [u8; 32] = rand::rng().random();
    hex::encode(random_bytes)
}
