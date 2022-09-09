use waves_rust::model::{
    Amount, AssetId, ChainId, ExchangeTransaction, Order, OrderType, PrivateKey, Transaction,
    TransactionData,
};
use waves_rust::node::{Node, Profile};
use waves_rust::util::{get_current_epoch_millis, JsonSerializer};

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//#[tokio::test]
async fn broadcast_and_read_test() {
    let bob = PrivateKey::from_seed("b", 0).expect("failed to get private key from seed phrase");
    let alice = PrivateKey::from_seed("a", 0).expect("failed to get private key from seed phrase");

    let price = Amount::new(1000, None);
    let amount = Amount::new(
        100,
        Some(AssetId::from_string("8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6").expect("failed")),
    );

    let matcher_fee = 300000;

    let buy = Order::new(
        ChainId::TESTNET.byte(),
        4,
        get_current_epoch_millis(),
        alice.public_key(),
        Amount::new(300000, None),
        OrderType::Buy,
        amount.clone(),
        price.clone(),
        bob.public_key(),
        Order::default_expiration(get_current_epoch_millis()),
    )
    .sign(&alice)
    .expect("failed to sign");

    let sell = Order::new(
        ChainId::TESTNET.byte(),
        4,
        get_current_epoch_millis(),
        bob.public_key(),
        Amount::new(300000, None),
        OrderType::Sell,
        amount.clone(),
        price.clone(),
        bob.public_key(),
        Order::default_expiration(get_current_epoch_millis()),
    )
    .sign(&bob)
    .expect("failed to sign");

    let transaction_data = TransactionData::Exchange(ExchangeTransaction::new(
        buy.clone(),
        sell.clone(),
        amount.value(),
        price.value(),
        matcher_fee,
        matcher_fee,
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(300000, None),
        timestamp,
        bob.public_key(),
        4,
        ChainId::TESTNET.byte(),
    )
    .sign(&bob)
    .expect("failed to sign transaction");

    let node = Node::from_profile(Profile::TESTNET);
    let tx_info = node.broadcast(&signed_tx).await;

    match tx_info {
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
