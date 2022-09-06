use crate::error::{Error, Result};
use crate::model::data_entry::DataEntry;
use crate::model::{
    ActionError, BurnAction, InvokeAction, IssueAction, LeaseInfo, ReissueAction, ScriptTransfer,
    SponsorFeeAction,
};
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StateChanges {
    data: Vec<DataEntry>,
    transfers: Vec<ScriptTransfer>,
    issues: Vec<IssueAction>,
    reissues: Vec<ReissueAction>,
    burns: Vec<BurnAction>,
    sponsor_fees: Vec<SponsorFeeAction>,
    leases: Vec<LeaseInfo>,
    lease_cancels: Vec<LeaseInfo>,
    invokes: Vec<InvokeAction>,
    error: Option<ActionError>,
}

#[allow(clippy::too_many_arguments)]
impl StateChanges {
    pub fn new(
        data: Vec<DataEntry>,
        transfers: Vec<ScriptTransfer>,
        issues: Vec<IssueAction>,
        reissues: Vec<ReissueAction>,
        burns: Vec<BurnAction>,
        sponsor_fees: Vec<SponsorFeeAction>,
        leases: Vec<LeaseInfo>,
        lease_cancels: Vec<LeaseInfo>,
        invokes: Vec<InvokeAction>,
        error: Option<ActionError>,
    ) -> StateChanges {
        StateChanges {
            data,
            transfers,
            issues,
            reissues,
            burns,
            sponsor_fees,
            leases,
            lease_cancels,
            invokes,
            error,
        }
    }

    pub fn data(&self) -> Vec<DataEntry> {
        self.data.clone()
    }

    pub fn transfers(&self) -> Vec<ScriptTransfer> {
        self.transfers.clone()
    }

    pub fn issues(&self) -> Vec<IssueAction> {
        self.issues.clone()
    }

    pub fn reissues(&self) -> Vec<ReissueAction> {
        self.reissues.clone()
    }

    pub fn burns(&self) -> Vec<BurnAction> {
        self.burns.clone()
    }

    pub fn sponsor_fees(&self) -> Vec<SponsorFeeAction> {
        self.sponsor_fees.clone()
    }

    pub fn leases(&self) -> Vec<LeaseInfo> {
        self.leases.clone()
    }

    pub fn lease_cancels(&self) -> Vec<LeaseInfo> {
        self.lease_cancels.clone()
    }

    pub fn invokes(&self) -> Vec<InvokeAction> {
        self.invokes.clone()
    }

    pub fn error(&self) -> Option<ActionError> {
        self.error.clone()
    }
}

impl TryFrom<&Value> for StateChanges {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let data: Vec<DataEntry> = JsonDeserializer::safe_to_array_from_field(value, "data")?
            .iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<DataEntry>>>()?;
        let transfers: Vec<ScriptTransfer> =
            JsonDeserializer::safe_to_array_from_field(value, "transfers")?
                .iter()
                .map(|item| item.try_into())
                .collect::<Result<Vec<ScriptTransfer>>>()?;
        let issues: Vec<IssueAction> = JsonDeserializer::safe_to_array_from_field(value, "issues")?
            .iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<IssueAction>>>()?;
        let reissues: Vec<ReissueAction> =
            JsonDeserializer::safe_to_array_from_field(value, "reissues")?
                .iter()
                .map(|item| item.try_into())
                .collect::<Result<Vec<ReissueAction>>>()?;
        let burns: Vec<BurnAction> = JsonDeserializer::safe_to_array_from_field(value, "burns")?
            .iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<BurnAction>>>()?;
        let sponsor_fees: Vec<SponsorFeeAction> =
            JsonDeserializer::safe_to_array_from_field(value, "sponsorFees")?
                .iter()
                .map(|item| item.try_into())
                .collect::<Result<Vec<SponsorFeeAction>>>()?;
        let leases: Vec<LeaseInfo> = JsonDeserializer::safe_to_array_from_field(value, "leases")?
            .iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<LeaseInfo>>>()?;
        let lease_cancels: Vec<LeaseInfo> =
            JsonDeserializer::safe_to_array_from_field(value, "leaseCancels")?
                .iter()
                .map(|item| item.try_into())
                .collect::<Result<Vec<LeaseInfo>>>()?;
        let invokes: Vec<InvokeAction> =
            JsonDeserializer::safe_to_array_from_field(value, "invokes")?
                .iter()
                .map(|item| item.try_into())
                .collect::<Result<Vec<InvokeAction>>>()?;
        let error = match value["error"]["code"].as_i64() {
            Some(code) => {
                let text = JsonDeserializer::safe_to_string_from_field(&value["error"], "text")?;
                Some(ActionError::new(code as u32, text))
            }
            None => None,
        };
        Ok(StateChanges {
            data,
            transfers,
            issues,
            reissues,
            burns,
            sponsor_fees,
            leases,
            lease_cancels,
            invokes,
            error,
        })
    }
}
