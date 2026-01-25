//! wegame world state replacements

use axum::Json;
use axum::Router;
use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::PaginatorTrait;
use serde::Deserialize;

use crate::AppState;
use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::api::Page;
use crate::api::Pager;
use crate::entities::prelude::WgWorldStateReplacements;
use crate::entities::wg_world_state_replacements;

#[derive(Debug, Deserialize)]
struct Pk {
    path: String,
    origin: String,
}

#[axum::debug_handler]
async fn list_replacements(
    state: State<AppState>,
    pager: Query<Pager>,
) -> BizResult<BizResponse<Page<wg_world_state_replacements::Model>>> {
    let p = WgWorldStateReplacements::find().paginate(&state.conn, pager.page_size);

    let total = p.num_items().await?;

    let items = p.fetch_page(pager.page).await?;

    Ok(BizResponse::ok_page(
        pager.page,
        pager.page_size,
        total,
        items,
    ))
}

async fn edit_replacements(
    State(state): State<AppState>,
    Json(data): Json<wg_world_state_replacements::Model>,
) -> BizResult<()> {
    let rep = WgWorldStateReplacements::find_by_id((data.path.clone(), data.origin.clone()))
        .one(&state.conn)
        .await?;
    let rep = rep.ok_or(BizError::custom("no such replacement"))?;
    let mut am = rep.into_active_model();
    am.replacement = Set(data.replacement);
    am.remark = Set(data.remark);
    // rep.replacement = data.replacement;
    // rep.remark = data.remark;
    WgWorldStateReplacements::update(am)
        .exec(&state.conn)
        .await?;
    Ok(())
}

async fn delete_replacement(
    State(state): State<AppState>,
    Query(Pk { path, origin }): Query<Pk>,
) -> BizResult<()> {
    WgWorldStateReplacements::delete_by_id((path, origin))
        .exec(&state.conn)
        .await?;
    Ok(())
}

pub fn rep_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(list_replacements)
            .put(edit_replacements)
            .delete(delete_replacement),
    )
}
