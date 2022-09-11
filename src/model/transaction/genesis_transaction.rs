use crate::error::{Error, Result};
use crate::model::Address;
use crate::util::JsonDeserializer;
use crate::waves_proto::GenesisTransactionData;
use serde_json::Value;

const TYPE: u8 = 1;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct GenesisTransactionInfo {
    recipient: Address,
    amount: u64,
}

impl GenesisTransactionInfo {
    pub fn new(recipient: Address, amount: u64) -> Self {
        Self { recipient, amount }
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct GenesisTransaction {
    recipient: Address,
    amount: u64,
}

impl GenesisTransaction {
    pub fn new(recipient: Address, amount: u64) -> Self {
        Self { recipient, amount }
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&GenesisTransaction> for GenesisTransactionData {
    type Error = Error;

    fn try_from(value: &GenesisTransaction) -> Result<Self> {
        Ok(GenesisTransactionData {
            recipient_address: value.recipient().bytes(),
            amount: value.amount() as i64,
        })
    }
}

impl TryFrom<&Value> for GenesisTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_string_from_field(value, "amount")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        Ok(GenesisTransactionInfo {
            recipient: Address::from_string(&recipient)?,
            amount: amount.parse().expect("failed to parse amount from string"),
        })
    }
}

impl TryFrom<&Value> for GenesisTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_string_from_field(value, "amount")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        Ok(GenesisTransaction {
            recipient: Address::from_string(&recipient)?,
            amount: amount.parse().expect("failed to parse amount from string"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{ChainId, GenesisTransactionInfo};
    use crate::util::JsonDeserializer;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_genesis_transaction() {
        let data =
            fs::read_to_string("./tests/resources/genesis_rs.json").expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let genesis_tx = JsonDeserializer::deserialize_signed_tx(&json, ChainId::TESTNET.byte())
            .expect("failed to deserialize");

        let genesis_tx_info = JsonDeserializer::deserialize_tx_info(&json, ChainId::TESTNET.byte())
            .expect("failed to deserialize");

        let genesis_from_json: GenesisTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(400000000000000, genesis_from_json.amount());
        assert_eq!(
            "3My3KZgFQ3CrVHgz6vGRt8687sH4oAA1qp8",
            genesis_from_json.recipient().encoded()
        );
    }
}
