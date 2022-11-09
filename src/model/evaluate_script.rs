use crate::error::Error;
use serde_json::{Map, Value};

use crate::model::Address;
use crate::util::JsonDeserializer;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EvaluateScriptResponse {
    pub result: Map<String, Value>,
    pub complexity: u64,
    pub expr: String,
    pub address: Address,
}

impl TryFrom<&Value> for EvaluateScriptResponse {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(EvaluateScriptResponse {
            result: JsonDeserializer::safe_to_map_from_field(value, "result")?,
            complexity: JsonDeserializer::safe_to_int_from_field(value, "complexity")? as u64,
            expr: JsonDeserializer::safe_to_string_from_field(value, "expr")?,
            address: Address::from_string(
                JsonDeserializer::safe_to_string_from_field(value, "address")?.as_str(),
            )?,
        })
    }
}
