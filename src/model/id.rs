use crate::error::Result;
use crate::util::Base58;
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct Id {
    bytes: Vec<u8>,
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id {{ {} }}", self.encoded())
    }
}

impl Id {
    pub fn from_string(id: &str) -> Result<Id> {
        Ok(Id {
            bytes: Base58::decode(id)?,
        })
    }

    pub fn from_bytes(id: &[u8]) -> Id {
        Id { bytes: id.into() }
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    pub fn encoded_with_prefix(&self) -> String {
        Base58::encode(&self.bytes, true)
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
