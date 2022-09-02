use crate::error::{Error, Result};
use crate::util::Base58;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AssetId {
    bytes: Vec<u8>,
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
