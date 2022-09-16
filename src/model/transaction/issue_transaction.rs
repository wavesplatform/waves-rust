use crate::error::{Error, Result};
use crate::model::{AssetId, Base64String, ByteString};
use crate::util::JsonDeserializer;
use crate::waves_proto::IssueTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 3;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IssueTransactionInfo {
    asset_id: AssetId,
    name: String,
    description: String,
    quantity: u64,
    decimals: u32,
    is_reissuable: bool,
    script: Option<Base64String>,
}

impl IssueTransactionInfo {
    pub fn new(
        asset_id: AssetId,
        name: String,
        description: String,
        quantity: u64,
        decimals: u32,
        is_reissuable: bool,
        script: Option<Base64String>,
    ) -> Self {
        IssueTransactionInfo {
            asset_id,
            name,
            description,
            quantity,
            decimals,
            is_reissuable,
            script,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn decimals(&self) -> u32 {
        self.decimals
    }

    pub fn is_reissuable(&self) -> bool {
        self.is_reissuable
    }

    pub fn script(&self) -> Option<Base64String> {
        self.script.clone()
    }
}

impl TryFrom<&Value> for IssueTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let issue_transaction: IssueTransaction = value.try_into()?;
        let asset_id = AssetId::from_string(&JsonDeserializer::safe_to_string_from_field(
            value, "assetId",
        )?)?;

        Ok(IssueTransactionInfo {
            asset_id,
            name: issue_transaction.name(),
            description: issue_transaction.description(),
            quantity: issue_transaction.quantity(),
            decimals: issue_transaction.decimals(),
            is_reissuable: issue_transaction.is_reissuable(),
            script: issue_transaction.script(),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IssueTransaction {
    name: String,
    description: String,
    quantity: u64,
    decimals: u32,
    is_reissuable: bool,
    script: Option<Base64String>,
}

impl IssueTransaction {
    pub fn new(
        name: String,
        description: String,
        quantity: u64,
        decimals: u32,
        is_reissuable: bool,
        script: Option<Base64String>,
    ) -> Self {
        IssueTransaction {
            name,
            description,
            quantity,
            decimals,
            is_reissuable,
            script,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn decimals(&self) -> u32 {
        self.decimals
    }

    pub fn is_reissuable(&self) -> bool {
        self.is_reissuable
    }

    pub fn script(&self) -> Option<Base64String> {
        self.script.clone()
    }
}

impl TryFrom<&IssueTransaction> for IssueTransactionData {
    type Error = Error;

    fn try_from(issue_tx: &IssueTransaction) -> Result<Self> {
        let script = match issue_tx.script() {
            Some(script) => script.bytes(),
            None => vec![],
        };

        Ok(IssueTransactionData {
            name: issue_tx.name(),
            description: issue_tx.description(),
            amount: issue_tx.quantity() as i64,
            decimals: issue_tx.decimals() as i32,
            reissuable: issue_tx.is_reissuable(),
            script,
        })
    }
}

impl TryFrom<&Value> for IssueTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let description = JsonDeserializer::safe_to_string_from_field(value, "description")?;
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")? as u64;
        let decimals = JsonDeserializer::safe_to_int_from_field(value, "decimals")? as u32;
        let is_reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "reissuable")?;
        let script = match value["script"].as_str() {
            Some(val) => Some(Base64String::from_string(val)?),
            None => None,
        };

        Ok(IssueTransaction {
            name,
            description,
            quantity,
            decimals,
            is_reissuable,
            script,
        })
    }
}

impl TryFrom<&IssueTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(issue_tx: &IssueTransaction) -> Result<Self> {
        let mut json = Map::new();
        json.insert("name".to_string(), issue_tx.name().into());
        json.insert("description".to_string(), issue_tx.description().into());
        json.insert("quantity".to_string(), issue_tx.quantity().into());
        json.insert("decimals".to_string(), issue_tx.decimals().into());
        json.insert("reissuable".to_string(), issue_tx.is_reissuable().into());
        json.insert(
            "script".to_string(),
            issue_tx.script().map(|it| it.encoded()).into(),
        );
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{Base64String, ByteString, IssueTransaction, IssueTransactionInfo};
    use crate::waves_proto::IssueTransactionData;
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_issue_transaction() {
        let data = fs::read_to_string("./tests/resources/issue_transaction_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let issue_tx_from_json: IssueTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(
            "5HCFX88m6Xxws4SunQuW9ghvYBmk8rK8b6xVCRL8PyAw",
            issue_tx_from_json.asset_id().encoded()
        );
        assert_eq!("test asset", issue_tx_from_json.name());
        assert_eq!(32, issue_tx_from_json.quantity());
        assert_eq!(false, issue_tx_from_json.is_reissuable());
        assert_eq!(3, issue_tx_from_json.decimals());
        assert_eq!("this is test asset", issue_tx_from_json.description());

        let script = "base64:AgQAAAAHbWFzdGVyMQkBAAAAEWFkZHJlc3NGcm9tU3RyaW5nAAAAAQIAAAAQMzMzbWFzdGVyQWRkcmVzcwQAAAAHJG1hdGNoMAUAAAACdHgDCQAAAQAAAAIFAAAAByRtYXRjaDACAAAAE1RyYW5zZmVyVHJhbnNhY3Rpb24EAAAAAXQFAAAAByRtYXRjaDADCQAAAAAAAAIIBQAAAAF0AAAABnNlbmRlcgUAAAAHbWFzdGVyMQYJAAAAAAAAAggFAAAAAXQAAAAJcmVjaXBpZW50BQAAAAdtYXN0ZXIxAwkAAAEAAAACBQAAAAckbWF0Y2gwAgAAABdNYXNzVHJhbnNmZXJUcmFuc2FjdGlvbgQAAAACbXQFAAAAByRtYXRjaDAJAAAAAAAAAggFAAAAAm10AAAABnNlbmRlcgUAAAAHbWFzdGVyMQMJAAABAAAAAgUAAAAHJG1hdGNoMAIAAAATRXhjaGFuZ2VUcmFuc2FjdGlvbgcGFLbwIw==";

        assert_eq!(
            script,
            issue_tx_from_json.script().unwrap().encoded_with_prefix()
        );
    }

    #[test]
    fn test_issue_transaction_to_proto() -> Result<()> {
        let issue_tx = &IssueTransaction::new(
            "name".to_owned(),
            "descr".to_owned(),
            32,
            0,
            false,
            Some(Base64String::from_bytes(vec![1, 2, 3])),
        );
        let proto: IssueTransactionData = issue_tx.try_into()?;
        assert_eq!(proto.name, issue_tx.name());
        assert_eq!(proto.description, issue_tx.description());
        assert_eq!(proto.amount as u64, issue_tx.quantity());
        assert_eq!(proto.decimals as u32, issue_tx.decimals());
        assert_eq!(proto.reissuable, issue_tx.is_reissuable());
        assert_eq!(proto.script, issue_tx.script().unwrap().bytes());
        Ok(())
    }

    #[test]
    fn test_issue_tx_to_json() -> Result<()> {
        let issue_tx = &IssueTransaction::new(
            "test asset".to_owned(),
            "this is test asset".to_owned(),
            32,
            3,
            false,
            Some(Base64String::from_bytes(vec![1, 2, 3])),
        );

        let map: Map<String, Value> = issue_tx.try_into()?;
        let json: Value = map.into();
        let expected_json = json!({
            "name": "test asset",
            "quantity": 32,
            "reissuable": false,
            "decimals": 3,
            "description": "this is test asset",
            "script": "AQID",
        });
        assert_eq!(expected_json, json);
        Ok(())
    }
}
