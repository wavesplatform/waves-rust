use crate::error::Result;
use crate::model::account::PrivateKey;
use crate::model::{Order, SignedOrder, SignedTransaction, Transaction};
use crate::util::BinarySerializer;

pub fn sign_tx(transaction: &Transaction, private_key: &PrivateKey) -> Result<SignedTransaction> {
    let bytes = BinarySerializer::tx_body_bytes(transaction);
    Ok(SignedTransaction::new(
        transaction.clone(),
        vec![private_key.sign(&bytes?)?],
    ))
}

pub fn sign_order(order: &Order, private_key: &PrivateKey) -> Result<SignedOrder> {
    let bytes = BinarySerializer::order_body_byte(order);
    Ok(SignedOrder::new(
        order.clone(),
        vec![private_key.sign(&bytes?)?],
    ))
}

#[cfg(test)]
mod tests {
    use crate::model::account::PrivateKey;
    use crate::model::data_entry::DataEntry;
    use crate::model::{Amount, ChainId, DataTransaction, Transaction, TransactionData};
    use crate::util::{sign_tx, Base58};

    const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
    trigger used census";

    #[test]
    fn test_sign_data_transaction() {
        let private_key =
            PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private key from seed");

        let binary_value: [u8; 12] = [0; 12];

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
                println!("signature {}", Base58::encode(&signature, false));

                let is_signature_valid = private_key
                    .is_signature_valid(
                        &transaction.bytes().expect("failed to get body bytes"),
                        &signature,
                    )
                    .expect("failed to validate signature");

                assert_eq!(true, is_signature_valid);
            }
            Err(err) => println!("{:?}", err),
        }
    }
}
