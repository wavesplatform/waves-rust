use crate::constants::HASH_LENGTH;
use serde_json::Value;

use crate::error::Error::{UnsupportedOperation, WrongTransactionType};
use crate::error::{Error, Result};
use crate::model::account::{PrivateKey, PublicKey};
use crate::model::transaction::data_transaction::DataTransaction;
use crate::model::transaction::TransactionData::Transfer;
use crate::model::transaction::TransferTransaction;
use crate::model::TransactionData::{
    Burn, CreateAlias, Data, Ethereum, Exchange, Genesis, InvokeScript, Issue, Lease, LeaseCancel,
    MassTransfer, Payment, Reissue, SetAssetScript, SetScript, SponsorFee, UpdateAssetInfo,
};
use crate::model::{Address, AssetId, BurnTransaction, BurnTransactionInfo, ByteString, CreateAliasTransaction, CreateAliasTransactionInfo, DataTransactionInfo, EthereumTransaction, EthereumTransactionInfo, ExchangeTransaction, ExchangeTransactionInfo, GenesisTransaction, GenesisTransactionInfo, Id, InvokeScriptTransaction, InvokeScriptTransactionInfo, IssueTransaction, IssueTransactionInfo, LeaseCancelTransaction, LeaseCancelTransactionInfo, LeaseTransaction, LeaseTransactionInfo, MassTransferTransaction, MassTransferTransactionInfo, PaymentTransaction, PaymentTransactionInfo, Proof, ReissueTransaction, ReissueTransactionInfo, SetAssetScriptTransaction, SetAssetScriptTransactionInfo, SetScriptTransaction, SetScriptTransactionInfo, SignedTransactionBuilder, SponsorFeeTransaction, SponsorFeeTransactionInfo, TransferTransactionInfo, UpdateAssetInfoTransaction, UpdateAssetInfoTransactionInfo};
use crate::util::{sign_tx, Base58, BinarySerializer, Hash, JsonDeserializer, JsonSerializer};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransactionInfoResponse {
    id: Id,
    status: ApplicationStatus,
    data: TransactionDataInfo,
    fee: Amount,
    timestamp: u64,
    public_key: PublicKey,
    tx_type: u8,
    version: u8,
    chain_id: u8,
    height: u32,
    proofs: Vec<Proof>,
}

#[allow(clippy::too_many_arguments)]
impl TransactionInfoResponse {
    pub fn new(
        id: Id,
        status: ApplicationStatus,
        data: TransactionDataInfo,
        fee: Amount,
        timestamp: u64,
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
    
    pub fn with_defaults(tx_data: TransactionData, chain_id: u8) -> SignedTransactionBuilder {
        SignedTransactionBuilder::new(tx_data, chain_id)
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
    Ethereum(EthereumTransactionInfo),
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
            TransactionDataInfo::Ethereum(_) => EthereumTransaction::tx_type(),
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
    Ethereum(EthereumTransaction),
}

impl TransactionData {
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
            Ethereum(_) => EthereumTransaction::tx_type(),
        }
    }

    pub fn get_min_supported_version(&self) -> u8 {
        match self {
            Genesis(_) => 2,
            Payment(_) => 2,
            Transfer(_) => 3,
            Issue(_) => 3,
            Reissue(_) => 3,
            Burn(_) => 3,
            Exchange(_) => 3,
            Lease(_) => 3,
            LeaseCancel(_) => 3,
            CreateAlias(_) => 3,
            MassTransfer(_) => 2,
            Data(_) => 2,
            SetScript(_) => 2,
            SponsorFee(_) => 2,
            SetAssetScript(_) => 2,
            InvokeScript(_) => 2,
            UpdateAssetInfo(_) => 1,
            Ethereum(_) => 1,
        }
    }

