use sha3::{Digest, Sha3_256};

pub fn get_hash(file_content: &Vec<u8>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(file_content);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_contents_produce_identical_hashes() {
        let first_hash = get_hash(&b"hello world".to_vec());
        let second_hash = get_hash(&b"hello world".to_vec());
        assert_eq!(first_hash, second_hash);
    }
    #[test]
    fn unidentical_contents_produce_different_hashes() {
        let first_hash = get_hash(&b"hello world".to_vec());
        let second_hash = get_hash(&b"hello world2".to_vec());
        assert_ne!(first_hash, second_hash);
    }
}
