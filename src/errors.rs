// #[derive(thiserror::Error, Debug)]
// pub enum NodeError {
//     #[error("{0}")]
//     ValidationError(String),
//     #[error(transparent)]
//     UnexpectedError(#[from] anyhow::Error),
// }

use serde_json::Value;
use std::fmt;

#[derive(thiserror::Error, Debug)]
pub struct NodeError {
    error: u32,
    message: String,
}

impl NodeError {
    pub fn new(error: u32, message: String) -> NodeError {
        NodeError { error, message }
    }
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NodeError\nerror: {}\nmessage: {}",
            self.error, self.message
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub struct FieldNotFoundError {
    field_name: String,
    json: String,
}

impl FieldNotFoundError {
    pub fn new(json: &Value, field_name: String) -> FieldNotFoundError {
        FieldNotFoundError {
            json: json.to_string(),
            field_name,
        }
    }
}

impl fmt::Display for FieldNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldNotFoundError\nfield {} not found in json {}",
            self.field_name, self.json
        )
    }
}
