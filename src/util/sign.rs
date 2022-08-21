use crate::model::account::PrivateKey;
use crate::model::{SignedTransaction, Transaction};
use crate::util::BinarySerializer;

pub fn sign(transaction: &Transaction, private_key: &PrivateKey) -> SignedTransaction {
    let bytes = BinarySerializer::body_bytes(transaction);
    SignedTransaction::new(transaction.clone(), vec![private_key.sign(&bytes)])
}

#[cfg(test)]
mod tests {
    use crate::model::account::PrivateKey;
    use crate::model::data_entry::DataEntry;
    use crate::model::{Amount, ChainId, DataTransaction, Transaction, TransactionData};
    use crate::util::sign;

    const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
    trigger used census";

    #[test]
    fn test_sign_data_transaction() {
        let private_key = PrivateKey::from_seed(SEED_PHRASE, 0);

        let binary_value: [u8; 32767] = [0; 32767];

        let transaction_data = TransactionData::Data(DataTransaction::new(vec![
            DataEntry::IntegerEntry {
                key: "int".to_string(),
                value: 12,
            },
            DataEntry::BooleanEntry {
                key: "bool".to_string(),
                value: false,
            },
            DataEntry::BinaryEntry {
                key: "binary".to_string(),
                value: binary_value.to_vec(),
            },
            DataEntry::StringEntry {
                key: "str".to_string(),
                value: "value".to_string(),
            },
        ]));

        let transaction = Transaction::new(
            transaction_data,
            Amount::new(3300000, None),
            1661111587962,
            private_key.public_key(),
            DataTransaction::tx_type(),
            2,
            ChainId::TESTNET.byte(),
        );

        let signed_tx = sign(&transaction, &private_key);

        let signature = signed_tx.proofs()[0].to_owned();

        let is_signature_valid = private_key.is_signature_valid(&transaction.bytes(), &signature);

        assert_eq!(true, is_signature_valid);
    }
}
