use std::num::ParseIntError;

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
use serde_json::json;
use thiserror::Error;

use crate::{AppState, settings};

mod token;
mod translations;
mod user;

use token::user_login;

pub(crate) fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/user", user::user_routes())
        .nest("/translations", translations::translations_routes())
        .layer(from_fn(auth_layer))
        .route("/token", post(user_login))
        .fallback(async || StatusCode::NOT_FOUND)
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: u64,
}

#[derive(Debug, Error)]
enum AuthError {
    #[error("unauthorized")]
    UnAuthorized,

    #[error(transparent)]
    HeaderToStr(#[from] axum::http::header::ToStrError),

    #[error("decode jwt error")]
    JwtDecode(#[from] jsonwebtoken::errors::Error),

    #[error("parse int error")]
    ParseInt(#[from] ParseIntError),
}

type AuthResult<T> = Result<T, AuthError>;

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::UnAuthorized
            | AuthError::HeaderToStr(_)
            | AuthError::JwtDecode(_)
            | AuthError::ParseInt(_) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "msg": "unauthorized"
                })),
            )
                .into_response(),
        }
    }
}

// static WHITE_LIST:RegexSet = RegexSet::new(&[""]).unwrap();

async fn auth_layer(mut req: Request, next: Next) -> AuthResult<Response> {
    // white list uri bypass
    static WHITE_LIST: Lazy<RegexSet> =
        Lazy::new(|| RegexSet::new(&["/translations/all"]).unwrap());
    if WHITE_LIST.is_match(req.uri().path()) {
        return Ok(next.run(req).await);
    }

    let token_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AuthError::UnAuthorized)?;

    let token = token_header.to_str()?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp"]);
    // validation
    let jwt = decode::<token::Claims>(
        token,
        &DecodingKey::from_secret(settings::get().auth.jwt.secret_key.as_ref()),
        &validation,
    )?;
    let id = jwt.claims.sub.parse::<u64>()?;
    req.extensions_mut().insert(CurrentUser { id });
    let response = next.run(req).await;
    Ok(response)
}
