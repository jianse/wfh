use crate::entities::prelude::*;

use crate::api::{BizError, BizResponse, BizResult};
use crate::{AppState, entities};
use axum::response::Response;
use axum::{Json, extract::State, response::IntoResponse};
use jsonwebtoken::{EncodingKey, Header};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
                exp: (chrono::Local::now().timestamp() as u64
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
    use jsonwebtoken::{DecodingKey, Validation};

    use super::*;

    const SECRET_KEY: &str = "SECRET_KEY";

    #[test]
    fn test_jwt_issue() {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &Claims {
                iss: "issuer".to_owned(),
                sub: 1,
                exp: chrono::Utc::now().timestamp() as usize + 3600,
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
