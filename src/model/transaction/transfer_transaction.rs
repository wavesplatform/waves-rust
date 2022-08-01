use serde_json::Value;

pub struct TransferTransaction {
    pub recipient: String,
    pub asset: Option<String>,
    pub amount: u64,
    pub fee_asset: Option<String>,
    pub attachment: Option<String>,
}

impl TransferTransaction {
    // todo return Result<TransferTransaction, Error>
    pub fn from_json(value: Value) -> TransferTransaction {
        let recipient = value["recipient"].as_str().unwrap().into();
        let asset: Option<String> = value["assetId"].as_str().map(|value| value.into());
        let amount = value["amount"].as_i64().unwrap() as u64;
        let fee_asset = value["feeAssetId"].as_str().map(|value| value.into());
        let attachment = value["attachment"].as_str().map(|value| value.into());

        TransferTransaction {
            recipient,
            asset,
            amount,
            fee_asset,
            attachment,
        }
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
}