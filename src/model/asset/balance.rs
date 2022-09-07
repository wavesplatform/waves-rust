use crate::model::{AssetId, IssueTransaction, IssueTransactionInfo};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AssetsBalance {
    asset_id: AssetId,
    balance: u64,
    reissuable: bool,
    min_sponsored_asset_fee: u64,
    sponsor_balance: u64,
    quantity: u64,
    issue_transaction: IssueTransactionInfo,
}

impl AssetsBalance {
    pub fn new(
        asset_id: AssetId,
        balance: u64,
        reissuable: bool,
        min_sponsored_asset_fee: u64,
        sponsor_balance: u64,
        quantity: u64,
        issue_transaction: IssueTransactionInfo,
    ) -> AssetsBalance {
        AssetsBalance {
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

    pub fn min_sponsored_asset_fee(&self) -> u64 {
        self.min_sponsored_asset_fee
    }

    pub fn sponsor_balance(&self) -> u64 {
        self.sponsor_balance
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn issue_transaction(&self) -> IssueTransactionInfo {
        self.issue_transaction.clone()
    }
}
