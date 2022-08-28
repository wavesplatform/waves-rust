use serde_json::Value;
use std::fmt;
use std::fmt::{Display, Formatter};

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

// #[derive(thiserror::Error, Debug)]
// pub enum NodeError {
//     #[error("{0}")]
//     ValidationError(String),
//     #[error(transparent)]
//     UnexpectedError(#[from] anyhow::Error),
// }

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    FieldNotFoundError { field_name: String, json: String },
    InvalidTypeError { json: String, json_type: String },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FieldNotFoundError { field_name, json } => {
                write!(
                    f,
                    "FieldNotFoundError\nfield {} not found in json {}",
                    field_name, json
                )
            }
            ParseError::InvalidTypeError { json, json_type } => {
                write!(
                    f,
                    "InvalidTypeError\nexpected type: {} actual type: {}",
                    json_type, json
                )
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub struct InvalidTypeError {
    json: String,
    json_type: String,
}

impl InvalidTypeError {
    pub fn new(json: &Value, json_type: String) -> InvalidTypeError {
        InvalidTypeError {
            json: json.to_string(),
            json_type,
        }
    }
}

impl Display for InvalidTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InvalidTypeError\nexpected type: {} actual type: {}",
            self.json_type, self.json
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

impl Display for FieldNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "FieldNotFoundError\nfield {} not found in json {}",
            self.field_name, self.json
        )
    }
}
