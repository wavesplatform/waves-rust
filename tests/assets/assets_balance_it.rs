use waves_rust::api::{Node, Profile};
use waves_rust::model::Address;

#[tokio::test]
async fn get_assets_balance_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    let balance = node.get_assets_balance(&address).await;

    match balance {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}
