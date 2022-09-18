use waves_rust::api::{Node, Profile};
use waves_rust::model::Arg::{Binary, Boolean, Integer, List, String};
use waves_rust::model::{
    Address, Amount, AssetId, Base58String, Base64String, ByteString, ChainId, Function, Id,
    InvokeScriptTransaction, IssueTransaction, PrivateKey, SetScriptTransaction, Transaction,
    TransactionData, TransactionDataInfo, TransferTransaction,
};
use waves_rust::util::{get_current_epoch_millis, Crypto};

#[tokio::test]
async fn get_node_info() {
    let seed_phrase = Crypto::get_random_seed_phrase(12);
    let alice = PrivateKey::from_seed("a", 0).unwrap();
    let bob = PrivateKey::from_seed("b", 0).unwrap();
    println!("{:?}", bob.public_key().address(ChainId::TESTNET.byte()));
    let public_key = alice.public_key();
    let address = public_key.address(ChainId::TESTNET.byte()).unwrap();
    println!("{:?}", address);
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

//InvokeScriptTransaction tx = InvokeScriptTransaction
//         .builder(bob.address(), Function.as("call",
//                 BinaryArg.as(alice.address().bytes()),
//                 BooleanArg.as(true),
//                 IntegerArg.as(100500),
//                 StringArg.as(alice.address().toString()),
//                 ListArg.as(IntegerArg.as(100500))
//         )).payments(
//                 Amount.of(1, assetId),
//                 Amount.of(2, assetId),
//                 Amount.of(3, assetId),
//                 Amount.of(4, assetId),
//                 Amount.of(5, assetId),
//                 Amount.of(6, assetId),
//                 Amount.of(7, assetId),
//                 Amount.of(8, assetId),
//                 Amount.of(9, assetId),
//                 Amount.of(10, assetId)
//         ).extraFee(1_00000000)
//         .getSignedWith(alice);
// node.waitForTransaction(node.broadcast(tx).id());

//Base64String script = node.compileScript(
//         "{-# STDLIB_VERSION 5 #-}\n" +
//         "{-# CONTENT_TYPE DAPP #-}\n" +
//         "{-# SCRIPT_TYPE ACCOUNT #-}\n" +
//         "@Callable(inv)\n" +
//         "func call(bv: ByteVector, b: Boolean, int: Int, str: String, list: List[Int]) = {\n" +
//         "  let asset = Issue(\"Asset\", \"\", 1, 0, true)\n" +
//         "  let assetId = asset.calculateAssetId()\n" +
//         "  let lease = Lease(inv.caller, 7)\n" +
//         "  let leaseId = lease.calculateLeaseId()\n" +
//         "  [\n" +
//         "    BinaryEntry(\"bin\", assetId),\n" +
//         "    BooleanEntry(\"bool\", true),\n" +
//         "    IntegerEntry(\"int\", 100500),\n" +
//         "    StringEntry(\"assetId\", assetId.toBase58String()),\n" +
//         "    StringEntry(\"leaseId\", leaseId.toBase58String()),\n" +
//         "    StringEntry(\"del\", \"\"),\n" +
//         "    DeleteEntry(\"del\"),\n" +
//         "    asset,\n" +
//         "    SponsorFee(assetId, 1),\n" +
//         "    Reissue(assetId, 4, false),\n" +
//         "    Burn(assetId, 3),\n" +
//         "    ScriptTransfer(inv.caller, 2, assetId),\n" +
//         "    lease,\n" +
//         "    LeaseCancel(lease.calculateLeaseId())\n" +
//         "  ]\n" +
//         "}").script();
// node.waitForTransaction(node.broadcast(
//         SetScriptTransaction.builder(script).getSignedWith(bob)).id());
