use crate::error::{Error, Result};
use crate::model::AssetId;
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ReissueAction {
    asset_id: AssetId,
    quantity: u64,
    reissuable: bool,
}

impl ReissueAction {
    pub fn new(asset_id: AssetId, quantity: u64, reissuable: bool) -> ReissueAction {
        ReissueAction {
            asset_id,
            quantity,
            reissuable,
        }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn is_reissuable(&self) -> bool {
        self.reissuable
    }
}

impl TryFrom<&Value> for ReissueAction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")?;
        let reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "isReissuable")?;
        Ok(ReissueAction {
            asset_id: AssetId::from_string(&asset)?,
            quantity: quantity as u64,
            reissuable,
        })
    }
}
