use waves_rust::model::ChainId;
use waves_rust::node::{Node, Profile};

#[tokio::test]
async fn get_addresses_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let addresses = node.get_addresses(ChainId::TESTNET.byte()).await.unwrap();
    let address = addresses.first().unwrap();
    println!("{}", address.encoded());
}

#[tokio::test]
async fn get_addresses_seq_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let addresses = node
        .get_addresses_seq(0, 1, ChainId::TESTNET.byte())
        .await
        .unwrap();
    let address = addresses.first().unwrap();
    println!("{}", address.encoded());
}
