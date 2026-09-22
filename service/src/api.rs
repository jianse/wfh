use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use crate::AppState;
use crate::api::token::nonce;
use crate::common;
use crate::entities;
use axum::Json;
use axum::Router;
use axum::extract::ConnectInfo;
use axum::extract::Path;
use axum::extract::Request;
use axum::extract::State;
use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use futures_util::future::BoxFuture;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tower::Layer;
use tower::Service;
use tower::ServiceExt;
use tracing::debug;
use tracing::error;
use tracing::info;

pub(crate) mod raw_world_state;
mod token;
mod translations;
mod user;
pub mod world_state;

use settings::PwdHasherError;
use token::create_token;
use token::refresh_token;

use tracing::warn;
// use tracing_subscriber::Layer;

pub(crate) fn api_routes(state: AppState) -> Router<AppState> {
    // public endpoints: no authentication required
    let public = Router::new()
        .route("/nonce", get(nonce))
        .route("/token", post(create_token))
        .route("/refresh", post(refresh_token))
        .nest("/translations", translations::public_routes())
        .nest("/world_state", world_state::public_routes())
        .nest("/raw_world_state", raw_world_state::public_routes());

    // authed endpoints: everything behind `auth_layer`
    let authed = Router::new()
        .nest("/user", user::user_routes(state.clone()))
        .nest("/translations", translations::authed_routes(state.clone()))
        .nest("/world_state", world_state::authed_routes())
        .nest(
            "/raw_world_state",
            raw_world_state::authed_routes(state.clone()),
        )
        .layer(from_fn(auth_layer));

    Router::new()
        .nest("/v1/public", public)
        .nest("/v1/authed", authed)
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
                        "code": BizStatus::ERROR.code,
                        "msg": "unauthorized"
                    })),
                )
                    .into_response()
            }
        }
    }
}

async fn auth_layer(mut req: Request, next: Next) -> AuthResult<Response> {
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

    #[error(transparent)]
    Acme(#[from] acme::AcmeError),

    #[error(transparent)]
    Multipart(#[from] MultipartError),
}

impl BizError {
    pub fn custom<S: Display>(msg: S) -> Self {
        BizError::Custom(msg.to_string())
    }

    /// HTTP status code that best describes this error.
    pub fn status(&self) -> StatusCode {
        use BizError::*;
        match self {
            NoSuchUsername(_) | NoSuchUserId(_) => StatusCode::NOT_FOUND,
            InvalidPassword => StatusCode::UNAUTHORIZED,
            Permission => StatusCode::FORBIDDEN,
            SerdeJson(_) | SerdeErrorPath(_) | Custom(_) | Multipart(_) => StatusCode::BAD_REQUEST,
            CallPeer(_) | PeerResponse(..) => StatusCode::BAD_GATEWAY,
            Db(_)
            | PwdHasher(_)
            | CreateToken(_)
            | Compress(_)
            | IoError(_)
            | Export(_)
            | Acme(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Message safe to return to clients (internal errors are never leaked).
    pub fn public_message(&self) -> String {
        match self {
            BizError::Custom(msg) => msg.clone(),
            BizError::NoSuchUsername(name) => format!("no such user. username = {name}"),
            BizError::NoSuchUserId(id) => format!("no such user. id = {id}"),
            BizError::InvalidPassword => "username or password error.".to_owned(),
            BizError::Permission => "permission denied".to_owned(),
            BizError::PeerResponse(code, msg) => {
                format!("peer response error. code: {code}, message: {msg}")
            }
            BizError::SerdeJson(_) | BizError::SerdeErrorPath(_) | BizError::Multipart(_) => {
                self.to_string()
            }
            _ => "internal server error".to_owned(),
        }
    }
}

impl IntoResponse for BizError {
    fn into_response(self) -> Response {
        let status = self.status();
        if status.is_server_error() {
            error!("encounter an error: {self}");
        } else {
            debug!("encounter an error: {self}");
        }
        (
            status,
            Json(json!({
                "code": BizStatus::ERROR.code,
                "msg": self.public_message(),
            })),
        )
            .into_response()
    }
}

pub type BizResult<T> = Result<T, BizError>;

/// The roles a user can have, ordered from least to most privileged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    Normal,
    Maintainer,
    Admin,
}

impl Role {
    /// All roles from least to most privileged.
    pub const ALL: [Role; 3] = [Role::Normal, Role::Maintainer, Role::Admin];

    pub fn as_str(self) -> &'static str {
        match self {
            Role::Normal => "normal",
            Role::Maintainer => "maintainer",
            Role::Admin => "admin",
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Role::Normal),
            "maintainer" => Ok(Role::Maintainer),
            "admin" => Ok(Role::Admin),
            other => Err(format!("unknown role: {other}")),
        }
    }
}

/// A [`tower::Layer`] that enforces a role requirement.
///
/// Build one with [`RequireRoleLayer::at_least`] / [`RequireRoleLayer::any_of`]
/// and apply it to a router or method router with `.layer(...)`.
#[derive(Clone)]
pub struct RequireRoleLayer {
    state: AppState,
    allowed: Arc<[Role]>,
}

impl<S> Layer<S> for RequireRoleLayer {
    type Service = RequireRole<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequireRole {
            inner,
            state: self.state.clone(),
            allowed: self.allowed.clone(),
        }
    }
}

impl RequireRoleLayer {
    /// Allow only the given roles.
    pub fn any_of(state: AppState, roles: impl IntoIterator<Item = Role>) -> Self {
        Self {
            state,
            allowed: roles.into_iter().collect::<Vec<_>>().into(),
        }
    }

    /// Allow `role` and any more privileged role.
    pub fn at_least(state: AppState, role: Role) -> Self {
        Self::any_of(state, Role::ALL.into_iter().filter(|r| *r >= role))
    }
}

/// The service produced by [`RequireRoleLayer`].
#[derive(Clone)]
pub struct RequireRole<S> {
    inner: S,
    state: AppState,
    allowed: Arc<[Role]>,
}

impl<S> Service<Request> for RequireRole<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let inner = self.inner.clone();
        let state = self.state.clone();
        let allowed = self.allowed.clone();

        Box::pin(async move {
            let Some(cur_user) = req.extensions().get::<CurrentUser>().cloned() else {
                return Ok(AuthError::UnAuthorized.into_response());
            };

            let user = match entities::user::Entity::find_by_id(cur_user.id)
                .one(&state.conn)
                .await
            {
                Ok(Some(user)) => user,
                Ok(None) => return Ok(AuthError::UnAuthorized.into_response()),
                Err(err) => return Ok(BizError::Db(err).into_response()),
            };

            let role = user.role.parse().unwrap_or(Role::Normal);
            if !allowed.contains(&role) {
                return Ok(BizError::Permission.into_response());
            }

            inner.oneshot(req).await
        })
    }
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

pub(super) async fn acme_handler(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Response {
    info!("acme call for {}", token);
    if let Some(v) = state.get_acme_token(&token).await {
        info!("acme token found for {}", token);
        return (StatusCode::OK, v).into_response();
    }
    warn!("acme token not found for {}", token);
    StatusCode::NOT_FOUND.into_response()
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
