use crate::error::{Error, Result};
use crate::model::{OrderInfo, SignedOrder};
use crate::util::JsonDeserializer;
use crate::waves_proto::ExchangeTransactionData;
use crate::waves_proto::Order as ProtoOrder;
use serde_json::{Map, Value};
use std::borrow::Borrow;

const TYPE: u8 = 7;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ExchangeTransactionInfo {
    order1: OrderInfo,
    order2: OrderInfo,
    amount: u64,
    price: u64,
    buy_matcher_fee: u64,
    sell_matcher_fee: u64,
}

impl ExchangeTransactionInfo {
    pub fn order1(&self) -> OrderInfo {
        self.order1.clone()
    }

    pub fn order2(&self) -> OrderInfo {
        self.order2.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn price(&self) -> u64 {
        self.price
    }

    pub fn buy_matcher_fee(&self) -> u64 {
        self.buy_matcher_fee
    }

    pub fn sell_matcher_fee(&self) -> u64 {
        self.sell_matcher_fee
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for ExchangeTransactionInfo {
    type Error = Error;

    fn try_from(exchange_tx_info_json: &Value) -> Result<Self> {
        let order1: OrderInfo = exchange_tx_info_json["order1"].borrow().try_into()?;
        let order2: OrderInfo = exchange_tx_info_json["order2"].borrow().try_into()?;
        let exchange_tx: ExchangeTransaction = exchange_tx_info_json.try_into()?;
        Ok(ExchangeTransactionInfo {
            order1,
            order2,
            amount: exchange_tx.amount,
            price: exchange_tx.price,
            buy_matcher_fee: exchange_tx.buy_matcher_fee,
            sell_matcher_fee: exchange_tx.sell_matcher_fee,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ExchangeTransaction {
    order1: SignedOrder,
    order2: SignedOrder,
    amount: u64,
    price: u64,
    buy_matcher_fee: u64,
    sell_matcher_fee: u64,
}

impl ExchangeTransaction {
    pub fn new(
        order1: SignedOrder,
        order2: SignedOrder,
        amount: u64,
        price: u64,
        buy_matcher_fee: u64,
        sell_matcher_fee: u64,
    ) -> ExchangeTransaction {
        ExchangeTransaction {
            order1,
            order2,
            amount,
            price,
            buy_matcher_fee,
            sell_matcher_fee,
        }
    }

    pub fn order1(&self) -> SignedOrder {
        self.order1.clone()
    }

    pub fn order2(&self) -> SignedOrder {
        self.order2.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn price(&self) -> u64 {
        self.price
    }

    pub fn buy_matcher_fee(&self) -> u64 {
        self.buy_matcher_fee
    }

    pub fn sell_matcher_fee(&self) -> u64 {
        self.sell_matcher_fee
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for ExchangeTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let order1: SignedOrder = value["order1"].borrow().try_into()?;
        let order2: SignedOrder = value["order2"].borrow().try_into()?;
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let price = JsonDeserializer::safe_to_int_from_field(value, "price")?;
        let buy_matcher_fee = JsonDeserializer::safe_to_int_from_field(value, "buyMatcherFee")?;
        let sell_matcher_fee = JsonDeserializer::safe_to_int_from_field(value, "sellMatcherFee")?;
        Ok(ExchangeTransaction::new(
            order1,
            order2,
            amount as u64,
            price as u64,
            buy_matcher_fee as u64,
            sell_matcher_fee as u64,
        ))
    }
}

impl TryFrom<&ExchangeTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(exchange_tx: &ExchangeTransaction) -> Result<Self> {
        let mut exchange_tx_json = Map::new();

        exchange_tx_json.insert("amount".to_owned(), exchange_tx.amount().into());
        exchange_tx_json.insert("price".to_owned(), exchange_tx.price().into());
        exchange_tx_json.insert(
            "buyMatcherFee".to_owned(),
            exchange_tx.buy_matcher_fee().into(),
        );
        exchange_tx_json.insert(
            "sellMatcherFee".to_owned(),
            exchange_tx.sell_matcher_fee().into(),
        );
        exchange_tx_json.insert("order1".to_owned(), exchange_tx.order1.borrow().try_into()?);
        exchange_tx_json.insert("order2".to_owned(), exchange_tx.order2.borrow().try_into()?);

        Ok(exchange_tx_json)
    }
}

impl TryFrom<&ExchangeTransaction> for ExchangeTransactionData {
    type Error = Error;

    fn try_from(exchange_tx: &ExchangeTransaction) -> Result<Self> {
        let order1: ProtoOrder = exchange_tx.order1().borrow().try_into()?;
        let order2: ProtoOrder = exchange_tx.order2().borrow().try_into()?;
        Ok(ExchangeTransactionData {
            amount: exchange_tx.amount as i64,
            price: exchange_tx.price as i64,
            buy_matcher_fee: exchange_tx.buy_matcher_fee as i64,
            sell_matcher_fee: exchange_tx.sell_matcher_fee as i64,
            orders: vec![order1, order2],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{
        Amount, AssetId, ByteString, ExchangeTransaction, ExchangeTransactionInfo, Order,
        OrderType, Proof, PublicKey, SignedOrder,
    };

    use crate::error::Result;
    use crate::waves_proto::order::Sender;
    use crate::waves_proto::ExchangeTransactionData;
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_exchange_tx_from_json() {
        let data =
            fs::read_to_string("./tests/resources/exchange_rs.json").expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let exchange_tx_from_json: ExchangeTransactionInfo = json.borrow().try_into().unwrap();

        let order_info1 = exchange_tx_from_json.order1();
        let chain_id = order_info1.chain_id();
        assert_eq!(4, order_info1.version());
        assert_eq!(
            "BBzUjBofreRf7gPMwWKT79ayZ5zNRQqgq8R9cRKZZ8ru",
            order_info1.id().encoded()
        );
        assert_eq!(
            "3MxjhrvCr1nnDxvNJiCQfSC557gd8QYEhDx",
            order_info1
                .sender()
                .address(chain_id)
                .expect("failed to get address")
                .encoded()
        );
        assert_eq!(
            "9oRf59sSHE2inwF6wraJDPQNsx7ktMKxaKvyFFL8GDrh",
            order_info1.sender().encoded()
        );
        assert_eq!(
            "CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt",
            order_info1.matcher().encoded()
        );
        assert_eq!(
            "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            order_info1
                .amount()
                .asset_id()
                .expect("asset must be non empty")
                .encoded()
        );
        assert_eq!(100, order_info1.amount().value());
        assert_eq!(None, order_info1.price().asset_id());
        assert_eq!(1000, order_info1.price().value());
        assert_eq!(OrderType::Buy, order_info1.order_type());
        assert_eq!(1662500994929, order_info1.timestamp());
        assert_eq!(1665092994929, order_info1.expiration());
        assert_eq!(300000, order_info1.fee().value());
        assert_eq!(None, order_info1.fee().asset_id());
        assert_eq!("2YgYwW6o88K3NXYy39TaUu1bwVkzpbr9oQwSDehnkJskfshC6f9F5vYmY736kEExRGHiDmW4hbuyxuqE8cw8WeJ8", &order_info1.proofs()[0].encoded());

        let order_info2 = exchange_tx_from_json.order2();
        let chain_id = order_info2.chain_id();
        assert_eq!(4, order_info2.version());
        assert_eq!(
            "3qKPaEp8DDxRJQCV8ZKs4MZcTTZSsio57kXxoZtGLVgk",
            order_info2.id().encoded()
        );
        assert_eq!(
            "3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK",
            order_info2
                .sender()
                .address(chain_id)
                .expect("failed to get address")
                .encoded()
        );
        assert_eq!(
            "CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt",
            order_info2.sender().encoded()
        );
        assert_eq!(
            "CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt",
            order_info2.matcher().encoded()
        );
        assert_eq!(
            "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            order_info2
                .amount()
                .asset_id()
                .expect("asset must be non empty")
                .encoded()
        );
        assert_eq!(100, order_info2.amount().value());
        assert_eq!(None, order_info2.price().asset_id());
        assert_eq!(1000, order_info2.price().value());
        assert_eq!(OrderType::Sell, order_info2.order_type());
        assert_eq!(1662500994931, order_info2.timestamp());
        assert_eq!(1665092994931, order_info2.expiration());
        assert_eq!(300000, order_info2.fee().value());
        assert_eq!(None, order_info2.fee().asset_id());
        assert_eq!("5Mbvg4kz1rPLBVBWoTcY2e6Zajoqxq6g38WPfvxCMiHmjxm8TPZpLpEitf9SdfGSpBHtAxas2YRe7X4UcmBugDFL", &order_info2.proofs()[0].encoded());

        assert_eq!(100, exchange_tx_from_json.amount());
        assert_eq!(1000, exchange_tx_from_json.price());
        assert_eq!(300000, exchange_tx_from_json.buy_matcher_fee());
        assert_eq!(300000, exchange_tx_from_json.sell_matcher_fee());
    }

    #[test]
    fn test_exchange_transaction_to_json() -> Result<()> {
        let buy_order = SignedOrder::new(
            Order::new(
            84,
            4,
            1662500994929,
            PublicKey::from_string("9oRf59sSHE2inwF6wraJDPQNsx7ktMKxaKvyFFL8GDrh")?,
            Amount::new(300000, None),
            OrderType::Buy,
            Amount::new(
                100,
                Some(AssetId::from_string(
                    "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                )?),
            ),
            Amount::new(1000, None),
            PublicKey::from_string("CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt")?,
            1665092994929,
            ),
            vec![Proof::from_string("2YgYwW6o88K3NXYy39TaUu1bwVkzpbr9oQwSDehnkJskfshC6f9F5vYmY736kEExRGHiDmW4hbuyxuqE8cw8WeJ8")?]
        );

        let sell_order = SignedOrder::new(Order::new(
            84,
            4,
            1662500994931,
            PublicKey::from_string("CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt")?,
            Amount::new(300000, None),
            OrderType::Sell,
            Amount::new(
                100,
                Some(AssetId::from_string(
                    "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                )?),
            ),
            Amount::new(1000, None),
            PublicKey::from_string("CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt")?,
            1665092994931,
            ),
        vec![Proof::from_string("5Mbvg4kz1rPLBVBWoTcY2e6Zajoqxq6g38WPfvxCMiHmjxm8TPZpLpEitf9SdfGSpBHtAxas2YRe7X4UcmBugDFL")?]
        );

        let exchange_transaction =
            &ExchangeTransaction::new(buy_order, sell_order, 100, 1000, 300000, 300000);
        let map: Map<String, Value> = exchange_transaction.try_into()?;
        let json: Value = map.into();

        let expected_json = json!({
            "order1": {
                "version": 4,
                "sender": "3MxjhrvCr1nnDxvNJiCQfSC557gd8QYEhDx",
                "senderPublicKey": "9oRf59sSHE2inwF6wraJDPQNsx7ktMKxaKvyFFL8GDrh",
                "matcherPublicKey": "CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt",
                "assetPair": {
                  "amountAsset": "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                  "priceAsset": null
                },
                "orderType": "buy",
                "amount": 100,
                "price": 1000,
                "timestamp": 1662500994929_i64,
                "expiration": 1665092994929_i64,
                "matcherFee": 300000,
                "signature": "2YgYwW6o88K3NXYy39TaUu1bwVkzpbr9oQwSDehnkJskfshC6f9F5vYmY736kEExRGHiDmW4hbuyxuqE8cw8WeJ8",
                "proofs": [
                  "2YgYwW6o88K3NXYy39TaUu1bwVkzpbr9oQwSDehnkJskfshC6f9F5vYmY736kEExRGHiDmW4hbuyxuqE8cw8WeJ8"
                ],
                "matcherFeeAssetId": null,
            },
            "order2": {
              "version": 4,
              "sender": "3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK",
              "senderPublicKey": "CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt",
              "matcherPublicKey": "CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt",
              "assetPair": {
                "amountAsset": "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                "priceAsset": null
              },
              "orderType": "sell",
              "amount": 100,
              "price": 1000,
              "timestamp": 1662500994931_i64,
              "expiration": 1665092994931_i64,
              "matcherFee": 300000,
              "signature": "5Mbvg4kz1rPLBVBWoTcY2e6Zajoqxq6g38WPfvxCMiHmjxm8TPZpLpEitf9SdfGSpBHtAxas2YRe7X4UcmBugDFL",
              "proofs": [
                "5Mbvg4kz1rPLBVBWoTcY2e6Zajoqxq6g38WPfvxCMiHmjxm8TPZpLpEitf9SdfGSpBHtAxas2YRe7X4UcmBugDFL"
              ],
              "matcherFeeAssetId": null,
            },
            "amount": 100,
            "price": 1000,
            "buyMatcherFee": 300000,
            "sellMatcherFee": 300000
        });

        assert_eq!(expected_json, json);

        Ok(())
    }

    #[test]
    fn test_exchange_transaction_to_proto() -> Result<()> {
        let buy_order = SignedOrder::new(
            Order::new(
                84,
                4,
                1662500994929,
                PublicKey::from_string("9oRf59sSHE2inwF6wraJDPQNsx7ktMKxaKvyFFL8GDrh")?,
                Amount::new(300000, None),
                OrderType::Buy,
                Amount::new(
                    100,
                    Some(AssetId::from_string(
                        "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                    )?),
                ),
                Amount::new(1000, None),
                PublicKey::from_string("CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt")?,
                1665092994929,
            ),
            vec![Proof::from_string("2YgYwW6o88K3NXYy39TaUu1bwVkzpbr9oQwSDehnkJskfshC6f9F5vYmY736kEExRGHiDmW4hbuyxuqE8cw8WeJ8")?]
        );

        let sell_order = SignedOrder::new(Order::new(
            84,
            4,
            1662500994931,
            PublicKey::from_string("CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt")?,
            Amount::new(300000, None),
            OrderType::Sell,
            Amount::new(
                100,
                Some(AssetId::from_string(
                    "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
                )?),
            ),
            Amount::new(1000, None),
            PublicKey::from_string("CJJu3U5UL35Dhq5KGRZw2rdundAv2pPgB7GF21G3y4vt")?,
            1665092994931,
        ),
           vec![Proof::from_string("5Mbvg4kz1rPLBVBWoTcY2e6Zajoqxq6g38WPfvxCMiHmjxm8TPZpLpEitf9SdfGSpBHtAxas2YRe7X4UcmBugDFL")?]
        );

        let exchange_transaction =
            &ExchangeTransaction::new(buy_order, sell_order, 100, 1000, 300000, 300000);
        let proto: ExchangeTransactionData = exchange_transaction.try_into()?;

        assert_eq!(exchange_transaction.amount(), proto.amount as u64);
        assert_eq!(
            exchange_transaction.sell_matcher_fee(),
            proto.sell_matcher_fee as u64
        );
        assert_eq!(exchange_transaction.price(), proto.price as u64);
        assert_eq!(
            exchange_transaction.buy_matcher_fee(),
            proto.buy_matcher_fee as u64
        );

        let buy_order_proto = &proto.orders[0];
        let buy_order = &exchange_transaction.order1.order();
        assert_eq!(buy_order.chain_id(), buy_order_proto.chain_id as u8);
        assert_eq!(buy_order.version(), buy_order_proto.version as u8);
        assert_eq!(buy_order.timestamp(), buy_order_proto.timestamp as u64);
        let proto_sender =
            if let Sender::SenderPublicKey(bytes) = buy_order_proto.clone().sender.unwrap() {
                bytes
            } else {
                panic!("expected sender public key")
            };
        assert_eq!(buy_order.sender().bytes(), proto_sender);
        assert_eq!(buy_order.amount().value(), buy_order_proto.amount as u64);
        assert_eq!(
            buy_order.fee().value(),
            buy_order_proto.clone().matcher_fee.unwrap().amount as u64
        );
        assert_eq!(
            buy_order
                .fee()
                .asset_id()
                .map(|it| it.bytes()).unwrap_or_default(),
            buy_order_proto.clone().matcher_fee.unwrap().asset_id
        );

        assert_eq!(
            match buy_order.order_type() {
                OrderType::Buy => 0,
                OrderType::Sell => 1,
            },
            buy_order_proto.order_side
        );

        assert_eq!(buy_order.amount().value(), buy_order_proto.amount as u64);
        assert_eq!(
            buy_order
                .amount()
                .asset_id()
                .map(|it| it.bytes()).unwrap_or_default(),
            buy_order_proto.clone().asset_pair.unwrap().amount_asset_id
        );

        assert_eq!(buy_order.price().value(), buy_order_proto.price as u64);
        assert_eq!(
            buy_order
                .price()
                .asset_id()
                .map(|it| it.bytes()).unwrap_or_default(),
            buy_order_proto.clone().asset_pair.unwrap().price_asset_id
        );

        assert_eq!(
            buy_order.matcher().bytes(),
            buy_order_proto.matcher_public_key
        );
        assert_eq!(
            exchange_transaction.order1.proofs()[0].bytes(),
            buy_order_proto.proofs[0]
        );

        let sell_order_proto = &proto.orders[1];
        let sell_order = &exchange_transaction.order2().order();
        assert_eq!(sell_order.chain_id(), sell_order_proto.chain_id as u8);
        assert_eq!(sell_order.version(), sell_order_proto.version as u8);
        assert_eq!(sell_order.timestamp(), sell_order_proto.timestamp as u64);
        let proto_sender =
            if let Sender::SenderPublicKey(bytes) = sell_order_proto.clone().sender.unwrap() {
                bytes
            } else {
                panic!("expected sender public key")
            };
        assert_eq!(sell_order.sender().bytes(), proto_sender);
        assert_eq!(sell_order.amount().value(), sell_order_proto.amount as u64);
        assert_eq!(
            sell_order.fee().value(),
            sell_order_proto.clone().matcher_fee.unwrap().amount as u64
        );
        assert_eq!(
            sell_order
                .fee()
                .asset_id()
                .map(|it| it.bytes()).unwrap_or_default(),
            sell_order_proto.clone().matcher_fee.unwrap().asset_id
        );

        assert_eq!(
            match sell_order.order_type() {
                OrderType::Buy => 0,
                OrderType::Sell => 1,
            },
            sell_order_proto.order_side
        );

        assert_eq!(sell_order.amount().value(), sell_order_proto.amount as u64);
        assert_eq!(
            sell_order
                .amount()
                .asset_id()
                .map(|it| it.bytes()).unwrap_or_default(),
            sell_order_proto.clone().asset_pair.unwrap().amount_asset_id
        );

        assert_eq!(sell_order.price().value(), sell_order_proto.price as u64);
        assert_eq!(
            sell_order
                .price()
                .asset_id()
                .map(|it| it.bytes()).unwrap_or_default(),
            sell_order_proto.clone().asset_pair.unwrap().price_asset_id
        );

        assert_eq!(
            sell_order.matcher().bytes(),
            sell_order_proto.matcher_public_key
        );
        assert_eq!(
            exchange_transaction.order2.proofs()[0].bytes(),
            sell_order_proto.proofs[0]
        );

        Ok(())
    }
}
