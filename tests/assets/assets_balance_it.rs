use waves_rust::api::{Node, Profile};
use waves_rust::model::Address;

#[tokio::test]
async fn get_address_data_by_keys_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    let data_entries = node.get_assets_balance(&address).await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}
