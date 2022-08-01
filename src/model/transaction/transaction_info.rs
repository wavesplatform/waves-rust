use crate::model::transaction::TransactionData::Transfer;
use crate::model::transaction::TransferTransaction;

pub struct TransactionInfo {
    id: String,
    transaction: Transaction,
    status: ApplicationStatus,
    height: u32,
}

impl TransactionInfo {
    pub fn new(id: String,
               transaction: Transaction,
               status: ApplicationStatus,
               height: u32,
    ) -> TransactionInfo {
        TransactionInfo {
            id,
            transaction,
            status,
            height,
        }
    }

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
    pub fn new(
        data: TransactionData,
        fee: u64, timestamp: u64,
        sender_public_key: String,
        tx_type: u8,
        version: u8,
    ) -> Transaction {
        Transaction {
            data,
            fee,
            timestamp,
            sender_public_key,
            tx_type,
            version,
        }
    }

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
    Transfer(TransferTransaction),
    Issue()
}

impl TransactionData {
    pub fn transfer(&self) -> Result<&TransferTransaction, String> {
        match self {
            Transfer(tx) => Ok(tx),
            _ => Err("failed".into())
        }
    }
}