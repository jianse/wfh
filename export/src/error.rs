use reqwest::header::ToStrError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Utf8Decode(#[from] FromUtf8Error),

    #[error("parse json error: {0}")]
    JsonParse(String),

    #[error(transparent)]
    HeaderToStr(#[from] ToStrError),

    #[error(transparent)]
    ParseDateTimeError(#[from] time::error::Parse),

    #[error("no export catgory {0} found")]
    NoSuchExport(String),
}

pub(crate) type Result<T> = std::result::Result<T, ExportError>;
