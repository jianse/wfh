use crate::entities::prelude::*;

use crate::AppState;
use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::entities;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use time::UtcDateTime;

#[derive(Debug, Deserialize)]
pub(super) struct LoginCredential {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Claims {
    /// issuer
    pub(super) iss: String,
    /// subject: this field is user_id for now
    pub(super) sub: i32,
    /// expiration time
    pub(super) exp: usize,
}

#[axum::debug_handler]
pub(super) async fn create_token(
    State(state): State<AppState>,
    Json(user): Json<LoginCredential>,
) -> BizResult<Response> {
    let db_user = User::find()
        .filter(entities::user::Column::Username.eq(&user.username))
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUsername(user.username))?;
    let valid = settings::get()
        .auth
        .pwd_hasher
        .verify(user.password, &db_user.password)?;

    if valid {
        // sign jwt token
        let token = jsonwebtoken::encode(
            &Header::default(),
            &Claims {
                iss: settings::get().auth.jwt.issuer.clone(),
                sub: db_user.id,
                exp: (UtcDateTime::now().unix_timestamp() as u64
                    + settings::get().auth.jwt.valid_time_secs) as usize,
            },
            &EncodingKey::from_secret(settings::get().auth.jwt.secret_key.as_ref()),
        )?;
        Ok(BizResponse::ok_with(json!({"token": token})).into_response())
    } else {
        Err(BizError::InvalidPassword)
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::DecodingKey;
    use jsonwebtoken::Validation;

    use super::*;

    const SECRET_KEY: &str = "SECRET_KEY";

    #[test]
    fn test_jwt_issue() {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &Claims {
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
        let res = jsonwebtoken::decode::<Claims>(
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
