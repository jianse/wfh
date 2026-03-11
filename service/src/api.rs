use std::fmt::Display;
use std::net::SocketAddr;

use crate::AppState;
use crate::api::token::nonce;
use crate::common;
use crate::entities;
use axum::Extension;
use axum::Json;
use axum::Router;
use axum::extract::ConnectInfo;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use once_cell::sync::Lazy;
use regex::RegexSet;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tracing::debug;
use tracing::info;

mod raw_world_state;
mod token;
mod translations;
mod user;
pub mod world_state;

use settings::PwdHasherError;
use token::create_token;
use token::refresh_token;

pub(crate) use raw_world_state::parse::load_deps;
use tracing::warn;

// panic on startup should be fine
static WHITE_LIST: Lazy<RegexSet> = Lazy::new(|| {
    RegexSet::new([
        "/translations/all",
        "/world_state/current",
        "/raw_world_state/current",
        "/raw_world_state/st_pvt",
    ])
    .unwrap()
});

pub(crate) fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/user", user::user_routes())
        .nest(
            "/translations",
            translations::translations_routes(state.clone()),
        )
        .nest("/world_state", world_state::world_state_routes())
        .nest(
            "/raw_world_state",
            raw_world_state::raw_world_state_routes(state),
        )
        .layer(from_fn(auth_layer))
        .route("/nonce", get(nonce))
        .route("/token", post(create_token))
        .route("/refresh", post(refresh_token))
        .fallback(api_not_found)
}

async fn api_not_found(ConnectInfo(client_ip): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    warn!(?client_ip, "client trying to access a not existed api.");
    StatusCode::NOT_FOUND
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

    #[error("decode jwt error, {0}")]
    JwtDecode(#[from] jsonwebtoken::errors::Error),
}

type AuthResult<T> = Result<T, AuthError>;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::UnAuthorized | AuthError::HeaderToStr(_) | AuthError::JwtDecode(_) => {
                debug!("auth error {self}");
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "msg": "unauthorized"
                    })),
                )
                    .into_response()
            }
        }
    }
}

async fn auth_layer(mut req: Request, next: Next) -> AuthResult<Response> {
    // white list uri bypass
    if WHITE_LIST.is_match(req.uri().path()) {
        return Ok(next.run(req).await);
    }

    let token_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AuthError::UnAuthorized)?;

    let token = token_header.to_str()?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[settings::get().auth.token.issuer.to_owned()]);
    validation.set_required_spec_claims(&["exp", "iss"]);
    // validation
    let jwt = decode::<token::AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(settings::get().auth.token.secret_key.as_ref()),
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

impl<I: Serialize> BizResponse<Page<I>> {
    pub fn ok_page(page: u64, page_size: u64, total: u64, items: Vec<I>) -> Self {
        Self::new(
            BizStatus::OK,
            Some(Page {
                page,
                page_size,
                total,
                items,
            }),
        )
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

    #[error("permission deny")]
    Permission,

    #[error("call peer error. {0}")]
    CallPeer(#[from] reqwest::Error),

    #[error("peer response error. code: {0}, message: {1}")]
    PeerResponse(String, String),

    #[error("serde json error. {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("serde json error. {0}")]
    SerdeErrorPath(#[from] serde_path_to_error::Error<serde_json::Error>),

    #[error(transparent)]
    Compress(#[from] common::CompressError),

    #[error("other error. {0}")]
    Custom(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    Export(#[from] export::error::ExportError),
}

impl BizError {
    pub fn custom<S: Display>(msg: S) -> Self {
        BizError::Custom(msg.to_string())
    }
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

async fn require_admin_role(
    Extension(cur_user): Extension<CurrentUser>,
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> BizResult<Response> {
    debug!("role layer");
    let u = entities::user::Entity::find_by_id(cur_user.id)
        .one(&state.conn)
        .await?;
    let user = u.ok_or(BizError::NoSuchUserId(cur_user.id))?;
    if user.role != "admin" {
        return Err(BizError::Permission);
    }
    Ok(next.run(req).await)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pager {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct Page<T: Serialize> {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auth() {
        fn assert_send_sync<T: Send + Sync + 'static>() {}

        assert_send_sync::<BizError>();
    }
}
