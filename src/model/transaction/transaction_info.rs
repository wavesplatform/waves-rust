use serde_json::Value;

use crate::error::Error::WrongTransactionType;
use crate::error::{Error, Result};
use crate::model::account::{PrivateKey, PublicKey};
use crate::model::transaction::data_transaction::DataTransaction;
use crate::model::transaction::TransactionData::Transfer;
use crate::model::transaction::TransferTransaction;
use crate::model::TransactionData::{
    Burn, CreateAlias, Data, Exchange, Genesis, InvokeScript, Issue, Lease, LeaseCancel,
    MassTransfer, Payment, Reissue, SetAssetScript, SetScript, SponsorFee, UpdateAssetInfo,
};
use crate::model::{
    AssetId, BurnTransaction, BurnTransactionInfo, CreateAliasTransaction,
    CreateAliasTransactionInfo, DataTransactionInfo, ExchangeTransaction, ExchangeTransactionInfo,
    GenesisTransaction, GenesisTransactionInfo, Id, InvokeScriptTransaction,
    InvokeScriptTransactionInfo, IssueTransaction, IssueTransactionInfo, LeaseCancelTransaction,
    LeaseCancelTransactionInfo, LeaseTransaction, LeaseTransactionInfo, MassTransferTransaction,
    MassTransferTransactionInfo, PaymentTransaction, PaymentTransactionInfo, Proof,
    ReissueTransaction, ReissueTransactionInfo, SetAssetScriptTransaction,
    SetAssetScriptTransactionInfo, SetScriptTransaction, SetScriptTransactionInfo,
    SponsorFeeTransaction, SponsorFeeTransactionInfo, TransferTransactionInfo,
    UpdateAssetInfoTransaction, UpdateAssetInfoTransactionInfo,
};
use crate::util::{sign_tx, BinarySerializer, Hash, JsonSerializer};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransactionInfoResponse {
    id: Id, //+
    status: ApplicationStatus,
    data: TransactionDataInfo,
    fee: Amount,    //+
    timestamp: u64, //+
    // todo check flatten for serde_json
    public_key: PublicKey,
    tx_type: u8, //+
    version: u8,
    chain_id: u8,
    height: u32,
    proofs: Vec<Proof>, //+
}

