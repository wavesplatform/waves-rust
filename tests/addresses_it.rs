use regex::Regex;
use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;
use waves_rust::model::{Address, ByteString};

#[ignore]
#[tokio::test]
async fn get_addresses_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let addresses = node.get_addresses().await.unwrap();
    println!("{:?}", addresses);
}

#[ignore]
#[tokio::test]
async fn get_addresses_seq_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let addresses = node.get_addresses_seq(0, 1).await.unwrap();
    println!("{:?}", addresses);
}

#[ignore]
#[tokio::test]
async fn get_balance_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balance = node
        .get_balance(&Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap())
        .await
        .unwrap();
    assert_eq!(true, balance > 0)
}

#[ignore]
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

#[ignore]
#[tokio::test]
async fn get_balances_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let balances = node
        .get_balances(&[
            Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap(),
            Address::from_string("3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW").unwrap(),
        ])
        .await;
    println!("{:#?}", balances);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_balances_at_height_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let balances = node
        .get_balances_at_height(
            &[
                Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap(),
                Address::from_string("3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW").unwrap(),
            ],
            2224968,
        )
        .await?;
    println!("{:#?}", balances);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_balance_details_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let balance_details = node
        .get_balance_details(&Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap())
        .await;
    println!("{:#?}", balance_details);
    Ok(())
}

#[ignore]
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

#[ignore]
#[tokio::test]
async fn get_address_data_by_keys_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    let data_entries = node
        .get_data_by_keys(&address, &["binary".to_owned(), "bool".to_owned()])
        .await;

    match data_entries {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}

#[ignore]
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

#[ignore]
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

#[ignore]
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

#[ignore]
#[tokio::test]
async fn get_script_meta_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mv1HwsRtMjyGKSe5DSDnbT2AoTsXAjtwZS").unwrap();
    let script_meta = node.get_script_meta(&address).await?;
    println!("{:#?}", script_meta);
    Ok(())
}
