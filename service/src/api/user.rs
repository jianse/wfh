use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::EntityTrait;
use serde::Serialize;

use crate::{AppState, entities};

use crate::api::{BizError, BizResponse, BizResult, CurrentUser};
use entities::prelude::*;

pub(crate) fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/current", get(get_current_user))
        .route("/{id}", get(get_user_by_id))
}

#[derive(Serialize)]
struct UserResp {
    id: i64,
    username: String,
}

impl From<entities::user::Model> for UserResp {
    fn from(value: entities::user::Model) -> Self {
        Self {
            id: value.id.into(),
            username: value.username,
        }
    }
}

#[axum::debug_handler]
async fn get_user_by_id(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserResp>, StatusCode> {
    let u = User::find_by_id(id).one(&state.conn).await.unwrap();
    if let Some(u) = u {
        Ok(Json(u.into()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[axum::debug_handler]
async fn get_current_user(
    state: State<AppState>,
    Extension(cur_user): Extension<CurrentUser>,
) -> BizResult<BizResponse<UserResp>> {
    let u = User::find_by_id(cur_user.id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(cur_user.id))?;
    Ok(BizResponse::ok_with(u.into()))
}
