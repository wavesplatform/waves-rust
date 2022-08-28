use waves_rust::model::account::PrivateKey;
use waves_rust::model::data_entry::DataEntry;
use waves_rust::model::{Amount, ChainId, DataTransaction, Transaction, TransactionData};
use waves_rust::node::{Node, Profile};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

//#[tokio::test]
async fn broadcast_and_read_test() {
    let private_key = PrivateKey::from_seed(SEED_PHRASE, 0);

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

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(100000, None),
        timestamp,
        private_key.public_key(),
        DataTransaction::tx_type(),
        2,
        ChainId::TESTNET.byte(),
    )
    .sign(&private_key);

    let node = Node::from_profile(Profile::TESTNET);
    let signed_tx_from_rs = node.broadcast(&signed_tx).await;

    if let Ok(signed_tx_from_rs) = signed_tx_from_rs {
        assert_eq!(signed_tx_from_rs.id().encoded(), signed_tx.id().encoded());
    } else {
        let node_error = signed_tx_from_rs.err().expect("No error");
        println!("{}", node_error);
    }
}
