use crate::error::Result;
use crate::model::ByteString;
use crate::util::Base58;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
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

    pub fn from_string(base58str: &str) -> Result<Self> {
        Ok(Self {
            bytes: Base58::decode(base58str)?,
        })
    }
}

impl ByteString for Proof {
    fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    fn encoded_with_prefix(&self) -> String {
        Base58::encode(&self.bytes, true)
    }
}
