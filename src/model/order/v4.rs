use crate::error::Result;
use crate::{
    model::{Amount, Id, PrivateKey, Proof, PublicKey},
    util::{sign_order, BinarySerializer, Hash},
};

use super::{Order, OrderType, PriceMode, SignedOrder};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OrderInfoV4 {
    id: Id,
    chain_id: u8,
    timestamp: u64,
    sender: PublicKey,
    fee: Amount,
    order_type: OrderType,
    amount: Amount,
    price: Amount,
    matcher: PublicKey,
    expiration: u64,
    proofs: Vec<Proof>,
    price_mode: PriceMode,
    //todo
    //eip_712_signature: Option<Vec<u8>>,
}

impl OrderInfoV4 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Id,
        chain_id: u8,
        timestamp: u64,
        sender: PublicKey,
        fee: Amount,
        order_type: OrderType,
        amount: Amount,
        price: Amount,
        matcher: PublicKey,
        expiration: u64,
        proofs: Vec<Proof>,
        price_mode: PriceMode,
    ) -> Self {
        Self {
            id,
            chain_id,
            timestamp,
            sender,
            fee,
            order_type,
            amount,
            price,
            matcher,
            expiration,
            proofs,
            price_mode,
        }
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn version(&self) -> u8 {
        4
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

    pub fn proofs(&self) -> Vec<Proof> {
        self.proofs.clone()
    }

    pub fn with_proofs(mut self, proofs: Vec<Proof>) -> Self {
        self.proofs = proofs;
        self
    }

    pub fn price_mode(&self) -> PriceMode {
        self.price_mode.clone()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OrderV4 {
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
    //eip_712_signature: Option<Vec<u8>>,
}

//todo add eip_712_signature read
#[allow(clippy::too_many_arguments)]
impl OrderV4 {
    pub fn new(
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
        Self {
            chain_id,
            timestamp,
            sender,
            fee,
            order_type,
            amount,
            price,
            matcher,
            expiration,
            price_mode,
        }
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn version(&self) -> u8 {
        4
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

    pub fn price_mode(&self) -> PriceMode {
        self.price_mode.clone()
    }

    pub fn id(&self) -> Result<Id> {
        Ok(Id::from_bytes(&Hash::blake(&self.bytes()?)?))
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        BinarySerializer::order_body_bytes(&Order::V4(self.clone()))
    }

    // pub fn eip_712_signature(&self) -> Option<Vec<u8>> {
    //     self.eip_712_signature.clone()
    // }

    pub fn sign(&self, private_key: &PrivateKey) -> Result<SignedOrder> {
        sign_order(&Order::V4(self.clone()), private_key)
    }

    pub fn default_expiration(current_time: u64) -> u64 {
        current_time + (30 * 24 * 60 * 60 * 1000)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Amount, AssetId, ByteString, PublicKey};

    use super::OrderV4;

    #[test]
    fn test_order_id_calculation() {
        let order = OrderV4 {
            chain_id: 84,
            timestamp: 1666571041063,
            sender: PublicKey::from_string("BDSyopLzAjMYvQSm4XuMA2gtjP5TPoZMWQ1sxnzTE1Y8")
                .expect("failed to parse sender public key"),
            fee: Amount::new(
                99143,
                Some(
                    AssetId::from_string("25FEqEjRkqK6yCkiT7Lz6SAYz7gUFCtxfCChnrVFD5AT")
                        .expect("failed to parse asset id"),
                ),
            ),
            order_type: super::OrderType::Buy,
            amount: Amount::new(660949620, None),
            price: Amount::new(
                15000000,
                Some(
                    AssetId::from_string("25FEqEjRkqK6yCkiT7Lz6SAYz7gUFCtxfCChnrVFD5AT")
                        .expect("failed to parse asset id"),
                ),
            ),
            price_mode: super::PriceMode::AssetDecimals,
            matcher: PublicKey::from_string("8QUAqtTckM5B8gvcuP7mMswat9SjKUuafJMusEoSn1Gy")
                .expect("failed to parse matcher public key"),
            expiration: 1669080241063,
        };

        assert_eq!(
            "3DCDNkx3iw9UBhKfQgibxrCes1uXPeMaexpgf5kQyz18",
            order.id().expect("failed to calculate order id").encoded()
        );
    }
}
