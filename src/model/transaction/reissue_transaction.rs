use crate::error::{Error, Result};
use crate::model::{Amount, AssetId, ByteString};
use crate::util::JsonDeserializer;
use crate::waves_proto::{Amount as ProtoAmount, ReissueTransactionData};
use serde_json::{Map, Value};

const TYPE: u8 = 5;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ReissueTransactionInfo {
    amount: Amount,
    reissuable: bool,
}

impl ReissueTransactionInfo {
    pub fn new(amount: Amount, reissuable: bool) -> Self {
        Self { amount, reissuable }
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn is_reissuable(&self) -> bool {
        self.reissuable
    }
}

impl TryFrom<&Value> for ReissueTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")?;
        let asset_id = match value["assetId"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };
        let reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "reissuable")?;

        Ok(ReissueTransactionInfo {
            amount: Amount::new(quantity as u64, asset_id),
            reissuable,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ReissueTransaction {
    amount: Amount,
    reissuable: bool,
}

impl ReissueTransaction {
    pub fn new(amount: Amount, reissuable: bool) -> Self {
        Self { amount, reissuable }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn is_reissuable(&self) -> bool {
        self.reissuable
    }
}

impl TryFrom<&Value> for ReissueTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")?;
        let asset_id = match value["assetId"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };
        let reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "reissuable")?;

        Ok(ReissueTransaction {
            amount: Amount::new(quantity as u64, asset_id),
            reissuable,
        })
    }
}

impl TryFrom<&ReissueTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &ReissueTransaction) -> Result<Self> {
        let mut issue_tx_json = Map::new();
        issue_tx_json.insert(
            "assetId".to_owned(),
            value
                .amount
                .asset_id()
                .map(|asset| asset.encoded().into())
                .unwrap_or(Value::Null),
        );
        issue_tx_json.insert("quantity".to_owned(), value.amount.value().into());
        issue_tx_json.insert("reissuable".to_owned(), value.reissuable.into());
        Ok(issue_tx_json)
    }
}

impl TryFrom<&ReissueTransaction> for ReissueTransactionData {
    type Error = Error;

    fn try_from(value: &ReissueTransaction) -> Result<Self> {
        let asset_id = match value.amount.asset_id() {
            Some(asset) => asset.bytes(),
            None => vec![],
        };
        let amount = Some(ProtoAmount {
            asset_id,
            amount: value.amount.value() as i64,
        });

        Ok(ReissueTransactionData {
            asset_amount: amount,
            reissuable: value.reissuable,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{Amount, AssetId, ByteString, ReissueTransaction, ReissueTransactionInfo};
    use crate::waves_proto::ReissueTransactionData;
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_reissue_transaction() {
        let data =
            fs::read_to_string("./tests/resources/reissue_rs.json").expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let reissue_from_json: ReissueTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(
            "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            reissue_from_json
                .amount()
                .asset_id()
                .expect("failed")
                .encoded()
        );
        assert_eq!(12, reissue_from_json.amount().value());
        assert_eq!(true, reissue_from_json.is_reissuable());
    }

    #[test]
    fn test_reissue_to_proto() -> Result<()> {
        let reissue_tx = &ReissueTransaction::new(
            Amount::new(
                32,
                Some(AssetId::from_string(
                    "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                )?),
            ),
            true,
        );
        let proto: ReissueTransactionData = reissue_tx.try_into()?;

        assert_eq!(proto.reissuable, reissue_tx.is_reissuable());
        let amount = proto.asset_amount.unwrap();
        assert_eq!(amount.amount as u64, reissue_tx.amount().value());
        assert_eq!(
            amount.asset_id,
            reissue_tx.amount().asset_id().unwrap().bytes()
        );
        Ok(())
    }

    #[test]
    fn test_reissue_to_json() -> Result<()> {
        let reissue_tx = &ReissueTransaction::new(
            Amount::new(
                32,
                Some(AssetId::from_string(
                    "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                )?),
            ),
            true,
        );

        let map: Map<String, Value> = reissue_tx.try_into()?;
        let json: Value = map.into();
        let expected_json = json!({
            "assetId": "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            "quantity": 32,
            "reissuable": true
        });
        assert_eq!(expected_json, json);
        Ok(())
    }
}
