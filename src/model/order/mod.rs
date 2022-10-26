pub mod v3;
pub mod v4;

use crate::error::Error::UnsupportedOperation;
use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId, ByteString, Id, Proof, PublicKey};
use crate::util::{Base58, BinarySerializer, Hash, JsonDeserializer, JsonSerializer};
use crate::waves_proto::order::Sender::SenderPublicKey;
use crate::waves_proto::{Amount as ProtoAmount, AssetPair, Order as ProtoOrder};
use serde_json::{Map, Value};

use self::v3::{OrderInfoV3, OrderV3};
use self::v4::{OrderInfoV4, OrderV4};

use super::PrivateKey;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum OrderInfo {
    V3(v3::OrderInfoV3),
    V4(v4::OrderInfoV4),
}

// could it be done using trait?
impl OrderInfo {
    pub fn id(&self) -> Id {
        match self {
            Self::V3(order) => order.id(),
            Self::V4(order) => order.id(),
        }
    }

    pub fn chain_id(&self) -> u8 {
        match self {
            Self::V3(order) => order.chain_id(),
            Self::V4(order) => order.chain_id(),
        }
    }

    pub fn order_type(&self) -> OrderType {
        match self {
            Self::V3(order) => order.order_type(),
            Self::V4(order) => order.order_type(),
        }
    }

    pub fn version(&self) -> u8 {
        match self {
            Self::V3(order) => order.version(),
            Self::V4(order) => order.version(),
        }
    }

    pub fn sender(&self) -> PublicKey {
        match self {
            Self::V3(order) => order.sender(),
            Self::V4(order) => order.sender(),
        }
    }

    pub fn amount(&self) -> Amount {
        match self {
            Self::V3(order) => order.amount(),
            Self::V4(order) => order.amount(),
        }
    }

    pub fn price(&self) -> Amount {
        match self {
            Self::V3(order) => order.price(),
            Self::V4(order) => order.price(),
        }
    }

    pub fn fee(&self) -> Amount {
        match self {
            Self::V3(order) => order.fee(),
            Self::V4(order) => order.fee(),
        }
    }

    pub fn matcher(&self) -> PublicKey {
        match self {
            Self::V3(order) => order.matcher(),
            Self::V4(order) => order.matcher(),
        }
    }

    pub fn timestamp(&self) -> u64 {
        match self {
            Self::V3(order) => order.timestamp(),
            Self::V4(order) => order.timestamp(),
        }
    }

    pub fn expiration(&self) -> u64 {
        match self {
            Self::V3(order) => order.expiration(),
            Self::V4(order) => order.expiration(),
        }
    }

    pub fn price_mode(&self) -> PriceMode {
        match self {
            Self::V3(_) => PriceMode::AssetDecimals,
            Self::V4(order) => order.price_mode(),
        }
    }

    pub fn proofs(&self) -> Vec<Proof> {
        match self {
            Self::V3(order) => order.proofs(),
            Self::V4(order) => order.proofs(),
        }
    }
}

impl TryFrom<&Value> for OrderInfo {
    type Error = Error;

