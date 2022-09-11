use crate::error::Result;
use crate::model::data_entry::DataEntry;
use crate::model::TransactionData::{Data, Exchange, InvokeScript, Issue, Transfer};
use crate::model::{ByteString, Order, Transaction, TransactionData};
use crate::util::ByteWriter;
use crate::waves_proto::data_transaction_data::data_entry::Value::{
    BinaryValue, BoolValue, IntValue, StringValue,
};
use crate::waves_proto::data_transaction_data::DataEntry as ProtoDataEntry;
use crate::waves_proto::transaction::Data as ProtoData;
use crate::waves_proto::{
    recipient, Amount as ProtoAmount, Amount, BurnTransactionData, CreateAliasTransactionData,
    LeaseCancelTransactionData, LeaseTransactionData, MassTransferTransactionData, Recipient,
    SetAssetScriptTransactionData, SetScriptTransactionData, SponsorFeeTransactionData,
    TransferTransactionData, UpdateAssetInfoTransactionData,
};
use crate::waves_proto::{DataTransactionData, InvokeScriptTransactionData, IssueTransactionData};
use crate::waves_proto::{ExchangeTransactionData, Transaction as ProtoTransaction};
use crate::waves_proto::{Order as ProtoOrder, ReissueTransactionData};
use prost::Message;

pub struct BinarySerializer;

