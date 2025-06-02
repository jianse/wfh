use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use log::info;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, TransactionTrait};
// use sea_orm_migration::schema::json;
use serde::Deserialize;
use serde_json::json;

use crate::{
    entities::{prelude::*, translation},
    error::BizResult,
};

use crate::AppState;

pub(crate) fn translations_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(add_or_update_translation).delete(delete_translation),
        )
        .route("/all", get(all_translations))
    // .route("/{*unique_name}", )
}

#[derive(Debug, Deserialize)]
struct TranslateItem {
    unique_name: String,
    translation: String,
}

async fn add_or_update_translation(
    State(state): State<AppState>,
    Json(trans): Json<TranslateItem>,
) -> BizResult<Response> {
    let txn = state.conn.begin().await?;

    let t_db = Translation::find_by_id(&trans.unique_name)
        .one(&txn)
        .await?;

    if let Some(t_db) = t_db {
        let mut am: translation::ActiveModel = t_db.into();
        am.translation = Set(trans.translation);
        am.update(&txn).await?;
    } else {
        let am = translation::ActiveModel {
            unique_name: Set(trans.unique_name),
            translation: Set(trans.translation),
        };
        am.insert(&txn).await?;
    }
    txn.commit().await?;
    Ok(StatusCode::OK.into_response())
}

async fn all_translations(State(state): State<AppState>) -> BizResult<Response> {
    let data = Translation::find().all(&state.conn).await?;
    Ok(Json(json!({"data":data})).into_response())
}

#[derive(Deserialize)]
struct DeleteQuery {
    unique_name: String,
}

async fn delete_translation(
    State(state): State<AppState>,
    // Path(unique_name): Path<String>,
    Query(DeleteQuery { unique_name }): Query<DeleteQuery>,
) -> BizResult<impl IntoResponse> {
    info!("deleting {}", unique_name);
    let _res = Translation::delete_by_id(unique_name)
        .exec(&state.conn)
        .await?;
    Ok(())
}
