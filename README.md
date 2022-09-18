# waves-rust
A Rust library for interacting with the Waves blockchain.

Supports node interaction, offline transaction signing and creating addresses and keys.

## Using waves-rust in your project
Use the code below to add waves-rust as a dependency for your project.

##### Cargo:
```cargo
waves-rust = "0.1.0"
```

### Getting started
Create an account from a private key ('T' for testnet) from random seed phrase:
```rust
use waves_rust::model::{ChainId, PrivateKey};
use waves_rust::util::Crypto;

let seed_phrase = Crypto::get_random_seed_phrase(12);
let private_key = PrivateKey::from_seed(&seed_phrase, 0).unwrap();
let public_key = private_key.public_key();
let address = public_key.address(ChainId::TESTNET.byte()).unwrap();
```

Create a Node and learn a few things about blockchain:
```rust
#[tokio::main]
async fn get_node_info() {
    let node = Node::from_profile(Profile::TESTNET);
    println!("Current height is {}", node.get_height().await.unwrap());
    println!("My balance is {}", node.get_balance(&address).await.unwrap());
    println!("With 100 confirmations: {}", node.get_balance_with_confirmations(&address, 100).await.unwrap());
}
```

Send some money to a buddy:
```rust
let buddy = Address::from_string("3N2yqTEKArWS3ySs2f6t8fpXdjX6cpPuhG8").unwrap();

let transaction_data = TransactionData::Transfer(TransferTransaction::new(
    buddy,
    Amount::new(1_00_000_000, None), // None is WAVES asset
    Base58String::from_string("thisisattachment".to_owned()).unwrap(),
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
.unwrap();

node.broadcast(&signed_tx).await.unwrap();
```

Set a script on an account. Be careful with the script you pass here, as it may lock the account forever!
```rust
let script =
"{-# CONTENT_TYPE EXPRESSION #-} sigVerify(tx.bodyBytes, tx.proofs[0], tx.senderPublicKey)";

let compiled_script = node.compile_script(script, true).await.unwrap();
let transaction_data =
TransactionData::SetScript(SetScriptTransaction::new(compiled_script.script()));

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
.unwrap();

node.broadcast(&signed_tx).await.unwrap();
```

### Reading transaction info
[Same transaction from REST API](https://nodes-stagenet.wavesnodes.com/transactions/info/CWuFY42te67sLmc5gwt4NxwHmFjVfJdHkKuLyshTwEct)

```rust
let node = Node::from_profile(Profile::STAGENET);

let id = Id::from_string("CWuFY42te67sLmc5gwt4NxwHmFjVfJdHkKuLyshTwEct").unwrap();
let tx_info = node.get_transaction_info(&id).await.unwrap();

println!("type: {:?}", tx_info.tx_type());
println!("id: {:?}", tx_info.id());
println!("fee: {:?}", tx_info.fee().value());
println!("feeAssetId: {:?}", tx_info.fee().asset_id());
println!("timestamp: {:?}", tx_info.timestamp());
println!("version: {:?}", tx_info.version());
println!("chainId: {:?}", tx_info.chain_id());
println!("sender: {:?}",tx_info.public_key().address(ChainId::STAGENET.byte()).unwrap().encoded());
println!("senderPublicKey: {:?}", tx_info.public_key().encoded());
println!("height: {:?}", tx_info.height());
println!("applicationStatus: {:?}", tx_info.status());

let eth_tx = match tx_info.data() {
    TransactionDataInfo::Ethereum(eth_tx) => eth_tx,
    _ => panic!("expected ethereum transaction"),
};

println!("bytes: {}", eth_tx.bytes().encoded());
println!("{:?}", eth_tx.payload());
```

### Broadcasting transactions
#### Creating accounts (see Getting started for more info about account creation)
```rust
let bob = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0);
let alice = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0);
```
#### Broadcasting exchange transaction
```rust
let price = Amount::new(1000, None);
let amount = Amount::new(
    100,
    Some(AssetId::from_string("8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6").unwrap()),
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
.unwrap();

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
.unwrap();

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
.unwrap();

let tx_info = node.broadcast(&signed_tx).await.unwrap();
```

### Working with dApp
#### Creating accounts (see Getting started for more info about account creation)
```rust
let bob = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0);
let alice = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0);
```
#### Broadcasting issue transaction
```rust
let transaction_data = TransactionData::Issue(IssueTransaction::new(
        "Asset".to_owned(),
        "this is test asset".to_owned(),
        1000,
        2,
        false,
        None,
));

let timestamp = get_current_epoch_millis();
let signed_tx = Transaction::new(
    transaction_data,
    Amount::new(100400000, None),
    timestamp,
    alice.public_key(),
    3,
    ChainId::TESTNET.byte(),
)
.sign(&alice)
.unwrap();

node.broadcast(&signed_tx).await.unwrap();
```

#### Compiling and broadcasting RIDE script
```rust
let script = r#"
        {-# STDLIB_VERSION 5 #-}
        {-# CONTENT_TYPE DAPP #-}
        {-# SCRIPT_TYPE ACCOUNT #-}
        
        @Callable(inv)
        func call(bv: ByteVector, b: Boolean, int: Int, str: String, list: List[Int]) = {
             let asset = Issue("Asset", "", 1, 0, true)
             let assetId = asset.calculateAssetId()
             let lease = Lease(inv.caller, 7)
             let leaseId = lease.calculateLeaseId()
             [
                BinaryEntry("bin", assetId),
                BooleanEntry("bool", true),
                IntegerEntry("int", 100500),
                StringEntry("assetId", assetId.toBase58String()),
                StringEntry("leaseId", leaseId.toBase58String()),
                StringEntry("del", ""),
                DeleteEntry("del"),
                asset,
                SponsorFee(assetId, 1),
                Reissue(assetId, 4, false),
                Burn(assetId, 3),
                ScriptTransfer(inv.caller, 2, assetId),
                lease,
                LeaseCancel(lease.calculateLeaseId())
             ]
        }
"#;

let compiled_script = node.compile_script(script, true).await.unwrap();

let transaction_data = TransactionData::SetScript(SetScriptTransaction::new(compiled_script.script()));

let timestamp = get_current_epoch_millis();
let signed_tx = Transaction::new(
    transaction_data,
    Amount::new(500000, None),
    timestamp,
    alice.public_key(),
    3,
    ChainId::TESTNET.byte(),
)
.sign(&alice)
.unwrap();

node.broadcast(&signed_tx).await.unwrap();
```

#### Calling dApp
```rust
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
```