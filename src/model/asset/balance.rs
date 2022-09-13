use crate::error::{Error, Result};
use crate::model::{Address, AssetId, IssueTransactionInfo, TransactionInfoResponse};
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

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn balances(&self) -> Vec<AssetBalance> {
        self.balances.clone()
    }
}

impl TryFrom<&Value> for AssetsBalanceResponse {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let address = Address::from_string(&JsonDeserializer::safe_to_string_from_field(
            value, "address",
        )?)?;
        let balances = JsonDeserializer::safe_to_array_from_field(value, "balances")?
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

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::asset::balance::AssetsBalanceResponse;
    use crate::model::{ByteString, IssueTransactionInfo, TransactionDataInfo};
    use serde_json::Value;
    use std::fs;

    #[test]
    fn test_json_to_assets_balance_response() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/assets/assets_balance_rs.json")
            .expect("Unable to read file");
        let json: &Value = &serde_json::from_str(&data).expect("failed to convert");
        let assets_balance: AssetsBalanceResponse = json.try_into()?;
        assert_eq!(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            assets_balance.address().encoded()
        );

        let balances = assets_balance.balances();
        let asset_balance1 = &balances[0];
        assert_eq!(
            "85gPhjumNgwaMUpGfx9jEQqJMorbEjTQ4EUAHwfoYKjd",
            asset_balance1.asset_id().encoded()
        );
        assert_eq!(false, asset_balance1.reissuable());
        assert_eq!(None, asset_balance1.min_sponsored_asset_fee());
        assert_eq!(None, asset_balance1.sponsor_balance());
        assert_eq!(32, asset_balance1.quantity());
        assert_eq!(42, asset_balance1.balance());
        let issue_tx = asset_balance1
            .issue_transaction()
            .expect("must not be empty");

        assert_eq!(
            "85gPhjumNgwaMUpGfx9jEQqJMorbEjTQ4EUAHwfoYKjd",
            issue_tx.asset_id().encoded()
        );
        assert_eq!("test asset", issue_tx.name());
        assert_eq!(32, issue_tx.quantity());
        assert_eq!(false, issue_tx.is_reissuable());
        assert_eq!(3, issue_tx.decimals());
        assert_eq!("this is test asset", issue_tx.description());
        assert_eq!(None, issue_tx.script());

        let asset_balance2 = &assets_balance.balances()[1];
        assert_eq!(
            "GyH2wqKQcjHtz6KgkUNzUpDYYy1azqZdYHZ2awXHWqYx",
            asset_balance2.asset_id().encoded()
        );
        assert_eq!(false, asset_balance2.reissuable());
        assert_eq!(Some(1), asset_balance2.min_sponsored_asset_fee());
        assert_eq!(Some(199900003), asset_balance2.sponsor_balance());
        assert_eq!(2, asset_balance2.quantity());
        assert_eq!(None, asset_balance2.issue_transaction());
        assert_eq!(2, asset_balance2.balance());

        Ok(())
    }
}
