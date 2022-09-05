use crate::error::{Error, Result};
use crate::model::AssetId;
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct BurnAction {
    asset_id: AssetId,
    amount: u64,
}

impl BurnAction {
    pub fn new(asset_id: AssetId, amount: u64) -> BurnAction {
        BurnAction { asset_id, amount }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}

impl TryFrom<&Value> for BurnAction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let amount = JsonDeserializer::safe_to_int_from_field(value, "quantity")?;
        Ok(BurnAction {
            asset_id: AssetId::from_string(&asset)?,
            amount: amount as u64,
        })
    }
}
