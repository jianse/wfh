use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheErr {
    #[error("BinRWErr")]
    BinRWErr(#[from] binrw::error::Error),

    #[error("IOErr")]
    IOErr(#[from] std::io::Error),

    #[error("HTTP error")]
    Http(#[from] reqwest::Error),

    #[error("unexpected HTTP status: {0}")]
    HttpStatus(u16),

    #[error("invalid client index: {0}")]
    InvalidIndex(String),
}

pub type CacheResult<T> = Result<T, CacheErr>;
