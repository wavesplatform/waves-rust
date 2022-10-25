use prost::Message;

use TransactionData::{
    Burn, CreateAlias, Ethereum, Genesis, Lease, LeaseCancel, MassTransfer, Payment, Reissue,
    SetAssetScript, SetScript, SponsorFee, UpdateAssetInfo,
};

use crate::error::Error::UnsupportedOperation;
use crate::error::Result;
use crate::model::TransactionData::{Data, Exchange, InvokeScript, Issue, Transfer};
use crate::model::{ByteString, Order, OrderType, Transaction, TransactionData};
use crate::waves_proto::transaction::Data as ProtoData;
use crate::waves_proto::{
    Amount as ProtoAmount, Order as ProtoOrder, Transaction as ProtoTransaction,
};

use super::ByteWriter;

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

        let buf = Message::encode_to_vec(&proto_tx);
        Ok(buf)
    }

    pub fn order_body_bytes(order: &Order) -> Result<Vec<u8>> {
        match order {
            Order::V3(order) => {
                let mut bw = ByteWriter::new();

                // https://docs.waves.tech/en/blockchain/binary-format/order-binary-format#version-3
                bw.push_byte(3);
                bw.push_bytes(&mut order.sender().bytes());
                bw.push_bytes(&mut order.matcher().bytes());
                match order.amount().asset_id() {
                    Some(asset_id) => {
                        bw.push_byte(1);
                        bw.push_bytes(&mut asset_id.bytes());
                    }
                    None => {
                        bw.push_byte(0);
                    }
                }
                match order.price().asset_id() {
                    Some(asset_id) => {
                        bw.push_byte(1);
                        bw.push_bytes(&mut asset_id.bytes());
                    }
                    None => {
                        bw.push_byte(0);
                    }
                }
                match order.order_type() {
                    OrderType::Buy => {
                        bw.push_byte(0);
                    }
                    OrderType::Sell => {
                        bw.push_byte(1);
                    }
                }
                bw.push_bytes(&mut order.price().value().to_be_bytes().to_vec());
                bw.push_bytes(&mut order.amount().value().to_be_bytes().to_vec());
                bw.push_bytes(&mut order.timestamp().to_be_bytes().to_vec());
                bw.push_bytes(&mut order.expiration().to_be_bytes().to_vec());
                bw.push_bytes(&mut order.fee().value().to_be_bytes().to_vec());
                match order.fee().asset_id() {
                    Some(asset_id) => {
                        bw.push_byte(1);
                        bw.push_bytes(&mut asset_id.bytes());
                    }
                    None => {
                        bw.push_byte(0);
                    }
                }

                Ok(bw.bytes())
            }
            Order::V4(_) => {
                let proto_order: ProtoOrder = order.try_into()?;
                let buf = Message::encode_to_vec(&proto_order);
                Ok(buf)
            }
        }
    }
}