    fn try_from(order_json: &Value) -> Result<Self> {
        let signed_order: SignedOrder = order_json.try_into()?;
        signed_order.try_into()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SignedOrder {
    order: Order,
    proofs: Vec<Proof>,
}

impl SignedOrder {
    pub fn new(order: Order, proofs: Vec<Proof>) -> SignedOrder {
        SignedOrder { order, proofs }
    }

    pub fn order(&self) -> Order {
        self.order.clone()
    }

    pub fn proofs(&self) -> Vec<Proof> {
        self.proofs.clone()
    }

    pub fn id(&self) -> Result<Id> {
        Ok(Id::from_bytes(&Hash::blake(&self.bytes()?)?))
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        BinarySerializer::order_body_bytes(&self.order())
    }

    pub fn to_json(&self) -> Result<Value> {
        JsonSerializer::serialize_signed_order(self)
    }
}

impl TryFrom<&Value> for SignedOrder {
    type Error = Error;

    fn try_from(signed_order_json: &Value) -> Result<Self> {
        let order: Order = signed_order_json.try_into()?;

        let signature = Proof::new(Base58::decode(
            &JsonDeserializer::safe_to_string_from_field(signed_order_json, "signature")?,
        )?);

        Ok(SignedOrder::new(order, vec![signature]))
    }
}

impl TryFrom<&SignedOrder> for Value {
    type Error = Error;

    fn try_from(signed_order: &SignedOrder) -> Result<Self> {
        let order = signed_order.order();
        let order_type = match order.order_type() {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
        };
        let mut order_json = Map::new();
        order_json.insert("orderType".to_owned(), order_type.into());
        order_json.insert("version".to_owned(), order.version().into());
        order_json.insert(
            "senderPublicKey".to_owned(),
            order.sender().encoded().into(),
        );
        order_json.insert(
            "sender".to_owned(),
            order.sender().address(order.chain_id())?.encoded().into(),
        );
        let mut asset_pair_json = Map::new();
        asset_pair_json.insert(
            "amountAsset".to_owned(),
            order
                .amount()
                .asset_id()
                .map(|asset| asset.encoded().into())
                .unwrap_or(Value::Null),
        );
        asset_pair_json.insert(
            "priceAsset".to_owned(),
            order
                .price()
                .asset_id()
                .map(|asset| asset.encoded().into())
                .unwrap_or(Value::Null),
        );

        order_json.insert("assetPair".to_owned(), asset_pair_json.into());
        order_json.insert("amount".to_owned(), order.amount().value().into());
        order_json.insert("price".to_owned(), order.price().value().into());
        order_json.insert("matcherFee".to_owned(), order.fee().value().into());
        order_json.insert(
            "matcherPublicKey".to_owned(),
            order.matcher().encoded().into(),
        );
        order_json.insert(
            "matcherFeeAssetId".to_owned(),
            order
                .fee()
                .asset_id()
                .map(|asset| asset.encoded().into())
                .unwrap_or(Value::Null),
        );
        order_json.insert("timestamp".to_owned(), order.timestamp().into());
        order_json.insert("expiration".to_owned(), order.expiration().into());
        let signature = signed_order.proofs[0].encoded();
        order_json.insert("signature".to_owned(), signature.clone().into());
        order_json.insert("proofs".to_owned(), vec![Value::String(signature)].into());

        match order {
            Order::V3(_) => {}
            Order::V4(order_v4) => {
                let price_mode = match order_v4.price_mode() {
                    PriceMode::Default => "default",
                    PriceMode::FixedDecimals => "fixedDecimals",
                    PriceMode::AssetDecimals => "assetDecimals",
                };
                order_json.insert("priceMode".to_owned(), price_mode.into());
            }
        }

        Ok(order_json.into())
    }
}

impl TryFrom<&SignedOrder> for ProtoOrder {
    type Error = Error;

    fn try_from(signed_order: &SignedOrder) -> Result<Self> {
        let order = signed_order.order();

        match order {
            Order::V3(_) => Err(UnsupportedOperation(
                "Order version 3 can't be transformed into protobuf message".to_owned(),
            )),
            Order::V4(order) => Ok(ProtoOrder {
                chain_id: order.chain_id() as i32,
                matcher_public_key: order.matcher().bytes(),
                asset_pair: map_asset_pair(&order),
                order_side: map_order_side(&order),
                amount: order.amount().value() as i64,
                price: order.price().value() as i64,
                timestamp: order.timestamp() as i64,
                expiration: order.expiration() as i64,
                matcher_fee: map_matcher_fee(&order),
                version: order.version() as i32,
                proofs: signed_order
                    .proofs()
                    .iter()
                    .map(|proof| proof.bytes())
                    .collect(),
                price_mode: map_price_mode(&order),
                sender: Some(SenderPublicKey(order.sender().bytes())),
            }),
        }
    }
}

impl TryFrom<SignedOrder> for OrderInfo {
    type Error = Error;

    fn try_from(signed_order: SignedOrder) -> Result<Self> {
        match signed_order.order() {
            Order::V3(order) => signed_order.id().map(|id| {
                OrderInfo::V3(OrderInfoV3::new(
                    id,
                    order.chain_id(),
                    order.timestamp(),
                    order.sender(),
                    order.fee(),
                    order.order_type(),
                    order.amount(),
                    order.price(),
                    order.matcher(),
                    order.expiration(),
                    signed_order.proofs(),
                ))
            }),
            Order::V4(order) => signed_order.id().map(|id| {
                OrderInfo::V4(OrderInfoV4::new(
                    id,
                    order.chain_id(),
                    order.timestamp(),
                    order.sender(),
                    order.fee(),
                    order.order_type(),
                    order.amount(),
                    order.price(),
                    order.matcher(),
                    order.expiration(),
                    signed_order.proofs(),
                    order.price_mode(),
                ))
            }),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Order {
    V3(v3::OrderV3),
    V4(v4::OrderV4),
}

// could it be done using trait?
impl Order {
    #[allow(clippy::too_many_arguments)]
    pub fn v3(
        chain_id: u8,
        timestamp: u64,
        sender: PublicKey,
        fee: Amount,
        order_type: OrderType,
        amount: Amount,
        price: Amount,
        matcher: PublicKey,
        expiration: u64,
    ) -> Self {
        Self::V3(OrderV3::new(
            chain_id, timestamp, sender, fee, order_type, amount, price, matcher, expiration,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn v4(
        chain_id: u8,
        timestamp: u64,
        sender: PublicKey,
        fee: Amount,
        order_type: OrderType,
        amount: Amount,
        price: Amount,
        matcher: PublicKey,
        expiration: u64,
        price_mode: PriceMode,
    ) -> Self {
        Self::V4(OrderV4::new(
            chain_id, timestamp, sender, fee, order_type, amount, price, matcher, expiration,
            price_mode,
        ))
    }

    pub fn id(&self) -> Result<Id> {
        match self {
            Self::V3(order) => order.id(),
            Self::V4(order) => order.id(),
        }
    }

    pub fn chain_id(&self) -> u8 {
        match self {
            Self::V3(order) => order.chain_id(),
            Self::V4(order) => order.chain_id(),
        }
    }

    pub fn order_type(&self) -> OrderType {
        match self {
            Self::V3(order) => order.order_type(),
            Self::V4(order) => order.order_type(),
        }
    }

    pub fn version(&self) -> u8 {
        match self {
            Self::V3(order) => order.version(),
            Self::V4(order) => order.version(),
        }
    }

    pub fn sender(&self) -> PublicKey {
        match self {
            Self::V3(order) => order.sender(),
            Self::V4(order) => order.sender(),
        }
    }

    pub fn amount(&self) -> Amount {
        match self {
            Self::V3(order) => order.amount(),
            Self::V4(order) => order.amount(),
        }
    }

    pub fn price(&self) -> Amount {
        match self {
            Self::V3(order) => order.price(),
            Self::V4(order) => order.price(),
        }
    }

    pub fn fee(&self) -> Amount {
        match self {
            Self::V3(order) => order.fee(),
            Self::V4(order) => order.fee(),
        }
    }

    pub fn matcher(&self) -> PublicKey {
        match self {
            Self::V3(order) => order.matcher(),
            Self::V4(order) => order.matcher(),
        }
    }

    pub fn timestamp(&self) -> u64 {
        match self {
            Self::V3(order) => order.timestamp(),
            Self::V4(order) => order.timestamp(),
        }
    }

    pub fn expiration(&self) -> u64 {
        match self {
            Self::V3(order) => order.expiration(),
            Self::V4(order) => order.expiration(),
        }
    }

    pub fn sign(&self, private_key: &PrivateKey) -> Result<SignedOrder> {
        match self {
            Self::V3(order) => order.sign(private_key),
            Self::V4(order) => order.sign(private_key),
        }
    }

    pub fn default_expiration(current_time: u64) -> u64 {
        current_time + (30 * 24 * 60 * 60 * 1000)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PriceMode {
    Default,
    FixedDecimals,
    AssetDecimals,
}

impl TryFrom<&Order> for ProtoOrder {
    type Error = Error;

    fn try_from(order: &Order) -> Result<Self> {
        match order {
            Order::V3(_) => Err(UnsupportedOperation(
                "Order version 3 can't be transformed into protobuf message".to_owned(),
            )),
            Order::V4(order) => Ok(ProtoOrder {
                chain_id: order.chain_id() as i32,
                matcher_public_key: order.matcher().bytes(),
                asset_pair: map_asset_pair(order),
                order_side: map_order_side(order),
                amount: order.amount().value() as i64,
                price: order.price().value() as i64,
                timestamp: order.timestamp() as i64,
                expiration: order.expiration() as i64,
                matcher_fee: map_matcher_fee(order),
                version: order.version() as i32,
                proofs: vec![],
                price_mode: map_price_mode(order),
                sender: Some(SenderPublicKey(order.sender().bytes())),
            }),
        }
    }
}

impl TryFrom<&Value> for Order {
    type Error = Error;

    fn try_from(order_json: &Value) -> Result<Self> {
        let order_type =
            match JsonDeserializer::safe_to_string_from_field(order_json, "orderType")?.as_str() {
                "buy" => OrderType::Buy,
                "sell" => OrderType::Sell,
                _ => return Err(UnsupportedOperation("unknown order type".to_owned())),
            };
        let version = JsonDeserializer::safe_to_int_from_field(order_json, "version")? as u8;
        let timestamp = JsonDeserializer::safe_to_int_from_field(order_json, "timestamp")?;
        let sender_public_key = PublicKey::from_string(
            &JsonDeserializer::safe_to_string_from_field(order_json, "senderPublicKey")?,
        )?;

        let sender = Address::from_string(&JsonDeserializer::safe_to_string_from_field(
            order_json, "sender",
        )?)?;

        let matcher_fee = JsonDeserializer::safe_to_int_from_field(order_json, "matcherFee")?;
        let matcher_fee_asset_id = match order_json["matcherFeeAssetId"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };

        let amount_asset = match order_json["assetPair"]["amountAsset"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };
        let amount = JsonDeserializer::safe_to_int_from_field(order_json, "amount")?;

        let price_asset = match order_json["assetPair"]["priceAsset"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };

        let price = JsonDeserializer::safe_to_int_from_field(order_json, "price")?;

        let matcher_public_key = PublicKey::from_string(
            &JsonDeserializer::safe_to_string_from_field(order_json, "matcherPublicKey")?,
        )?;

        let expiration = JsonDeserializer::safe_to_int_from_field(order_json, "expiration")?;

        let price_mode = match order_json["priceMode"].as_str() {
            Some(price_mode) => match price_mode {
                "default" => PriceMode::Default,
                "fixedDecimals" => PriceMode::FixedDecimals,
                "assetDecimals" => PriceMode::AssetDecimals,
                _ => return Err(UnsupportedOperation("unknown price mode".to_owned())),
            },
            // https://docs.waves.tech/en/blockchain/order#json-representation
            None => PriceMode::Default,
        };

        match version {
            3 => Ok(Order::V3(v3::OrderV3::new(
                sender.chain_id(),
                timestamp as u64,
                sender_public_key,
                Amount::new(matcher_fee as u64, matcher_fee_asset_id),
                order_type,
                Amount::new(amount as u64, amount_asset),
                Amount::new(price as u64, price_asset),
                matcher_public_key,
                expiration as u64,
            ))),
            4 => Ok(Order::V4(v4::OrderV4::new(
                sender.chain_id(),
                timestamp as u64,
                sender_public_key,
                Amount::new(matcher_fee as u64, matcher_fee_asset_id),
                order_type,
                Amount::new(amount as u64, amount_asset),
                Amount::new(price as u64, price_asset),
                matcher_public_key,
                expiration as u64,
                price_mode,
            ))),
            _ => Err(Error::UnsupportedOrderVersion),
        }
    }
}

fn map_asset_pair(order: &OrderV4) -> Option<AssetPair> {
    Some(AssetPair {
        amount_asset_id: order
            .amount()
            .asset_id()
            .map(|asset| asset.bytes())
            .unwrap_or_default(),
        price_asset_id: order
            .price()
            .asset_id()
            .map(|asset| asset.bytes())
            .unwrap_or_default(),
    })
}

fn map_order_side(order: &OrderV4) -> i32 {
    match order.order_type() {
        OrderType::Buy => 0,
        OrderType::Sell => 1,
    }
}

fn map_matcher_fee(order: &OrderV4) -> Option<ProtoAmount> {
    Some(ProtoAmount {
        asset_id: order
            .fee()
            .asset_id()
            .map(|asset| asset.bytes())
            .unwrap_or_default(),
        amount: order.fee().value() as i64,
    })
}

fn map_price_mode(order: &OrderV4) -> i32 {
    match order.price_mode() {
        PriceMode::Default => 0,
        PriceMode::FixedDecimals => 1,
        PriceMode::AssetDecimals => 2,
    }
}
