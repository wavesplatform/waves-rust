use std::ops::Deref;

use sha3::{Digest, Keccak256, Sha3_256};
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};

pub struct Hash;

impl Hash {
    pub fn secure_hash(source: Vec<u8>) -> Vec<u8> {
        Self::keccak(Self::blake(&source))
    }

    pub fn keccak(source: Vec<u8>) -> Vec<u8> {
        Keccak256::digest(source).to_vec()
    }

    pub fn blake(source: &Vec<u8>) -> Vec<u8> {
        let mut blake = Blake2bVar::new(32).unwrap();
        blake.update(source);
        let mut buf = [0u8; 32];
        blake.finalize_variable(&mut buf).unwrap();
        buf.to_vec()
    }

    pub fn sha256(source: &Vec<u8>) -> Vec<u8> {
        // todo maybe not this type
        Sha3_256::digest(source).to_vec()
    }
}


// public static byte[] keccak(byte[] source) {
// return hash(source, 0, source.length, KECCAK256);
// }

//private static byte[] hash(byte[] message, int ofs, int len, ThreadLocal<Digest> alg)