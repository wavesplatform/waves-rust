use serde_json::Value;
use waves_rust::json_serializer::{from_json, TransactionData};

fn main() {
    let tx_id = "8YsBZSZ3UmWAo8bCj8RN64BvoQUTdLtd567hXqQCYDVo";
    let body: Value = reqwest::blocking::get(format!("https://nodes.wavesnodes.com/transactions/info/{}", tx_id))
        .unwrap()
        .json()
        .unwrap();
    let transaction = from_json(body);
    let tx_kind = match transaction.data() {
        TransactionData::Transfer { .. } => "transfer"
    };
    println!("{}", tx_kind)
}