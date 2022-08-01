use serde_json::Value;
use crate::json_serializer::TransactionData::Transfer;

pub fn from_json(value: Value) -> TransactionInfo {
    let fee = value["fee"].as_i64().unwrap() as u64;
    let timestamp = value["timestamp"].as_i64().unwrap() as u64;
    let sender_public_key = value["senderPublicKey"].as_str().unwrap().into();
    let type_id = value["type"].as_i64().unwrap() as u8;
    let version = value["version"].as_i64().unwrap() as u8;
    let id = value["id"].as_str().unwrap().into();

    let application_status = match value["applicationStatus"].as_str().unwrap() {
        "succeeded" => ApplicationStatus::Succeed,
        "scriptExecutionFailed" => ApplicationStatus::ScriptExecutionFailed,
        &_ => ApplicationStatus::Unknown
    };
    let height = value["height"].as_i64().unwrap() as u32;

    TransactionInfo {
        id,
        transaction: Transaction {
            data: Transfer(TransferTransaction::from_json(value)),
            fee,
            timestamp,
            sender_public_key,
            tx_type: type_id,
            version,
        },
        status: application_status,
        height,
    }
}

pub struct TransactionInfo {
    id: String,
    transaction: Transaction,
    status: ApplicationStatus,
    height: u32,
}

impl TransactionInfo {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn status(&self) -> ApplicationStatus {
        self.status
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tx(&self) -> &Transaction {
        &self.transaction
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ApplicationStatus {
    Succeed,
    ScriptExecutionFailed,
    Unknown,
}

pub struct Transaction {
    data: TransactionData,
    fee: u64,
    timestamp: u64,
    sender_public_key: String,
    tx_type: u8,
    version: u8,
}

impl Transaction {
    pub fn data(&self) -> &TransactionData {
        &self.data
    }

    pub fn fee(&self) -> u64 {
        self.fee
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn sender_public_key(&self) -> String {
        self.sender_public_key.clone()
    }

    pub fn tx_type(&self) -> u8 {
        self.tx_type
    }

    pub fn version(&self) -> u8 {
        self.version
    }
}

pub enum TransactionData {
    // Transfer {
    //     recipient: String,
    //     asset: Option<String>,
    //     amount: u64,
    //     fee_asset: Option<String>,
    //     attachment: Option<String>,
    // },
    Transfer(TransferTransaction),
    Issue(String),
}

impl TransactionData {
    pub fn transfer(&self) -> Result<&TransferTransaction, String> {
        match self {
            Transfer(tx) => Ok(tx),
            _ => Err("failed".into())
        }
    }
}

pub struct TransferTransaction {
    pub recipient: String,
    pub asset: Option<String>,
    pub amount: u64,
    pub fee_asset: Option<String>,
    pub attachment: Option<String>,
}

impl TransferTransaction {
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