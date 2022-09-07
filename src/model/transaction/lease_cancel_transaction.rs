use crate::error::{Error, Result};
use crate::model::{Id, LeaseInfo};
use crate::util::JsonDeserializer;
use crate::waves_proto::LeaseCancelTransactionData;
use serde_json::{Map, Value};
use std::borrow::Borrow;

const TYPE: u8 = 9;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LeaseCancelTransactionInfo {
    lease_id: Id,
    lease_info: LeaseInfo,
}

impl LeaseCancelTransactionInfo {
    pub fn new(lease_id: Id, lease_info: LeaseInfo) -> Self {
        Self {
            lease_id,
            lease_info,
        }
    }

    pub fn lease_id(&self) -> Id {
        self.lease_id.clone()
    }

    pub fn lease_info(&self) -> LeaseInfo {
        self.lease_info.clone()
    }
}

impl TryFrom<&Value> for LeaseCancelTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let lease_id = JsonDeserializer::safe_to_string_from_field(value, "leaseId")?;

        let lease_info: LeaseInfo = value["lease"].borrow().try_into()?;
        Ok(LeaseCancelTransactionInfo {
            lease_id: Id::from_string(&lease_id)?,
            lease_info,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LeaseCancelTransaction {
    lease_id: Id,
}

impl LeaseCancelTransaction {
    pub fn new(lease_id: Id) -> Self {
        Self { lease_id }
    }

    pub fn lease_id(&self) -> Id {
        self.lease_id.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for LeaseCancelTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let lease_id = JsonDeserializer::safe_to_string_from_field(value, "leaseId")?;

        Ok(LeaseCancelTransaction {
            lease_id: Id::from_string(&lease_id)?,
        })
    }
}

impl TryFrom<&LeaseCancelTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &LeaseCancelTransaction) -> Result<Self> {
        let mut lease_cancel_tx_json = Map::new();
        lease_cancel_tx_json.insert("leaseId".to_owned(), value.lease_id.encoded().into());
        Ok(lease_cancel_tx_json)
    }
}

impl TryFrom<&LeaseCancelTransaction> for LeaseCancelTransactionData {
    type Error = Error;

    fn try_from(value: &LeaseCancelTransaction) -> Result<Self> {
        Ok(LeaseCancelTransactionData {
            lease_id: value.lease_id.bytes(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{LeaseCancelTransactionInfo, LeaseStatus, LeaseTransactionInfo};
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_lease_cancel_transaction() {
        let data = fs::read_to_string("./tests/resources/lease_cancel_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let lease_cancel_from_json: LeaseCancelTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(
            "5EWudZk4xXaqRezrh26zqjbNeAzvEzDATjs4paKdyhGy",
            lease_cancel_from_json.lease_id().encoded()
        );

        let lease_info_from_json = lease_cancel_from_json.lease_info();
        assert_eq!(
            "5EWudZk4xXaqRezrh26zqjbNeAzvEzDATjs4paKdyhGy",
            lease_info_from_json.id().encoded()
        );
        assert_eq!(
            "5EWudZk4xXaqRezrh26zqjbNeAzvEzDATjs4paKdyhGy",
            lease_info_from_json.origin_transaction_id().encoded()
        );
        assert_eq!(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            lease_info_from_json.sender().encoded()
        );
        assert_eq!(
            "3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK",
            lease_info_from_json.recipient().encoded()
        );
        assert_eq!(100, lease_info_from_json.amount());
        assert_eq!(2218886, lease_info_from_json.height());
        assert_eq!(LeaseStatus::Canceled, lease_info_from_json.status());
        assert_eq!(2218925, lease_info_from_json.cancel_height());
        assert_eq!(
            "FoPVrSqzK74bwt8hgCDsEb48HJv7g2nvjeCW5wBoWpXb",
            lease_info_from_json.cancel_transaction_id().encoded()
        );
    }
}
