use config::ConfigError;
use sea_orm::DbErr;
use thiserror::Error;

use crate::api::BizError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Db(#[from] DbErr),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Biz(#[from] BizError),
}

pub type Result<T> = std::result::Result<T, Error>;
