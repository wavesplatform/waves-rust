use waves_rust::api::{Node, Profile};
use waves_rust::model::{Address, Alias, ChainId};

#[ignore]
#[tokio::test]
async fn get_aliases_by_address_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK").unwrap();
    let aliases = node.get_aliases_by_address(&address).await;

    match aliases {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}

#[ignore]
#[tokio::test]
async fn get_address_by_alias_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let alias =
        Alias::new(ChainId::TESTNET.byte(), "alias1662650000377").expect("invalid alias name");
    let address = node.get_address_by_alias(&alias).await;

    match address {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}
