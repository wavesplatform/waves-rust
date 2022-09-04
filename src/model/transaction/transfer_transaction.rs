use crate::error::Result;
use crate::model::{Address, Amount, AssetId, Base58String};
use crate::util::{Base58, JsonDeserializer};
use serde_json::Value;

const TYPE: u8 = 4;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransferTransactionInfo {
    recipient: Address,
    amount: Amount,
    attachment: Base58String,
}

impl TransferTransactionInfo {
    pub fn from_json(value: &Value) -> Result<TransferTransactionInfo> {
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        let asset: Option<AssetId> = match value["assetId"].as_str() {
            Some(value) => {
                let vec = Base58::decode(value)?;
                Some(AssetId::from_bytes(vec))
            }
            None => None,
        };
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")? as u64;
        let attachment = match value["attachment"].as_str().map(|value| value.into()) {
            Some(value) => Base58String::from_string(value)?,
            None => Base58String::empty(),
        };

        Ok(TransferTransactionInfo {
            recipient: Address::from_string(&recipient)?,
            amount: Amount::new(amount, asset),
            attachment,
        })
    }

    pub fn attachment(&self) -> Base58String {
        self.attachment.clone()
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransferTransaction {
    recipient: Address,
    amount: Amount,
    attachment: Base58String,
}

impl TransferTransaction {
    pub fn new(
        recipient: Address,
        amount: Amount,
        attachment: Base58String,
    ) -> TransferTransaction {
        TransferTransaction {
            recipient,
            amount,
            attachment,
        }
    }

    pub fn from_json(value: &Value) -> Result<TransferTransaction> {
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        let asset: Option<AssetId> = match value["assetId"].as_str() {
            Some(value) => {
                let vec = Base58::decode(value)?;
                Some(AssetId::from_bytes(vec))
            }
            None => None,
        };
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")? as u64;
        let attachment = match value["attachment"].as_str().map(|value| value.into()) {
            Some(value) => Base58String::from_string(value)?,
            None => Base58String::empty(),
        };

        Ok(TransferTransaction {
            recipient: Address::from_string(&recipient)?,
            amount: Amount::new(amount, asset),
            attachment,
        })
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn attachment(&self) -> Base58String {
        self.attachment.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}
