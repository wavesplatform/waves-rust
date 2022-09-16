use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId, Base58String, ByteString};
use crate::util::{Base58, JsonDeserializer};
use crate::waves_proto::{recipient, Amount as ProtoAmount, Recipient, TransferTransactionData};
use serde_json::{Map, Value};

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

impl TryFrom<&TransferTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(transfer_tx: &TransferTransaction) -> Result<Self> {
        let mut json = Map::new();
        json.insert(
            "recipient".to_owned(),
            transfer_tx.recipient().encoded().into(),
        );
        json.insert("amount".to_owned(), transfer_tx.amount().value().into());
        json.insert("assetId".to_owned(), transfer_tx.amount().asset_id().into());
        json.insert(
            "attachment".to_owned(),
            transfer_tx.attachment().encoded().into(),
        );
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{
        Address, Amount, AssetId, Base58String, ByteString, SponsorFeeTransaction,
        TransferTransaction,
    };
    use crate::waves_proto::recipient::Recipient;
    use crate::waves_proto::{SponsorFeeTransactionData, TransferTransactionData};
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_transfer_transaction() {
        let data =
            fs::read_to_string("./tests/resources/transfer_rs.json").expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let transfer_tx_from_json: TransferTransaction = json.borrow().try_into().unwrap();

        assert_eq!(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            transfer_tx_from_json.recipient().encoded()
        );
        let amount = transfer_tx_from_json.amount();
        assert_eq!(None, amount.asset_id());
        assert_eq!(200000000, amount.value());
        assert_eq!("", transfer_tx_from_json.attachment().encoded())
    }

    #[test]
    fn test_transfer_to_proto() -> Result<()> {
        let transfer_tx = &TransferTransaction::new(
            Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?,
            Amount::new(
                32,
                Some(AssetId::from_string(
                    "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                )?),
            ),
            Base58String::from_bytes(vec![1, 2, 3]),
        );
        let proto: TransferTransactionData = transfer_tx.try_into()?;

        let proto_recipient = if let Recipient::PublicKeyHash(bytes) =
            proto.clone().recipient.unwrap().recipient.unwrap()
        {
            bytes
        } else {
            panic!("expected dapp public key hash")
        };
        assert_eq!(proto_recipient, transfer_tx.recipient().public_key_hash());
        let amount = proto.amount.unwrap();
        assert_eq!(amount.amount as u64, transfer_tx.amount().value());
        assert_eq!(
            Some(amount.asset_id),
            transfer_tx.amount().asset_id().map(|it| it.bytes())
        );

        Ok(())
    }

    #[test]
    fn test_transfer_to_json() -> Result<()> {
        let transfer_tx = &TransferTransaction::new(
            Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?,
            Amount::new(
                32,
                Some(AssetId::from_string(
                    "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                )?),
            ),
            Base58String::from_bytes(vec![1, 2, 3]),
        );
        let map: Map<String, Value> = transfer_tx.try_into()?;
        let json: Value = map.into();
        let expected_json = json!({
             "recipient": "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
             "assetId": "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
             "amount": 32,
             "attachment": "Ldp",
        });
        assert_eq!(expected_json, json);
        Ok(())
    }
}
