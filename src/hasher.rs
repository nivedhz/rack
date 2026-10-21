use sha3::{Digest, Sha3_256};

pub fn get_hash(file_content: &Vec<u8>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(file_content);
    hex::encode(hasher.finalize())
}
