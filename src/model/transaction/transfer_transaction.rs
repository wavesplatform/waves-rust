use crate::error::Result;
use crate::util::JsonDeserializer;
use serde_json::Value;

const TYPE: u8 = 4;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransferTransaction {
    pub recipient: String,
    //todo change to struct see WavesJ
    pub asset: Option<String>,
    pub amount: u64,
    pub fee_asset: Option<String>,
    pub attachment: Option<String>,
}

impl TransferTransaction {
    // todo return Result<TransferTransaction, Error>
    pub fn from_json(value: &Value) -> Result<TransferTransaction> {
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        let asset: Option<String> = value["assetId"].as_str().map(|value| value.into());
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")? as u64;
        let fee_asset = value["feeAssetId"].as_str().map(|value| value.into());
        let attachment = value["attachment"].as_str().map(|value| value.into());

        Ok(TransferTransaction {
            recipient,
            asset,
            amount,
            fee_asset,
            attachment,
        })
    }

    pub fn recipient(&self) -> String {
        self.recipient.clone()
    }

    pub fn asset(&self) -> Option<String> {
        self.asset.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn fee_asset(&self) -> Option<String> {
        self.fee_asset.clone()
    }

    pub fn attachment(&self) -> Option<String> {
        self.attachment.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}
