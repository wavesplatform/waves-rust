use waves_rust::api::{Node, Profile};
use waves_rust::model::Arg::{Binary, Boolean, Integer, List, String};
use waves_rust::model::{
    Address, Amount, Base64String, ByteString, ChainId, Function, InvokeScriptTransaction,
    PrivateKey, Transaction, TransactionData,
};
use waves_rust::util::{get_current_epoch_millis, Crypto};

#[tokio::main]
async fn get_node_info() {
    let bob = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0).unwrap();
    let alice = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0).unwrap();

    let node = Node::from_profile(Profile::TESTNET);

    let alice_address =
        Address::from_public_key(ChainId::TESTNET.byte(), &alice.public_key()).unwrap();
    let transaction_data = TransactionData::InvokeScript(InvokeScriptTransaction::new(
        alice_address.clone(),
        Function::new(
            "call".to_owned(),
            vec![
                Binary(Base64String::from_bytes(vec![1, 2, 3])),
                Boolean(true),
                Integer(100500),
                String(alice_address.encoded()),
                List(vec![Integer(100500)]),
            ],
        ),
        vec![
            Amount::new(1, None),
            Amount::new(2, None),
            Amount::new(3, None),
            Amount::new(4, None),
        ],
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(100500000, None),
        timestamp,
        bob.public_key(),
        3,
        ChainId::TESTNET.byte(),
    )
    .sign(&bob)
    .unwrap();

    node.broadcast(&signed_tx).await.unwrap();
}
