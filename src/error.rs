use axum::{Json, http::StatusCode, response::IntoResponse};
use config::ConfigError;
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;

use crate::settings::PwdHasherError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Db(#[from] DbErr),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// errors in handler
#[derive(Error, Debug)]
pub enum BizError {
    #[error(transparent)]
    Db(#[from] DbErr),

    #[error("no such user.")]
    NoSuchUser,

    #[error(transparent)]
    PwdHasher(#[from] PwdHasherError),

    #[error(transparent)]
    CreateToken(#[from] jsonwebtoken::errors::Error),

    #[error("username or password error.")]
    InvalidPassword,
}

impl IntoResponse for BizError {
    fn into_response(self) -> axum::response::Response {
        match self {
            BizError::Db(db_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg":db_err.to_string()})),
            )
                .into_response(),
            BizError::NoSuchUser => (
                StatusCode::NOT_FOUND,
                Json(json!({"msg":&self.to_string()})),
            )
                .into_response(),
            BizError::PwdHasher(pwd_hasher_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg":pwd_hasher_error.to_string()})),
            )
                .into_response(),
            BizError::CreateToken(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "msg":error.to_string()
                })),
            )
                .into_response(),
            BizError::InvalidPassword => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "msg": &self.to_string()
                })),
            )
                .into_response(),
        }
    }
}

pub type BizResult<T> = std::result::Result<T, BizError>;
