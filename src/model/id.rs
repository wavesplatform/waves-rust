use crate::error::Result;
use crate::model::ByteString;
use crate::util::Base58;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
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
}

impl ByteString for Id {
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
