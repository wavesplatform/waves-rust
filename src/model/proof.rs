use crate::util::Base58;
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct Proof {
    bytes: Vec<u8>,
}

impl fmt::Debug for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Proof {{ {} }}", self.encoded())
    }
}

impl Proof {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }
}
