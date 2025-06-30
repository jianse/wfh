use axum::{
    Json, Router,
    extract::{Query, State},
    middleware::from_fn_with_state,
    routing::{get, post},
};
use log::info;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, TransactionTrait};
use serde::Deserialize;

use crate::AppState;
use crate::api::{BizResponse, BizResult};
use crate::entities::translation::Model;
use crate::entities::{prelude::*, translation};

pub(crate) fn translations_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(add_or_update_translation).delete(delete_translation),
        )
        .route_layer(from_fn_with_state(state, super::require_admin_role))
        .route("/all", get(all_translations))
}

#[derive(Debug, Deserialize)]
struct TranslateItem {
    unique_name: String,
    content: String,
}

async fn add_or_update_translation(
    // Extension(cur_user):Extension<CurrentUser>,
    State(state): State<AppState>,
    Json(trans): Json<TranslateItem>,
) -> BizResult<BizResponse> {
    let txn = state.conn.begin().await?;

    let t_db = Translation::find_by_id(&trans.unique_name)
        .one(&txn)
        .await?;

    if let Some(t_db) = t_db {
        let mut am: translation::ActiveModel = t_db.into();
        am.content = Set(trans.content);
        am.update(&txn).await?;
    } else {
        let am = translation::ActiveModel {
            unique_name: Set(trans.unique_name),
            content: Set(trans.content),
        };
        am.insert(&txn).await?;
    }
    txn.commit().await?;
    Ok(BizResponse::ok())
}

async fn all_translations(State(state): State<AppState>) -> BizResult<BizResponse<Vec<Model>>> {
    let data = Translation::find().all(&state.conn).await?;
    Ok(BizResponse::ok_with(data))
}

#[derive(Deserialize)]
struct DeleteQuery {
    unique_name: String,
}

async fn delete_translation(
    State(state): State<AppState>,
    // Path(unique_name): Path<String>,
    Query(DeleteQuery { unique_name }): Query<DeleteQuery>,
) -> BizResult<BizResponse> {
    info!("deleting {unique_name}");
    let _res = Translation::delete_by_id(unique_name)
        .exec(&state.conn)
        .await?;
    Ok(BizResponse::ok())
}
