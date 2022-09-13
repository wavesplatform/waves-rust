use crate::error::{Error, Result};
use crate::model::{Amount, AssetId, ByteString};
use crate::util::JsonDeserializer;
use crate::waves_proto::{Amount as ProtoAmount, BurnTransactionData};
use serde_json::{Map, Value};

const TYPE: u8 = 6;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BurnTransactionInfo {
    amount: Amount,
}

impl BurnTransactionInfo {
    pub fn new(amount: Amount) -> Self {
        Self { amount }
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }
}

impl TryFrom<&Value> for BurnTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let asset_id = match value["assetId"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };

        Ok(BurnTransactionInfo {
            amount: Amount::new(amount as u64, asset_id),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BurnTransaction {
    amount: Amount,
}

impl BurnTransaction {
    pub fn new(amount: Amount) -> Self {
        Self { amount }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }
}

impl TryFrom<&Value> for BurnTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let asset_id = match value["assetId"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };

        Ok(BurnTransaction {
            amount: Amount::new(amount as u64, asset_id),
        })
    }
}

impl TryFrom<&BurnTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &BurnTransaction) -> Result<Self> {
        let mut issue_tx_json = Map::new();
        issue_tx_json.insert(
            "assetId".to_owned(),
            value
                .amount
                .asset_id()
                .map(|asset| asset.encoded().into())
                .unwrap_or(Value::Null),
        );
        issue_tx_json.insert("amount".to_owned(), value.amount.value().into());
        Ok(issue_tx_json)
    }
}

impl TryFrom<&BurnTransaction> for BurnTransactionData {
    type Error = Error;

    fn try_from(value: &BurnTransaction) -> Result<Self> {
        let asset_id = match value.amount.asset_id() {
            Some(asset) => asset.bytes(),
            None => vec![],
        };
        let amount = Some(ProtoAmount {
            asset_id,
            amount: value.amount.value() as i64,
        });

        Ok(BurnTransactionData {
            asset_amount: amount,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{BurnTransactionInfo, ByteString};
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_burn_transaction() {
        let data =
            fs::read_to_string("./tests/resources/burn_rs.json").expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let burn_from_json: BurnTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(
            "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            burn_from_json
                .amount()
                .asset_id()
                .expect("failed")
                .encoded()
        );
        assert_eq!(12, burn_from_json.amount().value());
    }
}
