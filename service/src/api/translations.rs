use axum::Json;
use axum::Router;
use axum::extract::Query;
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::routing::post;
use log::info;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::TransactionTrait;
use serde::Deserialize;

use crate::AppState;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::entities::prelude::*;
use crate::entities::translation::Model;

pub(crate) fn translations_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(add_or_update_translation).delete(delete_translation),
        )
        .route_layer(from_fn_with_state(state, super::require_admin_role))
        .route("/all", get(all_translations))
}

async fn add_or_update_translation(
    // Extension(cur_user):Extension<CurrentUser>,
    State(state): State<AppState>,
    Json(trans): Json<Model>,
) -> BizResult<BizResponse> {
    let txn = state.conn.begin().await?;

    let t_db = Translation::find_by_id((trans.unique_name.to_owned(), trans.prop.to_owned()))
        .one(&txn)
        .await?;

    if let Some(t_db) = t_db {
        let mut am = t_db.into_active_model();
        am.content = Set(trans.content);
        am.update(&txn).await?;
    } else {
        let am = trans.into_active_model();
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
    prop: String,
}

async fn delete_translation(
    State(state): State<AppState>,
    // Path(unique_name): Path<String>,
    Query(DeleteQuery { unique_name, prop }): Query<DeleteQuery>,
) -> BizResult<BizResponse> {
    info!("deleting {unique_name}");
    let _res = Translation::delete_by_id((unique_name, prop))
        .exec(&state.conn)
        .await?;
    Ok(BizResponse::ok())
}