#[allow(clippy::too_many_arguments)]
impl TransactionInfoResponse {
    pub fn new(
        id: Id,
        status: ApplicationStatus,
        data: TransactionDataInfo,
        fee: Amount,
        timestamp: u64,
        // todo check flatten for serde_json
        public_key: PublicKey,
        tx_type: u8,
        version: u8,
        chain_id: u8,
        height: u32,
        proofs: Vec<Proof>,
    ) -> TransactionInfoResponse {
        TransactionInfoResponse {
            id,
            status,
            data,
            fee,
            timestamp,
            // todo check flatten for serde_json
            public_key,
            tx_type,
            version,
            chain_id,
            height,
            proofs,
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn status(&self) -> ApplicationStatus {
        self.status
    }

    pub fn data(&self) -> TransactionDataInfo {
        self.data.clone()
    }

    pub fn fee(&self) -> Amount {
        self.fee.clone()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub fn height(&self) -> u32 {
        self.height
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

    pub fn proofs(&self) -> Vec<Proof> {
        self.proofs.clone()
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
        version: u8,
        chain_id: u8,
    ) -> Transaction {
        let tx_type = data.tx_type();
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

    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
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
        sign_tx(self, private_key)
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        BinarySerializer::tx_body_bytes(self)
    }

    pub fn id(&self) -> Result<Id> {
        match self.tx_type {
            1 | 2 => Err(Error::UnsupportedOperation(
                "id calculation from unsigned payment or genesis transaction".to_owned(),
            )),
            _ => Ok(Id::from_bytes(&Hash::blake(&self.bytes()?)?)),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
//todo fix it
#[allow(clippy::large_enum_variant)]
pub enum TransactionDataInfo {
    Genesis(GenesisTransactionInfo),
    Payment(PaymentTransactionInfo),
    Transfer(TransferTransactionInfo),
    Data(DataTransactionInfo),
    Issue(IssueTransactionInfo),
    Reissue(ReissueTransactionInfo),
    Lease(LeaseTransactionInfo),
    LeaseCancel(LeaseCancelTransactionInfo),
    CreateAlias(CreateAliasTransactionInfo),
    MassTransfer(MassTransferTransactionInfo),
    SetScript(SetScriptTransactionInfo),
    SponsorFee(SponsorFeeTransactionInfo),
    SetAssetScript(SetAssetScriptTransactionInfo),
    Burn(BurnTransactionInfo),
    Exchange(ExchangeTransactionInfo),
    Invoke(InvokeScriptTransactionInfo),
    UpdateAssetInfo(UpdateAssetInfoTransactionInfo),
}

impl TransactionDataInfo {
    pub fn transfer_tx(&self) -> Result<&TransferTransactionInfo> {
        match self {
            TransactionDataInfo::Transfer(tx) => Ok(tx),
            tx => Err(WrongTransactionType {
                expected_type: TransferTransaction::tx_type(),
                actual_type: tx.tx_type(),
            }),
        }
    }

    pub fn data_tx(&self) -> Result<&DataTransactionInfo> {
        match self {
            TransactionDataInfo::Data(tx) => Ok(tx),
            tx => Err(WrongTransactionType {
                expected_type: DataTransaction::tx_type(),
                actual_type: tx.tx_type(),
            }),
        }
    }

    pub fn tx_type(&self) -> u8 {
        match self {
            TransactionDataInfo::Genesis(_) => GenesisTransaction::tx_type(),
            TransactionDataInfo::Payment(_) => PaymentTransaction::tx_type(),
            TransactionDataInfo::Transfer(_) => TransferTransaction::tx_type(),
            TransactionDataInfo::Data(_) => DataTransaction::tx_type(),
            TransactionDataInfo::Issue(_) => IssueTransaction::tx_type(),
            TransactionDataInfo::Exchange(_) => ExchangeTransaction::tx_type(),
            TransactionDataInfo::Invoke(_) => InvokeScriptTransaction::tx_type(),
            TransactionDataInfo::Reissue(_) => ReissueTransaction::tx_type(),
            TransactionDataInfo::Burn(_) => BurnTransaction::tx_type(),
            TransactionDataInfo::Lease(_) => LeaseTransaction::tx_type(),
            TransactionDataInfo::LeaseCancel(_) => LeaseCancelTransaction::tx_type(),
            TransactionDataInfo::CreateAlias(_) => CreateAliasTransaction::tx_type(),
            TransactionDataInfo::MassTransfer(_) => MassTransferTransaction::tx_type(),
            TransactionDataInfo::SetScript(_) => SetScriptTransaction::tx_type(),
            TransactionDataInfo::SetAssetScript(_) => SetAssetScriptTransaction::tx_type(),
            TransactionDataInfo::SponsorFee(_) => SponsorFeeTransaction::tx_type(),
            TransactionDataInfo::UpdateAssetInfo(_) => UpdateAssetInfoTransaction::tx_type(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
//todo fix it
#[allow(clippy::large_enum_variant)]
pub enum TransactionData {
    Genesis(GenesisTransaction),
    Payment(PaymentTransaction),
    Transfer(TransferTransaction),
    Reissue(ReissueTransaction),
    Burn(BurnTransaction),
    Lease(LeaseTransaction),
    LeaseCancel(LeaseCancelTransaction),
    CreateAlias(CreateAliasTransaction),
    MassTransfer(MassTransferTransaction),
    SetScript(SetScriptTransaction),
    SponsorFee(SponsorFeeTransaction),
    SetAssetScript(SetAssetScriptTransaction),
    Data(DataTransaction),
    Issue(IssueTransaction),
    InvokeScript(InvokeScriptTransaction),
    UpdateAssetInfo(UpdateAssetInfoTransaction),
    Exchange(ExchangeTransaction),
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

    pub fn issue_tx(&self) -> Result<&IssueTransaction> {
        match self {
            Issue(tx) => Ok(tx),
            tx => Err(WrongTransactionType {
                expected_type: IssueTransaction::tx_type(),
                actual_type: tx.tx_type(),
            }),
        }
    }

    pub fn invoke_script_tx(&self) -> Result<&InvokeScriptTransaction> {
        match self {
            InvokeScript(tx) => Ok(tx),
            tx => Err(WrongTransactionType {
                expected_type: IssueTransaction::tx_type(),
                actual_type: tx.tx_type(),
            }),
        }
    }

    pub fn tx_type(&self) -> u8 {
        match self {
            Genesis(_) => GenesisTransaction::tx_type(),
            Payment(_) => PaymentTransaction::tx_type(),
            Transfer(_) => TransferTransaction::tx_type(),
            Data(_) => DataTransaction::tx_type(),
            Issue(_) => IssueTransaction::tx_type(),
            InvokeScript(_) => InvokeScriptTransaction::tx_type(),
            Exchange(_) => ExchangeTransaction::tx_type(),
            Reissue(_) => ReissueTransaction::tx_type(),
            Burn(_) => BurnTransaction::tx_type(),
            Lease(_) => LeaseTransaction::tx_type(),
            LeaseCancel(_) => LeaseCancelTransaction::tx_type(),
            CreateAlias(_) => CreateAliasTransaction::tx_type(),
            MassTransfer(_) => MassTransferTransaction::tx_type(),
            SetScript(_) => SetScriptTransaction::tx_type(),
            SetAssetScript(_) => SetAssetScriptTransaction::tx_type(),
            SponsorFee(_) => SponsorFeeTransaction::tx_type(),
            UpdateAssetInfo(_) => UpdateAssetInfoTransaction::tx_type(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SignedTransaction {
    transaction: Transaction,
    proofs: Vec<Proof>,
}

impl SignedTransaction {
    pub fn new(transaction: Transaction, proofs: Vec<Proof>) -> SignedTransaction {
        SignedTransaction {
            transaction,
            proofs,
        }
    }

    pub fn tx(&self) -> &Transaction {
        &self.transaction
    }

    pub fn id(&self) -> Result<Id> {
        let tx = self.tx();
        match tx.tx_type {
            1 | 2 => Ok(Id::from_bytes(&self.proofs[0].bytes())),
            _ => tx.id(),
        }
    }

    pub fn proofs(&self) -> Vec<Proof> {
        self.proofs.clone()
    }

    pub fn to_json(&self) -> Result<Value> {
        JsonSerializer::serialize_signed_tx(self)
    }

    //todo sign
}
