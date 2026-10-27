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
    #[test]
    fn random_fn_returns_64_bits() {
        let hash = get_random_hash();
        assert_eq!(hash.len(), 64);
    }
    #[test]
    // There can be collisions, but it is unlikely
    fn random_fn_returns_random_results() {
        let first_hash = get_random_hash();
        let second_hash = get_random_hash();
        assert_ne!(first_hash, second_hash);
    }
}
