use waves_rust::api::{Node, Profile};
use waves_rust::model::{
    Address, Amount, Base58String, ByteString, ChainId, PrivateKey, Transaction, TransactionData,
    TransferTransaction,
};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

//#[tokio::test]
async fn broadcast_and_read_test() {
    let private_key =
        PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private ket from seed phrase");

    let recipient = Address::from_string(
        &private_key
            .public_key()
            .address(ChainId::TESTNET.byte())
            .expect("failed to get public key")
            .encoded(),
    )
    .expect("failed to get address from public key");
    let transaction_data = TransactionData::Transfer(TransferTransaction::new(
        recipient,
        Amount::new(1, None),
        Base58String::empty(),
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(100000, None),
        timestamp,
        private_key.public_key(),
        3,
        ChainId::TESTNET.byte(),
    )
    .sign(&private_key)
    .expect("failed to sign transaction");

    let node = Node::from_profile(Profile::TESTNET);
    let signed_tx_from_rs = node.broadcast(&signed_tx).await;

    match signed_tx_from_rs {
        Ok(signed_tx_from_rs) => {
            assert_eq!(
                signed_tx_from_rs
                    .id()
                    .expect("failed to calculate tx id")
                    .encoded(),
                signed_tx.id().expect("failed to calculate id").encoded()
            )
        }
        Err(err) => println!("{:?}", err),
    }
}
