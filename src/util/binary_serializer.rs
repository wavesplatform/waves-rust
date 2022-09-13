use prost::Message;

use TransactionData::{
    Burn, CreateAlias, Ethereum, Genesis, Lease, LeaseCancel, MassTransfer, Payment, Reissue,
    SetAssetScript, SetScript, SponsorFee, UpdateAssetInfo,
};

use crate::error::Error::UnsupportedOperation;
use crate::error::Result;
use crate::model::TransactionData::{Data, Exchange, InvokeScript, Issue, Transfer};
use crate::model::{Order, Transaction, TransactionData};
use crate::waves_proto::transaction::Data as ProtoData;
use crate::waves_proto::{
    Amount as ProtoAmount, Order as ProtoOrder, Transaction as ProtoTransaction,
};

pub struct BinarySerializer;

impl BinarySerializer {
    pub fn tx_body_bytes(transaction: &Transaction) -> Result<Vec<u8>> {
        let proto_data = match transaction.data() {
            Genesis(tx) => ProtoData::Genesis(tx.try_into()?),
            Payment(tx) => ProtoData::Payment(tx.try_into()?),
            Transfer(tx) => ProtoData::Transfer(tx.try_into()?),
            Data(tx) => ProtoData::DataTransaction(tx.try_into()?),
            Issue(tx) => ProtoData::Issue(tx.try_into()?),
            InvokeScript(tx) => ProtoData::InvokeScript(tx.try_into()?),
            Exchange(tx) => ProtoData::Exchange(tx.try_into()?),
            Reissue(tx) => ProtoData::Reissue(tx.try_into()?),
            Burn(tx) => ProtoData::Burn(tx.try_into()?),
            Lease(tx) => ProtoData::Lease(tx.try_into()?),
            LeaseCancel(tx) => ProtoData::LeaseCancel(tx.try_into()?),
            CreateAlias(tx) => ProtoData::CreateAlias(tx.try_into()?),
            MassTransfer(tx) => ProtoData::MassTransfer(tx.try_into()?),
            SetScript(tx) => ProtoData::SetScript(tx.try_into()?),
            SetAssetScript(tx) => ProtoData::SetAssetScript(tx.try_into()?),
            SponsorFee(tx) => ProtoData::SponsorFee(tx.try_into()?),
            UpdateAssetInfo(tx) => ProtoData::UpdateAssetInfo(tx.try_into()?),
            Ethereum(_) => Err(UnsupportedOperation("ethereum transaction".to_owned()))?,
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

    pub fn order_body_bytes(order: &Order) -> Result<Vec<u8>> {
        let proto_order: ProtoOrder = order.try_into()?;
        let mut buf = vec![];
        proto_order.encode(&mut buf)?;
        Ok(buf)
    }
}
