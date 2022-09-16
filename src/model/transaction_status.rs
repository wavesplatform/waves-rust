use crate::error::{Error, Result};
use crate::model::{ApplicationStatus, Id};
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransactionStatus {
    id: Id,
    status: Status,
    app_status: ApplicationStatus,
    height: u32,
    confirmation: u32,
}

impl TransactionStatus {
    pub fn new(
        id: Id,
        status: Status,
        app_status: ApplicationStatus,
        height: u32,
        confirmation: u32,
    ) -> Self {
        Self {
            id,
            status,
            app_status,
            height,
            confirmation,
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn app_status(&self) -> ApplicationStatus {
        self.app_status
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn confirmation(&self) -> u32 {
        self.confirmation
    }
}

impl TryFrom<&Value> for TransactionStatus {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let status = JsonDeserializer::safe_to_string_from_field(value, "status")?;
        let tx_status = match status.as_str() {
            "not_found" => Status::NotFound,
            "unconfirmed" => Status::Unconfirmed,
            "confirmed" => Status::Confirmed,
            _ => Status::Unknown,
        };

        let id = JsonDeserializer::safe_to_string_from_field(value, "id")?;
        let application_status =
            match JsonDeserializer::safe_to_string_from_field(value, "applicationStatus")?.as_str()
            {
                "succeeded" => ApplicationStatus::Succeed,
                "script_execution_failed" => ApplicationStatus::ScriptExecutionFailed,
                &_ => ApplicationStatus::Unknown,
            };

        let height = JsonDeserializer::safe_to_int_from_field(value, "height")?;
        let confirmations = JsonDeserializer::safe_to_int_from_field(value, "confirmations")?;

        Ok(TransactionStatus {
            id: Id::from_string(&id)?,
            status: tx_status,
            app_status: application_status,
            height: height as u32,
            confirmation: confirmations as u32,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Status {
    NotFound,
    Unconfirmed,
    Confirmed,
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{
        Address, ApplicationStatus, ByteString, GenesisTransaction, GenesisTransactionInfo,
        SignedTransaction, Status, TransactionInfoResponse, TransactionStatus, Validation,
    };
    use crate::waves_proto::GenesisTransactionData;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_transaction_status() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/transaction_status_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let transaction_status: TransactionStatus = json.borrow().try_into()?;

        assert_eq!(transaction_status.status(), Status::Confirmed);
        assert_eq!(transaction_status.height(), 2217333);
        assert_eq!(transaction_status.confirmation(), 14051);
        assert_eq!(transaction_status.app_status(), ApplicationStatus::Succeed);
        assert_eq!(
            transaction_status.id().encoded(),
            "4XFVLLMBjBMPwGivgyLhw374kViANoToLAYUdEXWLsBJ"
        );
        Ok(())
    }
}