impl BinarySerializer {
    pub fn tx_body_bytes(transaction: &Transaction) -> Result<Vec<u8>> {
        let proto_data = match transaction.data() {
            TransactionData::Genesis(_) => todo!(),
            Transfer(_) => transfer_transaction_to_proto(transaction)?,
            Data(_) => data_transaction_to_proto(transaction)?,
            Issue(_) => issue_transaction_from_proto(transaction)?,
            InvokeScript(_) => invoke_script_from_proto(transaction)?,
            Exchange(exchange_tx) => {
                let proto_exchange_tx: ExchangeTransactionData = exchange_tx.try_into()?;
                ProtoData::Exchange(proto_exchange_tx)
            }
            TransactionData::Reissue(reissue_tx) => {
                let proto_reissue_tx: ReissueTransactionData = reissue_tx.try_into()?;
                ProtoData::Reissue(proto_reissue_tx)
            }
            TransactionData::Burn(burn_tx) => {
                let proto_burn_tx: BurnTransactionData = burn_tx.try_into()?;
                ProtoData::Burn(proto_burn_tx)
            }
            TransactionData::Lease(lease_tx) => {
                let proto_lease_tx: LeaseTransactionData = lease_tx.try_into()?;
                ProtoData::Lease(proto_lease_tx)
            }
            TransactionData::LeaseCancel(lease_cancel_tx) => {
                let proto_lease_cancel_tx: LeaseCancelTransactionData =
                    lease_cancel_tx.try_into()?;
                ProtoData::LeaseCancel(proto_lease_cancel_tx)
            }
            TransactionData::CreateAlias(create_alias_tx) => {
                let proto_create_alias_tx: CreateAliasTransactionData =
                    create_alias_tx.try_into()?;
                ProtoData::CreateAlias(proto_create_alias_tx)
            }
            TransactionData::MassTransfer(mass_transfer_tx) => {
                let proto_mass_transfer_tx: MassTransferTransactionData =
                    mass_transfer_tx.try_into()?;
                ProtoData::MassTransfer(proto_mass_transfer_tx)
            }
            TransactionData::SetScript(set_script_tx) => {
                let proto_set_script_tx: SetScriptTransactionData = set_script_tx.try_into()?;
                ProtoData::SetScript(proto_set_script_tx)
            }
            TransactionData::SetAssetScript(set_asset_script_tx) => {
                let proto_set_asset_script_tx: SetAssetScriptTransactionData =
                    set_asset_script_tx.try_into()?;
                ProtoData::SetAssetScript(proto_set_asset_script_tx)
            }
            TransactionData::SponsorFee(sponsor_fee_tx) => {
                let proto_sponsor_fee_tx: SponsorFeeTransactionData = sponsor_fee_tx.try_into()?;
                ProtoData::SponsorFee(proto_sponsor_fee_tx)
            }
            TransactionData::UpdateAssetInfo(update_asset_info_tx) => {
                let proto_update_asset_info: UpdateAssetInfoTransactionData =
                    update_asset_info_tx.try_into()?;
                ProtoData::UpdateAssetInfo(proto_update_asset_info)
            }
        };

        let fee_asset_id = match transaction.fee().asset_id() {
            None => vec![],
            Some(asset_id) => asset_id.bytes(),
        };

        let amount = ProtoAmount {
            amount: transaction.fee().value() as i64,
            asset_id: fee_asset_id,
        };

        let proto_tx = ProtoTransaction {
            chain_id: transaction.chain_id() as i32,
            data: Some(proto_data),
            fee: Some(amount),
            sender_public_key: transaction.public_key().bytes(),
            timestamp: transaction.timestamp() as i64,
            version: transaction.version() as i32,
        };

        let mut buf = vec![];
        proto_tx.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn order_body_byte(order: &Order) -> Result<Vec<u8>> {
        let proto_order: ProtoOrder = order.try_into()?;
        let mut buf = vec![];
        proto_order.encode(&mut buf)?;
        Ok(buf)
    }
}

pub fn transfer_transaction_to_proto(transaction: &Transaction) -> Result<ProtoData> {
    let transfer_tx = transaction.data().transfer_tx()?;
    let recipient = Some(Recipient {
        recipient: Some(recipient::Recipient::PublicKeyHash(
            transfer_tx.recipient().public_key_hash(),
        )),
    });
    let asset_id = match transfer_tx.amount().asset_id() {
        Some(value) => value.bytes(),
        None => vec![],
    };
    let amount = Some(Amount {
        asset_id,
        amount: transfer_tx.amount().value() as i64,
    });
    Ok(ProtoData::Transfer(TransferTransactionData {
        recipient,
        amount,
        attachment: transfer_tx.attachment().bytes(),
    }))
}

pub fn data_transaction_to_proto(transaction: &Transaction) -> Result<ProtoData> {
    let mut proto_data_entries: Vec<ProtoDataEntry> = vec![];
    let data_entries = transaction.data().data_tx()?.data();
    for data_entry in data_entries {
        let key = data_entry.key();
        match data_entry {
            DataEntry::IntegerEntry { key: _, value } => {
                proto_data_entries.push(ProtoDataEntry {
                    key,
                    value: Some(IntValue(value)),
                });
            }
            DataEntry::BooleanEntry { key: _, value } => {
                proto_data_entries.push(ProtoDataEntry {
                    key,
                    value: Some(BoolValue(value)),
                });
            }
            DataEntry::BinaryEntry { key: _, value } => proto_data_entries.push(ProtoDataEntry {
                key,
                value: Some(BinaryValue(value)),
            }),
            DataEntry::StringEntry { key: _, value } => proto_data_entries.push(ProtoDataEntry {
                key,
                value: Some(StringValue(value)),
            }),
            DataEntry::DeleteEntry { key: _ } => {
                proto_data_entries.push(ProtoDataEntry { key, value: None });
            }
        };
        //proto_data_entries.push(ProtoDataEntry { key, value });
    }
    let data_transaction = DataTransactionData {
        data: proto_data_entries,
    };

    Ok(ProtoData::DataTransaction(data_transaction))
}

pub fn issue_transaction_from_proto(transaction: &Transaction) -> Result<ProtoData> {
    let tx = transaction.data().issue_tx()?;
    let script = match tx.script() {
        Some(script) => script.bytes(),
        None => vec![],
    };
    let issue_transaction = IssueTransactionData {
        name: tx.name(),
        description: tx.description(),
        amount: tx.quantity() as i64,
        decimals: tx.decimals() as i32,
        reissuable: tx.is_reissuable(),
        script,
    };

    Ok(ProtoData::Issue(issue_transaction))
}

fn invoke_script_from_proto(transaction: &Transaction) -> Result<ProtoData> {
    let invoke_tx = transaction.data().invoke_script_tx()?;
    let dapp = Some(Recipient {
        recipient: Some(recipient::Recipient::PublicKeyHash(
            invoke_tx.dapp().public_key_hash(),
        )),
    });
    let payments: Vec<ProtoAmount> = invoke_tx
        .payment()
        .iter()
        .map(|amount| {
            let asset_id = match amount.asset_id() {
                Some(asset) => asset.bytes(),
                None => vec![],
            };
            ProtoAmount {
                asset_id,
                amount: amount.value() as i64,
            }
        })
        .collect();
    Ok(ProtoData::InvokeScript(InvokeScriptTransactionData {
        d_app: dapp,
        function_call: ByteWriter::bytes_from_function(&invoke_tx.function()),
        payments,
    }))
}
