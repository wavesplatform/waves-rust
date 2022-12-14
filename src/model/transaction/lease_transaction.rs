use crate::error::{Error, Result};
use crate::model::{Address, ByteString};
use crate::util::JsonDeserializer;
use crate::waves_proto::{recipient, LeaseTransactionData, Recipient};
use serde_json::{Map, Value};

const TYPE: u8 = 8;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LeaseTransactionInfo {
    recipient: Address,
    amount: u64,
}

impl LeaseTransactionInfo {
    pub fn new(recipient: Address, amount: u64) -> Self {
        Self { recipient, amount }
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }
}

impl TryFrom<&Value> for LeaseTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;

        Ok(LeaseTransactionInfo {
            recipient: Address::from_string(&recipient)?,
            amount: amount as u64,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LeaseTransaction {
    recipient: Address,
    amount: u64,
}

impl LeaseTransaction {
    pub fn new(recipient: Address, amount: u64) -> Self {
        Self { recipient, amount }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }
}

impl TryFrom<&Value> for LeaseTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;

        Ok(LeaseTransaction {
            recipient: Address::from_string(&recipient)?,
            amount: amount as u64,
        })
    }
}

impl TryFrom<&LeaseTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &LeaseTransaction) -> Result<Self> {
        let mut lease_tx_json = Map::new();

        lease_tx_json.insert("recipient".to_owned(), value.recipient().encoded().into());
        lease_tx_json.insert("amount".to_owned(), value.amount().into());

        Ok(lease_tx_json)
    }
}

impl TryFrom<&LeaseTransaction> for LeaseTransactionData {
    type Error = Error;

    fn try_from(value: &LeaseTransaction) -> Result<Self> {
        let recipient = Some(Recipient {
            recipient: Some(recipient::Recipient::PublicKeyHash(
                value.recipient().public_key_hash(),
            )),
        });

        Ok(LeaseTransactionData {
            recipient,
            amount: value.amount as i64,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{Address, ByteString, LeaseTransaction, LeaseTransactionInfo};
    use crate::waves_proto::recipient::Recipient;
    use crate::waves_proto::LeaseTransactionData;
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_lease_transaction() {
        let data =
            fs::read_to_string("./tests/resources/lease_rs.json").expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let lease_from_json: LeaseTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(
            "3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK",
            lease_from_json.recipient().encoded()
        );
        assert_eq!(100, lease_from_json.amount());
    }

    #[test]
    fn test_lease_transaction_to_proto() -> Result<()> {
        let lease_tx = &LeaseTransaction::new(
            Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK")?,
            32,
        );
        let proto: LeaseTransactionData = lease_tx.try_into()?;

        assert_eq!(proto.amount as u64, lease_tx.amount());

        let proto_recipient =
            if let Recipient::PublicKeyHash(bytes) = proto.recipient.unwrap().recipient.unwrap() {
                bytes
            } else {
                panic!("expected dapp public key hash")
            };
        assert_eq!(proto_recipient, lease_tx.recipient().public_key_hash());
        Ok(())
    }

    #[test]
    fn test_lease_transaction_to_json() -> Result<()> {
        let lease_tx = &LeaseTransaction::new(
            Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK")?,
            32,
        );

        let map: Map<String, Value> = lease_tx.try_into()?;
        let json: Value = map.into();
        let expected_json = json!({
            "amount": 32,
            "recipient": "3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK"
        });
        assert_eq!(expected_json, json);
        Ok(())
    }
}
