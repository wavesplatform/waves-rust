use crate::error::Result;
use crate::model::{Amount, PublicKey, Transaction, TransactionData};
use crate::util::get_current_epoch_millis;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransactionBuilder {
    data: TransactionData,
    fee: Option<Amount>,
    timestamp: Option<u64>,
    public_key: PublicKey,
    version: Option<u8>,
    chain_id: u8,
}

impl TransactionBuilder {
    pub fn new(public_key: &PublicKey, chain_id: u8, data: &TransactionData) -> TransactionBuilder {
        TransactionBuilder {
            data: data.clone(),
            fee: None,
            timestamp: None,
            public_key: public_key.clone(),
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

    pub fn version(&mut self, version: u8) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn build(&self) -> Result<Transaction> {
        let transaction_data = self.data.clone();
        let fee = match self.fee.clone() {
            Some(fee) => fee,
            None => transaction_data.get_min_fee()?,
        };

        let timestamp = match self.timestamp {
            Some(timestamp) => timestamp,
            None => get_current_epoch_millis(),
        };

        let version = match self.version {
            Some(version) => version,
            None => transaction_data.get_min_supported_version(),
        };

        Ok(Transaction::new(
            transaction_data,
            fee,
            timestamp,
            self.public_key.clone(),
            version,
            self.chain_id,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{
        Amount, BurnTransaction, ChainId, PrivateKey, TransactionBuilder, TransactionData,
    };

    #[test]
    fn test_builder_default_params() {
        let private_key = PrivateKey::from_seed("123", 0).unwrap();

        let burn_transaction = BurnTransaction::new(Amount::new(1, None));

        let signed_tx = TransactionBuilder::new(
            &private_key.public_key(),
            ChainId::TESTNET.byte(),
            &TransactionData::Burn(burn_transaction),
        )
        .build()
        .unwrap()
        .sign(&private_key)
        .unwrap();

        assert_eq!(signed_tx.tx().fee().value(), 100_000);
        assert_eq!(signed_tx.tx().public_key(), private_key.public_key());
        assert_eq!(signed_tx.tx().tx_type(), 6);
        assert_eq!(signed_tx.tx().version(), 3);
        assert_eq!(signed_tx.tx().chain_id(), ChainId::TESTNET.byte())
    }

    #[test]
    fn test_builder_user_defined_params() {
        let private_key = PrivateKey::from_seed("123", 0).unwrap();

        let burn_transaction = BurnTransaction::new(Amount::new(1, None));
        let signed_tx = TransactionBuilder::new(
            &private_key.public_key(),
            ChainId::TESTNET.byte(),
            &TransactionData::Burn(burn_transaction),
        )
        .fee(Amount::new(10, None))
        .timestamp(100)
        .version(4)
        .build()
        .unwrap()
        .sign(&private_key)
        .unwrap();

        assert_eq!(signed_tx.tx().fee().value(), 10);
        assert_eq!(signed_tx.tx().timestamp(), 100);
        assert_eq!(signed_tx.tx().public_key(), private_key.public_key());
        assert_eq!(signed_tx.tx().tx_type(), 6);
        assert_eq!(signed_tx.tx().version(), 4);
        assert_eq!(signed_tx.tx().chain_id(), ChainId::TESTNET.byte())
    }
}
