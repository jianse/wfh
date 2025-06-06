use crate::AppState;
use axum::Json;
use axum::middleware::{Next, from_fn};
use axum::response::IntoResponse;
use axum::{
    Router,
    extract::Request,
    http::{StatusCode, header},
    response::Response,
    routing::post,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use once_cell::sync::Lazy;
use regex::RegexSet;
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tracing::info;

mod token;
mod translations;
mod user;

use settings::PwdHasherError;
use token::create_token;

pub(crate) fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/user", user::user_routes())
        .nest("/translations", translations::translations_routes())
        .layer(from_fn(auth_layer))
        .route("/token", post(create_token))
        .fallback(async || StatusCode::NOT_FOUND)
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: i32,
}

#[derive(Debug, Error)]
enum AuthError {
    #[error("unauthorized")]
    UnAuthorized,

    #[error(transparent)]
    HeaderToStr(#[from] header::ToStrError),

    #[error("decode jwt error")]
    JwtDecode(#[from] jsonwebtoken::errors::Error),
}

type AuthResult<T> = Result<T, AuthError>;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::UnAuthorized | AuthError::HeaderToStr(_) | AuthError::JwtDecode(_) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "msg": "unauthorized"
                })),
            )
                .into_response(),
        }
    }
}

async fn auth_layer(mut req: Request, next: Next) -> AuthResult<Response> {
    // white list uri bypass
    static WHITE_LIST: Lazy<RegexSet> = Lazy::new(|| RegexSet::new(["/translations/all"]).unwrap());
    if WHITE_LIST.is_match(req.uri().path()) {
        return Ok(next.run(req).await);
    }

    let token_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AuthError::UnAuthorized)?;

    let token = token_header.to_str()?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[settings::get().auth.jwt.issuer.to_owned()]);
    validation.set_required_spec_claims(&["exp", "iss"]);
    // validation
    let jwt = decode::<token::Claims>(
        token,
        &DecodingKey::from_secret(settings::get().auth.jwt.secret_key.as_ref()),
        &validation,
    )?;
    let id = jwt.claims.sub;
    req.extensions_mut().insert(CurrentUser { id });
    let response = next.run(req).await;
    Ok(response)
}

pub(crate) struct BizStatus {
    code: u64,
    message: &'static str,
}

macro_rules! biz_status {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl BizStatus {
        $(
            $(#[$docs])*
            pub const $konst: BizStatus = BizStatus{code: $num,message:$phrase};
        )+

        }
    }
}

biz_status! {
    /// just ok status
    (20000, OK, "ok");
    (50000, ERROR, "error");
}

#[derive(Serialize)]
pub(crate) struct BizResponse<DATA: Serialize = ()> {
    code: u64,
    message: &'static str,
    data: Option<DATA>,
}

impl BizResponse<()> {
    pub fn ok() -> Self {
        Self::new(BizStatus::OK, None)
    }
    pub fn new_empty(status: BizStatus) -> Self {
        Self {
            code: status.code,
            message: status.message,
            data: None,
        }
    }
}

impl<DATA: Serialize> BizResponse<DATA> {
    pub fn ok_with(data: DATA) -> Self {
        Self::new_with(BizStatus::OK, data)
    }

    pub fn new_with(status: BizStatus, data: DATA) -> Self {
        Self::new(status, Some(data))
    }

    pub fn new(status: BizStatus, data: Option<DATA>) -> Self {
        Self {
            code: status.code,
            message: status.message,
            data,
        }
    }
}

impl<DATA: Serialize> IntoResponse for BizResponse<DATA> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// errors in handler
#[derive(Error, Debug)]
pub enum BizError {
    /// some error occur during db query this is not recoverable
    /// 5xx may throw
    #[error(transparent)]
    Db(#[from] DbErr),

    #[error("no such user. username = {0}")]
    NoSuchUsername(String),

    #[error("no such user. id = {0}")]
    NoSuchUserId(i32),

    #[error(transparent)]
    PwdHasher(#[from] PwdHasherError),

    #[error(transparent)]
    CreateToken(#[from] jsonwebtoken::errors::Error),

    #[error("username or password error.")]
    InvalidPassword,
}

impl IntoResponse for BizError {
    fn into_response(self) -> Response {
        info!("encounter an error: {}", self.to_string());
        match self {
            BizError::Db(db_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg":db_err.to_string()})),
            )
                .into_response(),
            _ => BizResponse::new_empty(BizStatus::ERROR).into_response(),
        }
    }
}

pub type BizResult<T> = Result<T, BizError>;
