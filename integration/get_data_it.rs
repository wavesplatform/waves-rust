use regex::Regex;
use waves_rust::model::account::Address;
use waves_rust::model::ChainId;
use waves_rust::node::{Node, Profile};

#[tokio::test]
async fn get_address_data_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let data_entries = node
        .get_data(&Address::from_string(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            ChainId::TESTNET.byte(),
        ))
        .await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{}", err),
    }
}

#[tokio::test]
async fn get_address_data_by_keys_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string(
        "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
        ChainId::TESTNET.byte(),
    );
    let data_entries = node
        .get_data_by_keys(&address, &vec!["binary".to_owned(), "bool".to_owned()])
        .await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{}", err),
    }
}

#[tokio::test]
async fn get_address_data_by_regex_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string(
        "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
        ChainId::TESTNET.byte(),
    );
    let regex = Regex::new(r"b\w+").unwrap();
    let data_entries = node.get_data_by_regex(&address, &regex).await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{}", err),
    }
}

#[tokio::test]
async fn get_address_data_by_key_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string(
        "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
        ChainId::TESTNET.byte(),
    );
    let data_entrie = node.get_data_by_key(&address, "bool").await;

    match data_entrie {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{}", err),
    }
}
