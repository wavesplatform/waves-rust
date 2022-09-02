use crate::error::Result;
use crate::util::Base58;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Id {
    bytes: Vec<u8>,
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
}