    pub fn get_min_fee(&self) -> Result<Amount> {
        let value = match self {
            Genesis(_) => 0,
            Payment(_) => 1,
            Transfer(_) => 100_000,
            Issue(tx) => tx.min_fee().value,
            Reissue(_) => 100_000,
            Burn(_) => 100_000,
            Exchange(_) => 300_000,
            Lease(_) => 100_000,
            LeaseCancel(_) => 100_000,
            CreateAlias(_) => 100_000,
            MassTransfer(_) => 100_000,
            Data(_) => 100_000,
            SetScript(_) => 1_000_000,
            SponsorFee(_) => 100_000,
            SetAssetScript(_) => 100_000_000,
            InvokeScript(_) => 500_000,
            UpdateAssetInfo(_) => 100_000,
            Ethereum(_) => Err(UnsupportedOperation("Min fee for Ethereum transaction is undefined".into()))?,
        };
        Ok(Amount::new(value, None))
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

    pub fn with_defaults(tx_data: TransactionData, chain_id: u8) -> SignedTransactionBuilder {
        SignedTransactionBuilder::new(tx_data, chain_id)
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

impl TryFrom<&Value> for TransactionInfoResponse {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let id = Id::from_string(&JsonDeserializer::safe_to_string_from_field(value, "id")?)?;

        let signed_tx: SignedTransaction = value.try_into()?;

        let tx_type = JsonDeserializer::safe_to_int_from_field(value, "type")? as u8;

        let application_status = if tx_type == 1 || tx_type == 2 {
            ApplicationStatus::Unknown
        } else {
            match value["applicationStatus"].as_str() {
                Some(status) => match status {
                    "succeeded" => ApplicationStatus::Succeed,
                    "script_execution_failed" => ApplicationStatus::ScriptExecutionFailed,
                    &_ => ApplicationStatus::Unknown,
                },
                None => ApplicationStatus::Unknown,
            }
        };
        let height = JsonDeserializer::safe_to_int_from_field(value, "height")? as u32;
        let transaction_data = match tx_type {
            1 => TransactionDataInfo::Genesis(value.try_into()?),
            2 => TransactionDataInfo::Payment(value.try_into()?),
            3 => TransactionDataInfo::Issue(value.try_into()?),
            4 => TransactionDataInfo::Transfer(value.try_into()?),
            5 => TransactionDataInfo::Reissue(value.try_into()?),
            6 => TransactionDataInfo::Burn(value.try_into()?),
            7 => TransactionDataInfo::Exchange(value.try_into()?),
            8 => TransactionDataInfo::Lease(value.try_into()?),
            9 => TransactionDataInfo::LeaseCancel(value.try_into()?),
            10 => TransactionDataInfo::CreateAlias(value.try_into()?),
            11 => TransactionDataInfo::MassTransfer(value.try_into()?),
            12 => TransactionDataInfo::Data(value.try_into()?),
            13 => TransactionDataInfo::SetScript(value.try_into()?),
            14 => TransactionDataInfo::SponsorFee(value.try_into()?),
            15 => TransactionDataInfo::SetAssetScript(value.try_into()?),
            16 => TransactionDataInfo::Invoke(value.try_into()?),
            17 => TransactionDataInfo::UpdateAssetInfo(value.try_into()?),
            18 => TransactionDataInfo::Ethereum(value.try_into()?),
            _ => return Err(UnsupportedOperation("unknown tx type".to_owned())),
        };
        let timestamp = JsonDeserializer::safe_to_int_from_field(value, "timestamp")? as u64;

        let tx = signed_tx.tx();
        Ok(TransactionInfoResponse::new(
            id,
            application_status,
            transaction_data,
            tx.fee(),
            timestamp,
            tx.public_key(),
            tx_type,
            tx.version(),
            tx.chain_id(),
            height,
            signed_tx.proofs(),
        ))
    }
}

impl TryFrom<&Value> for SignedTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let transaction: Transaction = value.try_into()?;

        let proofs_array = match transaction.tx_type() {
            1 => vec![Value::String(JsonDeserializer::safe_to_string_from_field(
                value,
                "signature",
            )?)],
            18 => vec![],
            _ => JsonDeserializer::safe_to_array_from_field(value, "proofs")?,
        };

        let proofs = proofs_array
            .iter()
            .map(|v| {
                Ok(Proof::new(Base58::decode(
                    &JsonDeserializer::safe_to_string(v)?,
                )?))
            })
            .collect::<Result<Vec<Proof>>>()?;
        Ok(SignedTransaction::new(transaction, proofs))
    }
}

impl TryFrom<&Value> for Transaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let tx_type = JsonDeserializer::safe_to_int_from_field(value, "type")? as u8;
        let fee = JsonDeserializer::safe_to_int_from_field(value, "fee")? as u64;
        let fee_asset_id = match value["feeAssetId"].as_str() {
            Some(val) => Some(AssetId::from_string(val)?),
            None => None,
        };
        let transaction_data = match tx_type {
            1 => Genesis(value.try_into()?),
            2 => Payment(value.try_into()?),
            3 => Issue(value.try_into()?),
            4 => Transfer(value.try_into()?),
            5 => Reissue(value.try_into()?),
            6 => Burn(value.try_into()?),
            7 => Exchange(value.try_into()?),
            8 => Lease(value.try_into()?),
            9 => LeaseCancel(value.try_into()?),
            10 => CreateAlias(value.try_into()?),
            11 => MassTransfer(value.try_into()?),
            12 => Data(value.try_into()?),
            13 => SetScript(value.try_into()?),
            14 => SponsorFee(value.try_into()?),
            15 => SetAssetScript(value.try_into()?),
            16 => InvokeScript(InvokeScriptTransaction::from_json(value)?),
            17 => UpdateAssetInfo(value.try_into()?),
            18 => Ethereum(value.try_into()?),
            _ => return Err(UnsupportedOperation("unknown transaction type".to_owned())),
        };
        let timestamp = JsonDeserializer::safe_to_int_from_field(value, "timestamp")? as u64;
        let public_key = match tx_type {
            1 => PublicKey::from_bytes(&[0; HASH_LENGTH])?,
            _ => {
                JsonDeserializer::safe_to_string_from_field(value, "senderPublicKey")?.try_into()?
            }
        };

        let chain_id = match tx_type {
            1 => Address::from_string(&JsonDeserializer::safe_to_string_from_field(
                value,
                "recipient",
            )?)?
                .chain_id(),
            _ => Address::from_string(&JsonDeserializer::safe_to_string_from_field(
                value, "sender",
            )?)?
                .chain_id(),
        };

        let version = match tx_type {
            1 | 2 => 1_u8,
            _ => JsonDeserializer::safe_to_int_from_field(value, "version")? as u8,
        };
        Ok(Transaction::new(
            transaction_data,
            Amount::new(fee, fee_asset_id),
            timestamp,
            public_key,
            version,
            chain_id,
        ))
    }
}
