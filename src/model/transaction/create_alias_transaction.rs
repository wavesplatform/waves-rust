use crate::error::{Error, Result};
use crate::util::JsonDeserializer;
use crate::waves_proto::CreateAliasTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 10;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CreateAliasTransactionInfo {
    alias: String,
}

impl CreateAliasTransactionInfo {
    pub fn new(alias: String) -> Self {
        Self { alias }
    }

    pub fn alias(&self) -> String {
        self.alias.clone()
    }
}

impl TryFrom<&Value> for CreateAliasTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let alias = JsonDeserializer::safe_to_string_from_field(value, "alias")?;

        Ok(CreateAliasTransactionInfo { alias })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CreateAliasTransaction {
    alias: String,
}

impl CreateAliasTransaction {
    pub fn new(alias: String) -> Self {
        Self { alias }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn alias(&self) -> String {
        self.alias.clone()
    }
}

impl TryFrom<&Value> for CreateAliasTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let alias = JsonDeserializer::safe_to_string_from_field(value, "alias")?;

        Ok(CreateAliasTransaction { alias })
    }
}

impl TryFrom<&CreateAliasTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &CreateAliasTransaction) -> Result<Self> {
        let mut create_alias_tx_json = Map::new();
        create_alias_tx_json.insert("alias".to_owned(), value.alias().into());
        Ok(create_alias_tx_json)
    }
}

impl TryFrom<&CreateAliasTransaction> for CreateAliasTransactionData {
    type Error = Error;

    fn try_from(value: &CreateAliasTransaction) -> Result<Self> {
        Ok(CreateAliasTransactionData {
            alias: value.alias(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::CreateAliasTransactionInfo;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_create_alias_transaction() {
        let data = fs::read_to_string("./tests/resources/create_alias_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let create_alias_tx_from_json: CreateAliasTransactionInfo =
            json.borrow().try_into().unwrap();

        assert_eq!("alias1662650000377", create_alias_tx_from_json.alias())
    }
}
