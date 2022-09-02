use base64::DecodeError;
use blake2::digest::{InvalidBufferSize, InvalidOutputSize};
use ed25519_dalek::SignatureError;
use prost::EncodeError;
use std::result;
use url::ParseError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("node return error response (error: {error:?}, message: {message:?}))")]
    NodeError { error: u32, message: String },
    #[error("io error {0}")]
    IoError(#[from] reqwest::Error),
    #[error("failed to parse field: [{field:?}] from response {json:?}")]
    JsonParseError { field: String, json: String },
    #[error("base64 error {0}")]
    Base64Error(#[from] DecodeError),
    #[error("base58 error {0}")]
    Base58Error(#[from] bs58::decode::Error),
    #[error("blake error {0}")]
    BlakeError(InvalidSizeError),
    #[error("url parse error {0}")]
    UrlParseError(#[from] ParseError),
    #[error("wrong transaction type expected {expected_type:?} found {actual_type:?}")]
    WrongTransactionType { expected_type: u8, actual_type: u8 },
    #[error("failed to encode protobuf {0}")]
    ProtobufEncodeError(#[from] EncodeError),
    #[error("signature error {0}")]
    SignatureError(#[from] SignatureError),
    #[error("invalid bytes length expected {expected_len:?} actual {actual_len:?}")]
    InvalidBytesLength {
        expected_len: usize,
        actual_len: usize,
    },
    #[error("failed to convert Montgomery Point to Edwards Point")]
    MontgomeryPointConversionError,
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidSizeError {
    #[error("invalid output size {0}")]
    InvalidOutputSize(#[from] InvalidOutputSize),
    #[error("invalid buffer size {0}")]
    InvalidBufferSize(#[from] InvalidBufferSize),
}

pub struct WrongTransactionTypeError {}
