use crate::error::{Error, Result};
use crate::model::{Address, AssetId, IssueTransactionInfo};
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AssetsBalanceResponse {
    address: Address,
    balances: Vec<AssetBalance>,
}

impl AssetsBalanceResponse {
    pub fn new(address: Address, balances: Vec<AssetBalance>) -> AssetsBalanceResponse {
        AssetsBalanceResponse { address, balances }
    }
}

impl TryFrom<Value> for AssetsBalanceResponse {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        let address = Address::from_string(&JsonDeserializer::safe_to_string_from_field(
            &value, "address",
        )?)?;
        let balances = JsonDeserializer::safe_to_array_from_field(&value, "balances")?
            .iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<AssetBalance>>>()?;
        Ok(AssetsBalanceResponse::new(address, balances))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AssetBalance {
    asset_id: AssetId,
    balance: u64,
    reissuable: bool,
    min_sponsored_asset_fee: Option<u64>,
    sponsor_balance: Option<u64>,
    quantity: u64,
    issue_transaction: Option<IssueTransactionInfo>,
}

impl AssetBalance {
    pub fn new(
        asset_id: AssetId,
        balance: u64,
        reissuable: bool,
        min_sponsored_asset_fee: Option<u64>,
        sponsor_balance: Option<u64>,
        quantity: u64,
        issue_transaction: Option<IssueTransactionInfo>,
    ) -> AssetBalance {
        AssetBalance {
            asset_id,
            balance,
            reissuable,
            min_sponsored_asset_fee,
            sponsor_balance,
            quantity,
            issue_transaction,
        }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn reissuable(&self) -> bool {
        self.reissuable
    }

    pub fn min_sponsored_asset_fee(&self) -> Option<u64> {
        self.min_sponsored_asset_fee
    }

    pub fn sponsor_balance(&self) -> Option<u64> {
        self.sponsor_balance
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn issue_transaction(&self) -> Option<IssueTransactionInfo> {
        self.issue_transaction.clone()
    }
}

impl TryFrom<&Value> for AssetBalance {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = AssetId::from_string(&JsonDeserializer::safe_to_string_from_field(
            value, "assetId",
        )?)?;
        let balance = JsonDeserializer::safe_to_int_from_field(value, "balance")? as u64;
        let reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "reissuable")?;
        let min_sponsored_asset_fee: Option<u64> = value["minSponsoredAssetFee"].as_u64();
        let sponsor_balance: Option<u64> = value["sponsorBalance"].as_u64();
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")? as u64;
        let issue_transaction = match value["issueTransaction"].as_object() {
            Some(obj) => {
                let issue_json: &Value = &obj.clone().into();
                Some(issue_json.try_into()?)
            }
            None => None,
        };
        Ok(AssetBalance::new(
            asset_id,
            balance,
            reissuable,
            min_sponsored_asset_fee,
            sponsor_balance,
            quantity,
            issue_transaction,
        ))
    }
}
