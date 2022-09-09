use crate::error::{Error, Result};
use crate::model::AssetId;
use crate::util::JsonDeserializer;
use crate::waves_proto::Amount as ProtoAmount;
use crate::waves_proto::SponsorFeeTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 14;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SponsorFeeTransactionInfo {
    asset_id: AssetId,
    min_sponsored_asset_fee: u64,
}

impl SponsorFeeTransactionInfo {
    pub fn new(asset_id: AssetId, min_sponsored_asset_fee: u64) -> Self {
        Self {
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

impl TryFrom<&Value> for SponsorFeeTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let min_sponsored_asset_fee =
            JsonDeserializer::safe_to_int_from_field(value, "minSponsoredAssetFee")?;

        Ok(SponsorFeeTransactionInfo {
            asset_id: AssetId::from_string(&asset_id)?,
            min_sponsored_asset_fee: min_sponsored_asset_fee as u64,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SponsorFeeTransaction {
    asset_id: AssetId,
    min_sponsored_asset_fee: u64,
}

impl SponsorFeeTransaction {
    pub fn new(asset_id: AssetId, min_sponsored_asset_fee: u64) -> Self {
        Self {
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

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for SponsorFeeTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let min_sponsored_asset_fee =
            JsonDeserializer::safe_to_int_from_field(value, "minSponsoredAssetFee")?;

        Ok(SponsorFeeTransaction {
            asset_id: AssetId::from_string(&asset_id)?,
            min_sponsored_asset_fee: min_sponsored_asset_fee as u64,
        })
    }
}

impl TryFrom<&SponsorFeeTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &SponsorFeeTransaction) -> Result<Self> {
        let mut sponsor_fee_tx_json = Map::new();
        sponsor_fee_tx_json.insert("assetId".to_owned(), value.asset_id().encoded().into());
        sponsor_fee_tx_json.insert(
            "minSponsoredAssetFee".to_owned(),
            value.min_sponsored_asset_fee().into(),
        );
        Ok(sponsor_fee_tx_json)
    }
}

impl TryFrom<&SponsorFeeTransaction> for SponsorFeeTransactionData {
    type Error = Error;

    fn try_from(value: &SponsorFeeTransaction) -> Result<Self> {
        let amount = ProtoAmount {
            asset_id: value.asset_id.bytes(),
            amount: value.min_sponsored_asset_fee as i64,
        };
        Ok(SponsorFeeTransactionData {
            min_fee: Some(amount),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::SponsorFeeTransaction;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_sponsor_fee_transaction() {
        let data = fs::read_to_string("./tests/resources/sponsor_fee_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let sponsor_fee_from_json: SponsorFeeTransaction = json.borrow().try_into().unwrap();

        assert_eq!(
            "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            sponsor_fee_from_json.asset_id().encoded()
        );
        assert_eq!(10, sponsor_fee_from_json.min_sponsored_asset_fee())
    }
}
