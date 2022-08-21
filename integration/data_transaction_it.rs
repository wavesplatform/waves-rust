use waves_rust::model::account::PrivateKey;
use waves_rust::model::data_entry::DataEntry;
use waves_rust::model::{Amount, DataTransaction, Transaction, TransactionData};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

#[test]
pub fn broadcast_and_read_test() {
    let private_key = PrivateKey::from_seed(SEED_PHRASE, 0);

    let transaction_data = TransactionData::Data(DataTransaction::new(vec![
        DataEntry::IntegerEntry {
            key: "12".to_string(),
            value: 12,
        },
        DataEntry::StringEntry {
            key: "string_entry".to_string(),
            value: "value".to_string(),
        },
    ]));

    let _transaction = Transaction::new(
        transaction_data,
        Amount::new(1000000, None),
        get_current_epoch_millis(),
        private_key.public_key(),
        DataTransaction::tx_type(),
        1,
        1,
    );
}
