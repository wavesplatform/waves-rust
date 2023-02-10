use crate::model::{Amount, ChainId, PrivateKey, PublicKey, SignedTransaction, Transaction, TransactionData};
use crate::error::Result;
use crate::util::get_current_epoch_millis;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SignedTransactionBuilder {
    data: TransactionData,
    fee: Option<Amount>,
    timestamp: Option<u64>,
    public_key: Option<PublicKey>,
    version: Option<u8>,
    chain_id: u8,
}

impl SignedTransactionBuilder {
    pub fn new(data: TransactionData, chain_id: u8) -> SignedTransactionBuilder {
        SignedTransactionBuilder {
            data,
            fee: None,
            timestamp: None,
            public_key: None,
            version: None,
            chain_id,
        }
    }

    pub fn fee(&mut self, fee: Amount) -> &mut Self {
        self.fee = Some(fee);
        self
    }

    pub fn timestamp(&mut self, timestamp: u64) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn public_key(&mut self, public_key: PublicKey) -> &mut Self {
        self.public_key = Some(public_key);
        self
    }

    pub fn version(&mut self, version: u8) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn build(&self, private_key: &PrivateKey) -> Result<SignedTransaction> {
        let transaction_data = self.data.clone();
        let fee = match self.fee.clone() {
            Some(fee) => fee,
            None => transaction_data.get_min_fee()?
        };

        let timestamp = match self.timestamp {
            Some(timestamp) => timestamp,
            None => get_current_epoch_millis()
        };

        let public_key = match self.public_key.clone() {
            Some(public_key) => public_key,
            None => private_key.public_key()
        };

        let version = match self.version {
            Some(version) => version,
            None => transaction_data.get_min_supported_version()
        };

        Transaction::new(
            transaction_data,
            fee,
            timestamp,
            public_key,
            version,
            self.chain_id,
        ).sign(private_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Amount, BurnTransaction, ChainId, PrivateKey, PublicKey, SignedTransactionBuilder, TransactionData};

    #[test]
    fn test_builder_default_params() {
        let private_key = PrivateKey::from_seed("123", 0).unwrap();

        let burn_transaction = BurnTransaction::new(
            Amount::new(1, None)
        );

        let signed_tx = SignedTransactionBuilder::new(
            TransactionData::Burn(burn_transaction),
            ChainId::TESTNET.byte(),
        ).build(&private_key).unwrap();

        assert_eq!(signed_tx.tx().fee().value(), 100_000);
        assert_eq!(signed_tx.tx().public_key(), private_key.public_key());
        assert_eq!(signed_tx.tx().tx_type(), 6);
        assert_eq!(signed_tx.tx().version(), 3);
        assert_eq!(signed_tx.tx().chain_id(), ChainId::TESTNET.byte())
    }

    #[test]
    fn test_builder_user_defined_params() {
        let private_key = PrivateKey::from_seed("123", 0).unwrap();

        let burn_transaction = BurnTransaction::new(
            Amount::new(1, None)
        );
        let pk = PublicKey::from_string("aaaaaaa").unwrap();
        let signed_tx = SignedTransactionBuilder::new(
            TransactionData::Burn(burn_transaction),
            ChainId::TESTNET.byte(),
        )
            .fee(Amount::new(10, None))
            .timestamp(100)
            .public_key(pk.clone())
            .version(4)
            .build(&private_key).unwrap();


        assert_eq!(signed_tx.tx().fee().value(), 10);
        assert_eq!(signed_tx.tx().timestamp(), 100);
        assert_eq!(signed_tx.tx().public_key(), pk);
        assert_eq!(signed_tx.tx().tx_type(), 6);
        assert_eq!(signed_tx.tx().version(), 4);
        assert_eq!(signed_tx.tx().chain_id(), ChainId::TESTNET.byte())
    }
}