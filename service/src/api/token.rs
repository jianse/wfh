use std::net::SocketAddr;

use crate::entities::prelude::*;

use crate::AppState;
use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::entities;
use axum::Json;
use axum::extract::ConnectInfo;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use chrono::DateTime;
use chrono::Utc;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use rand::TryRng;
use rand::rngs::SysRng;
use sea_orm::ActiveValue;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tracing::info;

#[derive(Deserialize)]

pub(super) struct RefreshCredential {
    refresh_token: String,
    client_id: String,
}

#[derive(Debug, Serialize)]
pub(super) struct TokenResp {
    token: String,
    token_valid_until: DateTime<Utc>,
    refresh_token: String,
    refresh_valid_until: DateTime<Utc>,
}

/// generate new token and refresh token via refresh token
#[axum::debug_handler]
pub(super) async fn refresh_token(
    State(state): State<AppState>,
    ConnectInfo(client_ip): ConnectInfo<SocketAddr>,

    TypedHeader(ua): TypedHeader<UserAgent>,

    Json(cred): Json<RefreshCredential>,
) -> BizResult<BizResponse<TokenResp>> {
    let parsed_ua = state.ua_extractor.extract(ua.as_str());

    let record = RefreshToken::find()
        .filter(entities::refresh_token::Column::TokenHash.eq(cred.refresh_token))
        .one(&state.conn)
        .await?;

    match record {
        Some(r) => {
            // use different device_id must be rejected
            if r.device_id != cred.client_id {
                return Err(BizError::custom("client environment abnormal"));
            }

            let db_user = User::find_by_id(r.user_id)
                .one(&state.conn)
                .await?
                .ok_or_else(|| BizError::NoSuchUserId(-1))?;

            // TODO: do account security assessment
            let refresh_token = gen_refresh_token();

            let create_time = Utc::now();
            let valid_until = create_time + settings::get().auth.token.refresh_valid;

            RefreshToken::update(entities::refresh_token::ActiveModel {
                // id: ActiveValue::Set(Default::default()),
                // user_id: ActiveValue::Set(db_user.id),
                // device_id: ActiveValue::Set(user.client_id),
                token_hash: ActiveValue::Set(refresh_token.clone()),
                browser_family: ActiveValue::Set(parsed_ua.browser_family()),
                browser_version: ActiveValue::Set(parsed_ua.browser_version()),
                os_family: ActiveValue::Set(parsed_ua.os_family()),
                os_version: ActiveValue::Set(parsed_ua.os_version()),
                device_family: ActiveValue::Set(parsed_ua.device_model()),
                ip_address: ActiveValue::Set(client_ip.ip().to_string()),
                create_time: ActiveValue::Set(create_time),
                expires_time: ActiveValue::Set(valid_until),
                ..r.into()
            })
            .exec(&state.conn)
            .await?;

            let token_valid_until = Utc::now() + settings::get().auth.token.token_valid;
            let token = jsonwebtoken::encode(
                &Header::default(),
                &AccessTokenClaims {
                    iss: settings::get().auth.token.issuer.clone(),
                    sub: db_user.id,
                    exp: token_valid_until.timestamp() as usize,
                },
                &EncodingKey::from_secret(settings::get().auth.token.secret_key.as_ref()),
            )?;
            Ok(BizResponse::ok_with(TokenResp {
                token,
                token_valid_until,
                refresh_token,
                refresh_valid_until: valid_until,
            }))
        }
        None => Err(BizError::custom("not found")),
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct LoginCredential {
    username: String,
    password: String,
    client_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct AccessTokenClaims {
    /// issuer
    pub(super) iss: String,
    /// subject: this field is user_id for now
    pub(super) sub: i32,
    /// expiration time
    pub(super) exp: usize,
}

/// get token via username and password
/// this will generate a token and refresh token
/// the token is a short lived jwt token. Which means once it is issued, it has no way to expire it.
/// the refresh token is a long lived token bounded with client environment.
/// and it will also store in the database. so we can disable the refresh token through server side.
#[axum::debug_handler]
pub(super) async fn create_token(
    State(state): State<AppState>,
    ConnectInfo(client_ip): ConnectInfo<SocketAddr>,
    TypedHeader(ua): TypedHeader<UserAgent>,
    // jar: CookieJar,
    Json(user): Json<LoginCredential>,
) -> BizResult<BizResponse<TokenResp>> {
    // let cid = jar.get(CID_COOKIE_NAME).ok_or(BizError::custom("400"))?.value();

    let parsed_ua = state.ua_extractor.extract(ua.as_str());
    info!(?client_ip, ?parsed_ua);
    let db_user = User::find()
        .filter(entities::user::Column::Username.eq(&user.username))
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUsername(user.username))?;
    let valid = settings::get()
        .auth
        .pwd_hasher
        .verify(user.password, &db_user.password)?;

    // TODO: do account security assessment here!

    if valid {
        // sign jwt token
        let refresh_token = gen_refresh_token();

        let create_time = Utc::now();
        let valid_until = create_time + settings::get().auth.token.refresh_valid;

        // find or insert refresh token into database
        let rt = RefreshToken::find()
            .filter(entities::refresh_token::Column::UserId.eq(db_user.id))
            .filter(entities::refresh_token::Column::DeviceId.eq(&user.client_id))
            .one(&state.conn)
            .await?;
        match rt {
            Some(rt_db) => {
                // alter refresh token
                // this makes the old refresh token invalid
                RefreshToken::update(entities::refresh_token::ActiveModel {
                    // id: ActiveValue::Set(Default::default()),
                    // user_id: ActiveValue::Set(db_user.id),
                    // device_id: ActiveValue::Set(user.client_id),
                    token_hash: ActiveValue::Set(refresh_token.clone()),
                    browser_family: ActiveValue::Set(parsed_ua.browser_family()),
                    browser_version: ActiveValue::Set(parsed_ua.browser_version()),
                    os_family: ActiveValue::Set(parsed_ua.os_family()),
                    os_version: ActiveValue::Set(parsed_ua.os_version()),
                    device_family: ActiveValue::Set(parsed_ua.device_model()),
                    ip_address: ActiveValue::Set(client_ip.ip().to_string()),
                    create_time: ActiveValue::Set(create_time),
                    expires_time: ActiveValue::Set(valid_until),
                    ..rt_db.into()
                })
                .exec(&state.conn)
                .await?;
            }
            None => {
                // insert refresh token
                RefreshToken::insert(entities::refresh_token::ActiveModel {
                    id: ActiveValue::Set(uuid::Uuid::new_v4()),
                    user_id: ActiveValue::Set(db_user.id),
                    device_id: ActiveValue::Set(user.client_id),
                    token_hash: ActiveValue::Set(refresh_token.clone()),
                    browser_family: ActiveValue::Set(parsed_ua.browser_family()),
                    browser_version: ActiveValue::Set(parsed_ua.browser_version()),
                    os_family: ActiveValue::Set(parsed_ua.os_family()),
                    os_version: ActiveValue::Set(parsed_ua.os_version()),
                    device_family: ActiveValue::Set(parsed_ua.device_model()),
                    ip_address: ActiveValue::Set(client_ip.ip().to_string()),
                    create_time: ActiveValue::Set(create_time),
                    expires_time: ActiveValue::Set(valid_until),
                })
                .exec(&state.conn)
                .await?;
            }
        }

        let token_valid_until = Utc::now() + settings::get().auth.token.token_valid;

        let token = jsonwebtoken::encode(
            &Header::default(),
            &AccessTokenClaims {
                iss: settings::get().auth.token.issuer.clone(),
                sub: db_user.id,
                exp: token_valid_until.timestamp() as usize,
            },
            &EncodingKey::from_secret(settings::get().auth.token.secret_key.as_ref()),
        )?;
        Ok(BizResponse::ok_with(TokenResp {
            token,
            refresh_token,
            refresh_valid_until: valid_until,
            token_valid_until,
        }))
    } else {
        Err(BizError::InvalidPassword)
    }
}

fn gen_refresh_token() -> String {
    let mut rng = SysRng;
    let mut rand_var = [0u8; 32];
    rng.try_fill_bytes(&mut rand_var).unwrap();
    BASE64_URL_SAFE_NO_PAD.encode(rand_var)
}

#[axum::debug_handler]
pub(super) async fn nonce() -> BizResult<Response> {
    let mut rng = SysRng;
    let mut rand_var = [0u8; 32];
    rng.try_fill_bytes(&mut rand_var)
        .map_err(|e| BizError::custom(e.to_string()))?;
    let nonce = BASE64_URL_SAFE_NO_PAD.encode(rand_var);
    Ok(BizResponse::ok_with(json!({"nonce": nonce})).into_response())
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::DecodingKey;
    use jsonwebtoken::Validation;
    use time::UtcDateTime;

    use super::*;

    const SECRET_KEY: &str = "SECRET_KEY";

    #[test]
    fn test_jwt_issue() {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &AccessTokenClaims {
                iss: "issuer".to_owned(),
                sub: 1,
                exp: UtcDateTime::now().unix_timestamp() as usize + 3600,
            },
            &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
        );
        assert!(token.is_ok());
        println!("{}", token.unwrap());
    }

    #[test]
    fn test_decode_token() {
        // now we just want to make sure the token is valid
        // so escape the expiration time validation
        let mut validation = Validation::default();
        validation.validate_exp = false;

        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJpc3N1ZXIiLCJzdWIiOjEsImV4cCI6MTc1NTk0NTkzNH0.YbEf_18IHfRGp9hmeOAWuLvp3vrZD4jzcUAqxNk4XbM";
        let res = jsonwebtoken::decode::<AccessTokenClaims>(
            token,
            &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
            &validation,
        );

        assert!(res.is_ok());
        let claims = res.unwrap().claims;
        assert_eq!(claims.iss, "issuer");
        assert_eq!(claims.sub, 1);

        dbg!(claims);
    }
}
