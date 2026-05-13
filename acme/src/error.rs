use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AcmeError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("ACME protocol error [{status}]: {detail}")]
    Acme { status: u16, detail: String },

    #[error("Account does not exist")]
    AccountNotExist,

    #[error("Order is not valid: {0}")]
    OrderNotValid(String),

    #[error("Certificate error: {0}")]
    Certificate(String),

    #[error("Missing header: {0}")]
    MissingHeader(String),

    #[error("Max retry count reached")]
    MaxRetryReached,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Header error: {0}")]
    Header(#[from] InvalidHeaderValue),
}

pub type AcmeResult<T> = std::result::Result<T, AcmeError>;

impl From<std::str::Utf8Error> for AcmeError {
    fn from(e: std::str::Utf8Error) -> Self {
        AcmeError::Parse(e.to_string())
    }
}

impl From<reqwest::header::ToStrError> for AcmeError {
    fn from(e: reqwest::header::ToStrError) -> Self {
        AcmeError::Parse(e.to_string())
    }
}

impl From<std::time::SystemTimeError> for AcmeError {
    fn from(e: std::time::SystemTimeError) -> Self {
        AcmeError::Parse(e.to_string())
    }
}

impl From<chrono::OutOfRangeError> for AcmeError {
    fn from(e: chrono::OutOfRangeError) -> Self {
        AcmeError::Parse(e.to_string())
    }
}

impl From<std::str::ParseBoolError> for AcmeError {
    fn from(e: std::str::ParseBoolError) -> Self {
        AcmeError::Parse(e.to_string())
    }
}

impl From<x509_cert::der::Error> for AcmeError {
    fn from(e: x509_cert::der::Error) -> Self {
        AcmeError::Certificate(e.to_string())
    }
}

impl From<x509_cert::builder::Error> for AcmeError {
    fn from(e: x509_cert::builder::Error) -> Self {
        AcmeError::Certificate(e.to_string())
    }
}

impl From<p256::pkcs8::Error> for AcmeError {
    fn from(e: p256::pkcs8::Error) -> Self {
        AcmeError::Certificate(e.to_string())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct AcmeErrorResponse {
    pub status: u32,
    #[serde(rename = "type")]
    pub typ: String,
    pub detail: String,
}

impl AcmeError {
    pub fn from_response(status: reqwest::StatusCode, body: &str) -> Option<Self> {
        if let Ok(err) = serde_json::from_str::<AcmeErrorResponse>(body) {
            return Some(match err.typ.as_str() {
                "urn:ietf:params:acme:error:accountDoesNotExist" => Self::AccountNotExist,
                "urn:ietf:params:acme:error:orderNotValid" => Self::OrderNotValid(err.detail),
                _ => Self::Acme {
                    status: status.as_u16(),
                    detail: format!("{}: {}", err.typ, err.detail),
                },
            });
        }
        None
    }
}
