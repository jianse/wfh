//! deps api

use std::collections::HashMap;

use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use serde::Deserialize;
use serde::Serialize;

use crate::AppState;
use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::api::raw_world_state::parse::save_sortie_modifiers;
use crate::api::require_admin_role;

/// router for deps management
pub(crate) fn deps_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/reload", post(force_reload_deps))
        .route(
            "/sortie_modifiers",
            get(list_sortie_modifiers).post(add_new_sortie_modifier),
        )
        .route("/sortie_modifiers/{id}", delete(remove_sortie_modifier))
        .layer(from_fn_with_state(state, require_admin_role))
        .route("/lang", post(get_lang_items))
}

#[derive(Debug, Deserialize)]
pub struct LangItemsRequest {
    names: Vec<String>,
}

/// get lang items
/// this endpoint is used to get the lang items from the deps
/// uses `POST` method instead of `GET`, because the lang items may be too large to return in a GET request
async fn get_lang_items(
    State(state): State<AppState>,
    Json(req): Json<LangItemsRequest>,
) -> BizResult<BizResponse<HashMap<String, Option<String>>>> {
    let deps = state.deps.deps.read().await;

    let mut res = HashMap::new();
    for name in req.names {
        let item = deps.languages.get(&name);
        res.insert(name, item.cloned());
    }

    Ok(BizResponse::ok_with(res))
}

/// list sortie modifiers
async fn list_sortie_modifiers(
    State(state): State<AppState>,
) -> BizResult<BizResponse<Vec<SortieModifier>>> {
    let deps = state.deps.deps.read().await;
    let modifiers = deps
        .sortie_modifiers
        .iter()
        .map(SortieModifier::from)
        .collect();
    Ok(BizResponse::ok_with(modifiers))
}

/// add new sortie modifier
async fn add_new_sortie_modifier(
    State(state): State<AppState>,
    Json(new_item): Json<SortieModifier>,
) -> BizResult<BizResponse> {
    // check if the modifier exists
    {
        let deps = state.deps.deps.read().await;

        // override the existing modifier is not allowed!
        if deps.sortie_modifiers.contains_key(&new_item.id) {
            return Err(BizError::custom(format!(
                "sortie modifier {} already exists",
                new_item.id
            )));
        }
    }

    // update the deps
    {
        let mut deps = state.deps.deps.write().await;
        deps.sortie_modifiers
            .insert(new_item.id.clone(), new_item.into());
    }

    // we should write sortie modifiers to file
    let deps = state.deps.deps.read().await;
    save_sortie_modifiers("data", &deps.sortie_modifiers)?;

    Ok(BizResponse::ok())
}

/// force reload the dependencies to parse the raw world state
async fn force_reload_deps(State(state): State<AppState>) -> BizResult<()> {
    state.deps.force_load_deps().await
}

/// remove sortie modifier
async fn remove_sortie_modifier(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> BizResult<BizResponse> {
    let mut deps = state.deps.deps.write().await;
    deps.sortie_modifiers.shift_remove(&id);

    // we should write sortie modifiers to file
    save_sortie_modifiers("data", &deps.sortie_modifiers)?;

    Ok(BizResponse::ok())
}

#[derive(Debug, Deserialize, Serialize)]
struct SortieModifier {
    id: String,
    name: String,
    description: String,
    #[serde(default = "Default::default")]
    vars: HashMap<String, String>,
}

impl From<(&String, &super::parse::SortieModifier)> for SortieModifier {
    fn from(value: (&String, &super::parse::SortieModifier)) -> Self {
        Self {
            id: value.0.clone(),
            name: value.1.name.clone(),
            description: value.1.description.clone(),
            vars: value.1.vars.clone(),
        }
    }
}

impl From<SortieModifier> for super::parse::SortieModifier {
    fn from(value: SortieModifier) -> Self {
        Self {
            name: value.name,
            description: value.description,
            vars: value.vars,
        }
    }
}
