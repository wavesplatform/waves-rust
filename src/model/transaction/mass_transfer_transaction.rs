use crate::error::{Error, Result};
use crate::model::{Amount, AssetId, Base58String, ByteString, Transfer};
use crate::util::JsonDeserializer;
use crate::waves_proto::mass_transfer_transaction_data::Transfer as ProtoTransfer;
use crate::waves_proto::MassTransferTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 11;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MassTransferTransactionInfo {
    asset_id: Option<AssetId>,
    transfers: Vec<Transfer>,
    attachment: Base58String,
    transfer_count: u32,
    total_amount: u64,
}

impl MassTransferTransactionInfo {
    pub fn new(
        asset_id: Option<AssetId>,
        transfers: Vec<Transfer>,
        attachment: Base58String,
        transfer_count: u32,
        total_amount: u64,
    ) -> Self {
        Self {
            asset_id,
            transfers,
            attachment,
            transfer_count,
            total_amount,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn asset_id(&self) -> Option<AssetId> {
        self.asset_id.clone()
    }

    pub fn transfers(&self) -> Vec<Transfer> {
        self.transfers.clone()
    }

    pub fn attachment(&self) -> Base58String {
        self.attachment.clone()
    }

    pub fn transfer_count(&self) -> u32 {
        self.transfer_count
    }

    pub fn total_amount(&self) -> u64 {
        self.total_amount
    }
}

impl TryFrom<&Value> for MassTransferTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let mass_transfer_tx: MassTransferTransaction = value.try_into()?;

        let transfer_count = JsonDeserializer::safe_to_int_from_field(value, "transferCount")?;
        let total_amount = JsonDeserializer::safe_to_int_from_field(value, "totalAmount")?;

        Ok(MassTransferTransactionInfo {
            asset_id: mass_transfer_tx.asset_id,
            transfers: mass_transfer_tx.transfers,
            attachment: mass_transfer_tx.attachment,
            transfer_count: transfer_count as u32,
            total_amount: total_amount as u64,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MassTransferTransaction {
    asset_id: Option<AssetId>,
    transfers: Vec<Transfer>,
    attachment: Base58String,
}

impl MassTransferTransaction {
    pub fn new(
        asset_id: Option<AssetId>,
        transfers: Vec<Transfer>,
        attachment: Base58String,
    ) -> Self {
        Self {
            asset_id,
            transfers,
            attachment,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn asset_id(&self) -> Option<AssetId> {
        self.asset_id.clone()
    }

    pub fn transfers(&self) -> Vec<Transfer> {
        self.transfers.clone()
    }

    pub fn attachment(&self) -> Base58String {
        self.attachment.clone()
    }
}

impl TryFrom<&Value> for MassTransferTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = match value["assetId"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };

        let transfers = JsonDeserializer::safe_to_array_from_field(value, "transfers")?
            .iter()
            .map(|transfer| transfer.try_into())
            .collect::<Result<Vec<Transfer>>>()?;

        let attachment = Base58String::from_string(JsonDeserializer::safe_to_string_from_field(
            value,
            "attachment",
        )?)?;

        Ok(MassTransferTransaction {
            asset_id,
            transfers,
            attachment,
        })
    }
}

impl TryFrom<&MassTransferTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &MassTransferTransaction) -> Result<Self> {
        let mut mass_transfer_tx_json = Map::new();

        let asset = value
            .asset_id()
            .map(|asset| asset.encoded().into())
            .unwrap_or(Value::Null);
        mass_transfer_tx_json.insert("assetId".to_owned(), asset);

        mass_transfer_tx_json.insert("attachment".to_owned(), value.attachment.encoded().into());

        let transfers: Vec<Value> = value
            .transfers
            .iter()
            .map(|transfer| transfer.try_into())
            .collect::<Result<Vec<Value>>>()?;

        mass_transfer_tx_json.insert("transfers".to_owned(), Value::Array(transfers));

        Ok(mass_transfer_tx_json)
    }
}

impl TryFrom<&MassTransferTransaction> for MassTransferTransactionData {
    type Error = Error;

    fn try_from(value: &MassTransferTransaction) -> Result<Self> {
        let asset_id = match value.asset_id() {
            Some(asset) => asset.bytes(),
            None => vec![],
        };
        let transfers = value
            .transfers
            .iter()
            .map(|transfer| transfer.try_into())
            .collect::<Result<Vec<ProtoTransfer>>>()?;
        Ok(MassTransferTransactionData {
            asset_id,
            transfers,
            attachment: value.attachment.bytes(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Address, ByteString, MassTransferTransactionInfo, Transfer};
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_mass_transfer_transaction() {
        let data = fs::read_to_string("./tests/resources/mass_transfer_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let mass_transfer_from_json: MassTransferTransactionInfo =
            json.borrow().try_into().unwrap();

        assert_eq!(None, mass_transfer_from_json.asset_id());
        assert_eq!("Ldp", mass_transfer_from_json.attachment().encoded());
        assert_eq!(2, mass_transfer_from_json.transfer_count());
        assert_eq!(22, mass_transfer_from_json.total_amount());

        let transfers = vec![
            Transfer::new(
                Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK").expect("failed"),
                10,
            ),
            Transfer::new(
                Address::from_string("3MxjhrvCr1nnDxvNJiCQfSC557gd8QYEhDx").expect("failed"),
                12,
            ),
        ];
        assert_eq!(transfers, mass_transfer_from_json.transfers())
    }
}
