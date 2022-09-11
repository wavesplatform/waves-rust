use regex::Regex;
use waves_rust::api::{Node, Profile};
use waves_rust::model::{Address, ByteString};

#[tokio::test]
async fn get_addresses_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let addresses = node.get_addresses().await.unwrap();
    let address = addresses.first().unwrap();
    println!("{}", address.encoded());
}

#[tokio::test]
async fn get_addresses_seq_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let addresses = node.get_addresses_seq(0, 1).await.unwrap();
    let address = addresses.first().unwrap();
    println!("{}", address.encoded());
}

#[tokio::test]
async fn get_balance_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balance = node
        .get_balance(&Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap())
        .await
        .unwrap();
    assert_eq!(true, balance > 0)
}

#[tokio::test]
async fn get_balance_with_confirmation_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balance = node
        .get_balance_with_confirmations(
            &Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap(),
            100,
        )
        .await;

    match balance {
        Ok(result) => assert_eq!(true, result > 0),
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_balances_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balances = node
        .get_balances(&vec![
            Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap(),
            Address::from_string("3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW").unwrap(),
        ])
        .await;

    match balances {
        Ok(result) => {
            for balance in result {
                println!("{:?}", balance);
            }
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_balances_at_height_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balances = node
        .get_balances_at_height(
            &vec![
                Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap(),
                Address::from_string("3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW").unwrap(),
            ],
            2202560,
        )
        .await;
    match balances {
        Ok(result) => {
            for balance in result {
                println!("{:?}", balance);
            }
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_balance_details_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balance_details = node
        .get_balance_details(&Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap())
        .await;
    match balance_details {
        Ok(result) => {
            println!("address: {}", result.address().encoded());
            println!("available: {}", result.available());
            println!("regular: {}", result.regular());
            println!("generating: {}", result.generating());
            println!("effective: {}", result.effective());
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_address_data_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let data_entries = node
        .get_data(&Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap())
        .await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_address_data_by_keys_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    let data_entries = node
        .get_data_by_keys(&address, &vec!["binary".to_owned(), "bool".to_owned()])
        .await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_address_data_by_regex_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    let regex = Regex::new(r"b\w+").unwrap();
    let data_entries = node.get_data_by_regex(&address, &regex).await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_address_data_by_key_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    let data_entrie = node.get_data_by_key(&address, "bool").await;

    match data_entrie {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_script_info_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mv1HwsRtMjyGKSe5DSDnbT2AoTsXAjtwZS").unwrap();
    let script_info = node.get_script_info(&address).await;
    match script_info {
        Ok(result) => {
            println!("{}", result.script().encoded());
            println!("{}", result.complexity());
            println!("{}", result.verifier_complexity());
            println!("{:?}", result.callable_complexities());
            println!("{}", result.extra_fee());
            println!("{}", result.script_text());
        }
        Err(err) => println!("{:?}", err),
    }
}

#[tokio::test]
async fn get_script_meta_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mv1HwsRtMjyGKSe5DSDnbT2AoTsXAjtwZS").unwrap();
    let script_meta = node.get_script_meta(&address).await;
    match script_meta {
        Ok(result) => {
            println!("{}", result.meta_version());
            println!("{:#?}", result.callable_functions());
        }
        Err(err) => println!("{:?}", err),
    }
}
