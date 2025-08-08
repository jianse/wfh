//! wegame world state replacements

use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use sea_orm::{ActiveValue::Set, EntityTrait, IntoActiveModel, PaginatorTrait};
use serde::Deserialize;

use crate::{
    AppState,
    api::{BizError, BizResponse, BizResult, Page, Pager},
    entities::{prelude::WgWorldStateReplacements, wg_world_state_replacements},
};

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
