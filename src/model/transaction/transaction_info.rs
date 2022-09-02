use crate::error::Error::WrongTransactionType;
use crate::error::Result;
use crate::model::account::{PrivateKey, PublicKey};
use crate::model::transaction::data_transaction::DataTransaction;
use crate::model::transaction::TransactionData::Transfer;
use crate::model::transaction::TransferTransaction;
use crate::model::TransactionData::Data;
use crate::model::{AssetId, Id};
use crate::util::{sign, BinarySerializer, Hash, JsonSerializer};
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransactionInfo {
    id: String,
    signed_transaction: SignedTransaction,
    status: ApplicationStatus,
    height: u32,
}

impl TransactionInfo {
    pub fn new(
        id: String,
        signed_transaction: SignedTransaction,
        status: ApplicationStatus,
        height: u32,
    ) -> TransactionInfo {
        TransactionInfo {
            id,
            signed_transaction,
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

    pub fn signed_tx(&self) -> &SignedTransaction {
        &self.signed_transaction
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ApplicationStatus {
    Succeed,
    ScriptExecutionFailed,
    Unknown,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Transaction {
    data: TransactionData,
    fee: Amount,
    timestamp: u64,
    // todo check flatten for serde_json
    public_key: PublicKey,
    tx_type: u8,
    version: u8,
    chain_id: u8,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Amount {
    value: u64,
    asset_id: Option<AssetId>,
}

impl Amount {
    pub fn new(value: u64, asset_id: Option<AssetId>) -> Amount {
        Amount { value, asset_id }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn asset_id(&self) -> Option<AssetId> {
        self.asset_id.clone()
    }
}

impl Transaction {
    pub fn new(
        data: TransactionData,
        fee: Amount,
        timestamp: u64,
        public_key: PublicKey,
        tx_type: u8,
        version: u8,
        chain_id: u8,
    ) -> Transaction {
        Transaction {
            data,
            fee,
            timestamp,
            public_key,
            tx_type,
            version,
            chain_id,
        }
    }

    pub fn data(&self) -> &TransactionData {
        &self.data
    }

    pub fn fee(&self) -> Amount {
        self.fee.clone()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn tx_type(&self) -> u8 {
        self.tx_type
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn sign(&self, private_key: &PrivateKey) -> Result<SignedTransaction> {
        sign(self, private_key)
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        BinarySerializer::body_bytes(self)
    }

    pub fn id(&self) -> Result<Id> {
        Ok(Id::from_bytes(&Hash::blake(&self.bytes()?)?))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TransactionData {
    Transfer(TransferTransaction),
    Data(DataTransaction),
}

impl TransactionData {
    pub fn transfer_tx(&self) -> Result<&TransferTransaction> {
        match self {
            Transfer(tx) => Ok(tx),
            tx => Err(WrongTransactionType {
                expected_type: TransferTransaction::tx_type(),
                actual_type: tx.tx_type(),
            }),
        }
    }

    pub fn data_tx(&self) -> Result<&DataTransaction> {
        match self {
            Data(tx) => Ok(tx),
            tx => Err(WrongTransactionType {
                expected_type: DataTransaction::tx_type(),
                actual_type: tx.tx_type(),
            }),
        }
    }

    pub fn tx_type(&self) -> u8 {
        match self {
            Transfer(_) => TransferTransaction::tx_type(),
            Data(_) => DataTransaction::tx_type(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SignedTransaction {
    transaction: Transaction,
    proofs: Vec<Vec<u8>>,
}

impl SignedTransaction {
    pub fn new(transaction: Transaction, proofs: Vec<Vec<u8>>) -> SignedTransaction {
        SignedTransaction {
            transaction,
            proofs,
        }
    }

    pub fn tx(&self) -> &Transaction {
        &self.transaction
    }

    pub fn id(&self) -> Result<Id> {
        self.tx().id()
    }

    pub fn proofs(&self) -> Vec<Vec<u8>> {
        self.proofs.clone()
    }

    pub fn to_json(&self) -> Result<Value> {
        JsonSerializer::serialize_signed_tx(self)
    }

    //todo sign
}
