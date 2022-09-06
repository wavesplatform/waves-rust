use crate::error::{Error, Result};
use crate::model::AssetId;
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SponsorFeeAction {
    asset_id: AssetId,
    min_sponsored_asset_fee: u64,
}

impl SponsorFeeAction {
    pub fn new(asset_id: AssetId, min_sponsored_asset_fee: u64) -> SponsorFeeAction {
        SponsorFeeAction {
            asset_id,
            min_sponsored_asset_fee,
        }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn min_sponsored_asset_fee(&self) -> u64 {
        self.min_sponsored_asset_fee
    }
}

impl TryFrom<&Value> for SponsorFeeAction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let fee = JsonDeserializer::safe_to_int_from_field(value, "minSponsoredAssetFee")?;
        Ok(SponsorFeeAction {
            asset_id: AssetId::from_string(&asset)?,
            min_sponsored_asset_fee: fee as u64,
        })
    }
}
