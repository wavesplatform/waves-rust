

use crate::error::Error::UnsupportedTransactionVersion;
use crate::error::Result;
use crate::model::account::PrivateKey;
use crate::model::{
    Order, Proof, SignedOrder, SignedTransaction, Transaction,
};
use crate::util::BinarySerializer;

pub fn sign_tx(transaction: &Transaction, private_key: &PrivateKey) -> Result<SignedTransaction> {
    check_version(transaction)?;
    let bytes = BinarySerializer::tx_body_bytes(transaction);
    Ok(SignedTransaction::new(
        transaction.clone(),
        vec![Proof::new(private_key.sign(&bytes?)?)],
    ))
}

pub fn sign_order(order: &Order, private_key: &PrivateKey) -> Result<SignedOrder> {
    let bytes = BinarySerializer::order_body_bytes(order);
    Ok(SignedOrder::new(
        order.clone(),
        vec![Proof::new(private_key.sign(&bytes?)?)],
    ))
}

fn check_version(transaction: &Transaction) -> Result<()> {
    let tx_version = transaction.version();
    let min_supported_version = transaction.data().get_min_supported_version();
    if tx_version < min_supported_version {
        return Err(UnsupportedTransactionVersion {
            actual_version: tx_version,
            supported_version: min_supported_version,
            tx_type: transaction.data().clone(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::model::account::PrivateKey;
    use crate::model::data_entry::DataEntry;
    use crate::model::{
        Amount, ByteString, ChainId, DataTransaction, Transaction, TransactionData,
    };
    use crate::util::sign_tx;

    const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
    trigger used census";

    #[test]
    fn test_sign_data_transaction() {
        let private_key =
            PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private key from seed");

        let transaction = Transaction::new(
            TransactionData::Data(create_data_tx()),
            Amount::new(100000, None),
            1661456063029,
            private_key.public_key(),
            2,
            ChainId::TESTNET.byte(),
        );

        let signed_tx = sign_tx(&transaction, &private_key);

        match signed_tx {
            Ok(success) => {
                let signature = success.proofs()[0].to_owned();

                let is_signature_valid = private_key
                    .is_signature_valid(
                        &transaction.bytes().expect("failed to get body bytes"),
                        &signature.bytes(),
                    )
                    .expect("failed to validate signature");

                assert_eq!(true, is_signature_valid);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[test]
    fn test_when_tx_version_less_than_min_supported_return_sign_err() {
        let private_key =
            PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private key from seed");

        let transaction = Transaction::new(
            TransactionData::Data(create_data_tx()),
            Amount::new(100000, None),
            1661456063029,
            private_key.public_key(),
            1,
            ChainId::TESTNET.byte(),
        );

        match sign_tx(&transaction, &private_key) {
            Ok(_) => {
                panic!("Error expected")
            }
            Err(err) => match err {
                Error::UnsupportedTransactionVersion {
                    actual_version,
                    supported_version,
                    tx_type,
                } => {
                    assert_eq!(actual_version, 1);
                    assert_eq!(supported_version, 2);
                    match tx_type {
                        TransactionData::Data(_) => {}
                        _ => panic!("DataTransaction expected"),
                    }
                }
                _ => panic!("UnsupportedTransactionVersion error expected"),
            },
        }
    }

    #[test]
    fn test_when_tx_version_greater_than_min_supported_return_sign_ok() {
        let private_key =
            PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private key from seed");

        let transaction = Transaction::new(
            TransactionData::Data(create_data_tx()),
            Amount::new(100000, None),
            1661456063029,
            private_key.public_key(),
            3,
            ChainId::TESTNET.byte(),
        );

        match sign_tx(&transaction, &private_key) {
            Ok(_) => {}
            Err(_) => {
                panic!("Ok expected")
            }
        }
    }

    #[test]
    fn test_when_tx_version_eq_min_supported_return_sign_ok() {
        let private_key =
            PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private key from seed");

        let transaction = Transaction::new(
            TransactionData::Data(create_data_tx()),
            Amount::new(100000, None),
            1661456063029,
            private_key.public_key(),
            2,
            ChainId::TESTNET.byte(),
        );

        match sign_tx(&transaction, &private_key) {
            Ok(_) => {}
            Err(_) => {
                panic!("Ok expected")
            }
        }
    }

    fn create_data_tx() -> DataTransaction {
        DataTransaction::new(vec![
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
                value: [0; 12].to_vec(),
            },
            DataEntry::StringEntry {
                key: "str".to_string(),
                value: "value".to_string(),
            },
        ])
    }
}
