use crate::error::{Error, Result};
use crate::util::Base58;
use serde_json::Value;
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct AssetId {
    bytes: Vec<u8>,
}

impl fmt::Debug for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AssetId {{ {} }}", self.encoded())
    }
}

impl AssetId {
    pub fn from_string(asset_id: &str) -> Result<AssetId> {
        Ok(Self::from_bytes(Base58::decode(asset_id)?))
    }

    pub fn from_bytes(bytes: Vec<u8>) -> AssetId {
        AssetId { bytes }
    }

    //todo fix
    pub fn is_waves(&self) -> bool {
        false
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }
}

impl TryFrom<&str> for AssetId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        AssetId::from_string(value)
    }
}

impl From<AssetId> for Value {
    fn from(value: AssetId) -> Self {
        Value::String(value.encoded())
    }
}
