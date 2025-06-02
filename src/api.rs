use axum::middleware::{Next, from_fn};
use axum::{
    Router,
    extract::Request,
    http::{StatusCode, header},
    response::Response,
    routing::post,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

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

async fn auth_layer(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let token_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = token_header
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp"]);
    // validation
    let jwt = decode::<token::Claims>(
        token,
        &DecodingKey::from_secret(settings::get().auth.jwt_secret_key.as_ref()),
        &validation,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let id = jwt
        .claims
        .sub
        .parse::<u64>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert(CurrentUser { id });
    let response = next.run(req).await;
    Ok(response)
}
