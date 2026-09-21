use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheErr {
    #[error("BinRWErr")]
    BinRWErr(#[from] binrw::error::Error),

    #[error("IOErr")]
    IOErr(#[from] std::io::Error),
}

pub type CacheResult<T> = Result<T, CacheErr>;
