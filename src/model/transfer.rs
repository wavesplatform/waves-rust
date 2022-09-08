use crate::error::{Error, Result};
use crate::model::{Address, AssetId};
use crate::util::JsonDeserializer;
use crate::waves_proto::mass_transfer_transaction_data::Transfer as ProtoTransfer;
use crate::waves_proto::{recipient, Recipient};
use serde_json::{Map, Value};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Transfer {
    recipient: Address,
    amount: u64,
}

impl Transfer {
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

impl TryFrom<&Value> for Transfer {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        Ok(Transfer {
            recipient: Address::from_string(&recipient)?,
            amount: amount as u64,
        })
    }
}

impl TryFrom<&Transfer> for Value {
    type Error = Error;

    fn try_from(value: &Transfer) -> Result<Self> {
        let mut transfer = Map::new();

        transfer.insert("recipient".to_owned(), value.recipient().encoded().into());
        transfer.insert("amount".to_owned(), value.amount().into());
        Ok(transfer.into())
    }
}

impl TryFrom<&Transfer> for ProtoTransfer {
    type Error = Error;

    fn try_from(value: &Transfer) -> Result<Self> {
        let recipient = Recipient {
            recipient: Some(recipient::Recipient::PublicKeyHash(
                value.recipient().public_key_hash(),
            )),
        };
        Ok(ProtoTransfer {
            recipient: Some(recipient),
            amount: value.amount() as i64,
        })
    }
}
