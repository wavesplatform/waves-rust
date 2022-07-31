use sha3::{Digest, Keccak256};
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use sha2::{Sha256};

pub struct Hash;

impl Hash {
    pub fn secure_hash(source: &[u8]) -> Vec<u8> {
        Self::keccak(&Self::blake(source))
    }

    pub fn keccak(source: &Vec<u8>) -> Vec<u8> {
        Keccak256::digest(source).to_vec()
    }

    pub fn blake(source: &[u8]) -> Vec<u8> {
        let mut blake = Blake2bVar::new(32).unwrap();
        blake.update(source);
        let mut buf = [0u8; 32];
        blake.finalize_variable(&mut buf).unwrap();
        buf.to_vec()
    }

    pub fn sha256(source: &Vec<u8>) -> Vec<u8> {
        let hash_seed = Sha256::digest(source.as_slice());
        hash_seed.as_slice().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{Base58, Hash};

    #[test]
    fn test_secure_hash() {
        let source = "test".as_bytes().to_vec();
        let expected_secure = Base58::decode(
            "JDJkZrg24XwvBgBUi6PgpHzrAFgeefb7nU8LJPRR58ga"
        ).unwrap();
        assert_eq!(Hash::secure_hash(&source), expected_secure)
    }

    #[test]
    fn test_sha256() {
        let source = "test".as_bytes().to_vec();
        let expected_sha = Base58::decode(
            "Bjj4AWTNrjQVHqgWbP2XaxXz4DYH1WZMyERHxsad7b2w"
        ).unwrap();

        assert_eq!(Hash::sha256(&source), expected_sha)
    }
}