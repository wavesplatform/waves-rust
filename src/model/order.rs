use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId, Id, PrivateKey, PublicKey};
use crate::util::{sign_order, Base58, BinarySerializer, Hash, JsonDeserializer};
use crate::waves_proto::order::Sender::SenderPublicKey;
use crate::waves_proto::{Amount as ProtoAmount, AssetPair, Order as ProtoOrder};
use serde_json::{Map, Value};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OrderInfo {
    id: Id,
    chain_id: u8,
    version: u8,
    timestamp: u64,
    sender: PublicKey,
    fee: Amount,
    order_type: OrderType,
    amount: Amount,
    price: Amount,
    matcher: PublicKey,
    expiration: u64,
    proofs: Vec<Vec<u8>>,
    //eip_712_signature: Option<Vec<u8>>,
}

impl OrderInfo {
    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn sender(&self) -> PublicKey {
        self.sender.clone()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn fee(&self) -> Amount {
        self.fee.clone()
    }

    pub fn order_type(&self) -> OrderType {
        self.order_type.clone()
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn price(&self) -> Amount {
        self.price.clone()
    }

    pub fn matcher(&self) -> PublicKey {
        self.matcher.clone()
    }

    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn proofs(&self) -> Vec<Vec<u8>> {
        self.proofs.clone()
    }
}

impl TryFrom<&Value> for OrderInfo {
    type Error = Error;

    fn try_from(order_json: &Value) -> Result<Self> {
        let signed_order: SignedOrder = order_json.try_into()?;
        let id = JsonDeserializer::safe_to_string_from_field(order_json, "id")?;
        Ok(OrderInfo {
            id: Id::from_string(&id)?,
            chain_id: signed_order.order.chain_id,
            version: signed_order.order.version,
            timestamp: signed_order.order.timestamp,
            sender: signed_order.order.sender,
            fee: signed_order.order.fee,
            order_type: signed_order.order.order_type,
            amount: signed_order.order.amount,
            price: signed_order.order.price,
            matcher: signed_order.order.matcher,
            expiration: signed_order.order.expiration,
            proofs: signed_order.proofs,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SignedOrder {
    order: Order,
    proofs: Vec<Vec<u8>>,
}

impl SignedOrder {
    pub fn new(order: Order, proofs: Vec<Vec<u8>>) -> SignedOrder {
        SignedOrder { order, proofs }
    }

    pub fn order(&self) -> Order {
        self.order.clone()
    }

    pub fn proofs(&self) -> Vec<Vec<u8>> {
        self.proofs.clone()
    }

    pub fn id(&self) -> Result<Id> {
        self.order.id()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Order {
    chain_id: u8,
    version: u8,
    timestamp: u64,
    sender: PublicKey,
    fee: Amount,
    order_type: OrderType,
    amount: Amount,
    price: Amount,
    matcher: PublicKey,
    expiration: u64,
    //eip_712_signature: Option<Vec<u8>>,
}

//todo add eip_712_signature read
#[allow(clippy::too_many_arguments)]
impl Order {
    pub fn new(
        chain_id: u8,
        version: u8,
        timestamp: u64,
        sender: PublicKey,
        fee: Amount,
        order_type: OrderType,
        amount: Amount,
        price: Amount,
        matcher: PublicKey,
        expiration: u64,
    ) -> Order {
        Order {
            chain_id,
            version,
            timestamp,
            sender,
            fee,
            order_type,
            amount,
            price,
            matcher,
            expiration,
        }
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn sender(&self) -> PublicKey {
        self.sender.clone()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn fee(&self) -> Amount {
        self.fee.clone()
    }

    pub fn order_type(&self) -> OrderType {
        self.order_type.clone()
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn price(&self) -> Amount {
        self.price.clone()
    }

    pub fn matcher(&self) -> PublicKey {
        self.matcher.clone()
    }

    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    pub fn id(&self) -> Result<Id> {
        Ok(Id::from_bytes(&Hash::blake(&self.bytes()?)?))
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        BinarySerializer::order_body_byte(self)
    }

    // pub fn eip_712_signature(&self) -> Option<Vec<u8>> {
    //     self.eip_712_signature.clone()
    // }

    pub fn sign(&self, private_key: &PrivateKey) -> Result<SignedOrder> {
        sign_order(self, private_key)
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
        order_json.insert(
            "matcherPublicKey".to_owned(),
            order.matcher().encoded().into(),
        );
        order_json.insert("matcherFee".to_owned(), order.fee().value().into());
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
        let signature = Base58::encode(&signed_order.proofs[0], false);
        order_json.insert("signature".to_owned(), signature.clone().into());
        order_json.insert("proofs".to_owned(), vec![Value::String(signature)].into());
        Ok(order_json.into())
    }
}

impl TryFrom<&SignedOrder> for ProtoOrder {
    type Error = Error;

    fn try_from(signed_order: &SignedOrder) -> Result<Self> {
        let order = signed_order.order();

        Ok(ProtoOrder {
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
            proofs: signed_order.proofs(),
            //todo price_mode what to do?
            price_mode: 0,
            sender: Some(SenderPublicKey(order.sender().bytes())),
        })
    }
}

impl TryFrom<&Order> for ProtoOrder {
    type Error = Error;

    fn try_from(order: &Order) -> Result<Self> {
        Ok(ProtoOrder {
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
            //todo proofs price_mode what to do?
            proofs: vec![],
            price_mode: 0,
            sender: Some(SenderPublicKey(order.sender().bytes())),
        })
    }
}

impl TryFrom<&Value> for SignedOrder {
    type Error = Error;

    fn try_from(signed_order_json: &Value) -> Result<Self> {
        let order: Order = signed_order_json.try_into()?;
        let signature = Base58::decode(&JsonDeserializer::safe_to_string_from_field(
            signed_order_json,
            "signature",
        )?)?;
        Ok(SignedOrder::new(order, vec![signature]))
    }
}

impl TryFrom<&Value> for Order {
    type Error = Error;

    fn try_from(order_json: &Value) -> Result<Self> {
        let order_type =
            match JsonDeserializer::safe_to_string_from_field(order_json, "orderType")?.as_str() {
                "buy" => OrderType::Buy,
                "sell" => OrderType::Sell,
                _ => panic!("unknown order type"),
            };
        let version = JsonDeserializer::safe_to_int_from_field(order_json, "version")?;
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

        Ok(Order::new(
            sender.chain_id(),
            version as u8,
            timestamp as u64,
            sender_public_key,
            Amount::new(matcher_fee as u64, matcher_fee_asset_id),
            order_type,
            Amount::new(amount as u64, amount_asset),
            Amount::new(price as u64, price_asset),
            matcher_public_key,
            expiration as u64,
        ))
    }
}

fn map_asset_pair(order: &Order) -> Option<AssetPair> {
    Some(AssetPair {
        amount_asset_id: order
            .amount
            .asset_id()
            .map(|asset| asset.bytes())
            .unwrap_or_default(),
        price_asset_id: order
            .price
            .asset_id()
            .map(|asset| asset.bytes())
            .unwrap_or_default(),
    })
}

fn map_order_side(order: &Order) -> i32 {
    match order.order_type {
        OrderType::Buy => 0,
        OrderType::Sell => 1,
    }
}

fn map_matcher_fee(order: &Order) -> Option<ProtoAmount> {
    Some(ProtoAmount {
        asset_id: order
            .fee()
            .asset_id()
            .map(|asset| asset.bytes())
            .unwrap_or_default(),
        amount: order.fee().value() as i64,
    })
}
