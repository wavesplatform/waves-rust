use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId, Base58String, ByteString};
use crate::util::{Base58, JsonDeserializer};
use crate::waves_proto::{recipient, Amount as ProtoAmount, Recipient, TransferTransactionData};
use serde_json::Value;

const TYPE: u8 = 4;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransferTransactionInfo {
    recipient: Address,
    amount: Amount,
    attachment: Base58String,
}

impl TransferTransactionInfo {
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

impl TryFrom<&Value> for TransferTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let transfer_transaction: TransferTransaction = value.try_into()?;
        Ok(TransferTransactionInfo {
            recipient: transfer_transaction.recipient(),
            amount: transfer_transaction.amount(),
            attachment: transfer_transaction.attachment(),
        })
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

impl TryFrom<&TransferTransaction> for TransferTransactionData {
    type Error = Error;

    fn try_from(transfer_tx: &TransferTransaction) -> Result<Self> {
        let recipient = Some(Recipient {
            recipient: Some(recipient::Recipient::PublicKeyHash(
                transfer_tx.recipient().public_key_hash(),
            )),
        });
        let asset_id = match transfer_tx.amount().asset_id() {
            Some(value) => value.bytes(),
            None => vec![],
        };
        let amount = Some(ProtoAmount {
            asset_id,
            amount: transfer_tx.amount().value() as i64,
        });
        Ok(TransferTransactionData {
            recipient,
            amount,
            attachment: transfer_tx.attachment().bytes(),
        })
    }
}

impl TryFrom<&Value> for TransferTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
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
}
