use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::EntityTrait;
use serde::Serialize;

use crate::{AppState, entities};

use entities::prelude::*;

pub(crate) fn user_routes() -> Router<AppState> {
    Router::new().route("/{id}", get(get_user_by_id))
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
