use crate::model::data_entry::DataEntry;
use crate::model::TransactionData::{Data, Transfer};
use crate::model::{Transaction, TransactionData};
use crate::util::Base58;
use crate::waves_proto::data_transaction_data::data_entry::Value::{
    BinaryValue, BoolValue, IntValue, StringValue,
};
use crate::waves_proto::data_transaction_data::DataEntry as ProtoDataEntry;
use crate::waves_proto::transaction::Data as ProtoData;
use crate::waves_proto::Amount as ProtoAmount;
use crate::waves_proto::DataTransactionData;
use crate::waves_proto::Transaction as ProtoTransaction;
use prost::Message;

pub struct BinarySerializer;

impl BinarySerializer {
    pub fn body_bytes(transaction: &Transaction) -> Vec<u8> {
        let proto_data = match transaction.data() {
            Transfer(_) => todo!(),
            Data(_) => data_transaction_to_proto(transaction),
            TransactionData::Issue() => todo!(),
        };

        let fee_asset_id = match transaction.fee().fee_asset_id() {
            None => vec![],
            Some(asset_id) => Base58::decode(&asset_id).unwrap(),
        };

        let amount = ProtoAmount {
            amount: transaction.fee().fee() as i64,
            asset_id: fee_asset_id,
        };

        let proto_tx = ProtoTransaction {
            chain_id: transaction.chain_id() as i32,
            data: Some(proto_data),
            fee: Some(amount),
            sender_public_key: transaction.public_key().bytes().clone(),
            timestamp: transaction.timestamp() as i64,
            version: transaction.version() as i32,
        };

        let mut buf = vec![];
        proto_tx.encode(&mut buf).unwrap();
        buf
    }
}

pub fn data_transaction_to_proto(transaction: &Transaction) -> ProtoData {
    let mut proto_data_entries: Vec<ProtoDataEntry> = vec![];
    let data_entries = transaction.data().data_tx().unwrap().data();
    for data_entry in data_entries {
        let key = data_entry.key();
        let value = match data_entry {
            DataEntry::IntegerEntry { key: _, value } => Some(IntValue(value)),
            DataEntry::BooleanEntry { key: _, value } => Some(BoolValue(value)),
            DataEntry::BinaryEntry { key: _, value } => Some(BinaryValue(value)),
            DataEntry::StringEntry { key: _, value } => Some(StringValue(value)),
        };
        proto_data_entries.push(ProtoDataEntry { key, value });
    }
    let data_transaction = DataTransactionData {
        data: proto_data_entries,
    };

    ProtoData::DataTransaction(data_transaction)
}
