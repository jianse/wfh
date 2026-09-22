//! deps api

use std::collections::HashMap;

use axum::Json;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::extract::Multipart;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::patch;
use axum::routing::post;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;

use crate::AppState;
use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::api::Page;
use crate::api::Pager;
use crate::api::RequireRoleLayer;
use crate::api::Role;
use crate::api::raw_world_state::parse::save_langs;
use crate::api::raw_world_state::parse::save_sortie_modifiers;

/// router for deps management
pub(crate) fn public_routes() -> Router<AppState> {
    Router::new().route("/lang", post(get_lang_items))
}

pub(crate) fn authed_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/reload", post(force_reload_deps))
        .route(
            "/sortie_modifiers",
            get(list_sortie_modifiers).post(add_new_sortie_modifier),
        )
        .route(
            "/sortie_modifiers/{id}",
            delete(remove_sortie_modifier)
                .layer(RequireRoleLayer::at_least(state.clone(), Role::Admin)),
        )
        .route("/lang", patch(update_lang_item))
        .route(
            "/lang/{unique_name}",
            delete(remove_lang_item).layer(RequireRoleLayer::at_least(state.clone(), Role::Admin)),
        )
        .route("/langs", get(list_langs).post(new_lang_item))
        .route("/langs/search", get(search_langs))
        .route(
            "/langs/upload",
            post(upload_langs)
                .route_layer(DefaultBodyLimit::max(50 * 1024 * 1024))
                .route_layer(RequireRoleLayer::at_least(state, Role::Admin)),
        )
}

#[derive(Debug, Serialize, Deserialize)]
struct LangItem {
    unique_name: String,
    value: String,
}

impl From<(&String, &String)> for LangItem {
    fn from(value: (&String, &String)) -> Self {
        Self {
            unique_name: value.0.clone(),
            value: value.1.clone(),
        }
    }
}

async fn list_langs(
    State(state): State<AppState>,
    Query(pager): Query<Pager>,
) -> BizResult<BizResponse<Page<LangItem>>> {
    let deps = state.deps.deps.read().await;
    let total = deps.languages.len() as u64;
    let langs = deps
        .languages
        .iter()
        .skip((pager.page * pager.page_size) as usize)
        .take(pager.page_size as usize)
        .map(Into::into)
        .collect();
    // Page::new(pager.page, pager.page_size, langs.len() as u64, langs)
    Ok(BizResponse::ok_page(
        pager.page,
        pager.page_size,
        total,
        langs,
    ))
}

/// Query parameters for language search.
#[derive(Debug, Deserialize)]
struct LangSearchQuery {
    q: String,
    #[serde(default)]
    page: u64,
    #[serde(default = "default_lang_page_size")]
    page_size: u64,
}

fn default_lang_page_size() -> u64 {
    20
}

/// Full-text search over language entries (jieba tokenized).
///
/// Returns the matched entries ordered by relevance. The result can be paged
/// with `page` (0 based) and `page_size`.
async fn search_langs(
    State(state): State<AppState>,
    Query(query): Query<LangSearchQuery>,
) -> BizResult<BizResponse<Page<LangItem>>> {
    let keyword = query.q.trim();
    if keyword.is_empty() || query.page_size == 0 {
        return Ok(BizResponse::ok_page(
            query.page,
            query.page_size,
            0,
            Vec::new(),
        ));
    }

    let deps = state.deps.deps.read().await;
    let index = deps
        .lang_index
        .as_ref()
        .ok_or_else(|| BizError::custom("language search index is not ready"))?;

    let offset = query.page.saturating_mul(query.page_size) as usize;
    let limit = query.page_size as usize;
    let (names, total) = index
        .search(keyword, offset, limit)
        .map_err(|e| BizError::custom(e.to_string()))?;

    let items = names
        .into_iter()
        .map(|unique_name| {
            let value = deps
                .languages
                .get(&unique_name)
                .cloned()
                .unwrap_or_default();
            LangItem { unique_name, value }
        })
        .collect();

    Ok(BizResponse::ok_page(
        query.page,
        query.page_size,
        total as u64,
        items,
    ))
}

async fn new_lang_item(
    State(state): State<AppState>,
    Json(req): Json<LangItem>,
) -> BizResult<BizResponse> {
    let mut deps = state.deps.deps.write().await;
    if deps.languages.contains_key(&req.unique_name) {
        return Err(BizError::custom(format!(
            "lang item {} already exists",
            req.unique_name
        )));
    }
    if let Some(index) = deps.lang_index.as_ref() {
        index
            .upsert(&req.unique_name, &req.value)
            .map_err(|e| BizError::custom(e.to_string()))?;
    }
    deps.languages.insert(req.unique_name, req.value);
    save_langs("data", &deps.languages)?;
    Ok(BizResponse::ok())
}

async fn remove_lang_item(
    State(state): State<AppState>,
    Path(unique_name): Path<String>,
) -> BizResult<BizResponse> {
    let mut deps = state.deps.deps.write().await;
    if let Some(index) = deps.lang_index.as_ref() {
        index
            .remove(&unique_name)
            .map_err(|e| BizError::custom(e.to_string()))?;
    }
    deps.languages.shift_remove(&unique_name);
    save_langs("data", &deps.languages)?;
    Ok(BizResponse::ok())
}

async fn update_lang_item(
    State(state): State<AppState>,
    // Path(unique_name): Path<String>,
    Json(req): Json<LangItem>,
) -> BizResult<BizResponse> {
    let unique_name = req.unique_name;
    let mut deps = state.deps.deps.write().await;
    if !deps.languages.contains_key(&unique_name) {
        return Err(BizError::custom(format!(
            "lang item {} not exists",
            unique_name
        )));
    }
    if let Some(index) = deps.lang_index.as_ref() {
        index
            .upsert(&unique_name, &req.value)
            .map_err(|e| BizError::custom(e.to_string()))?;
    }
    deps.languages.insert(unique_name, req.value);
    save_langs("data", &deps.languages)?;
    Ok(BizResponse::ok())
}

async fn upload_langs(
    State(state): State<AppState>,
    mut multipart: Multipart,
    // Multi(req): Json<HashMap<String, String>>,
) -> BizResult<BizResponse> {
    info!("upload langs");
    while let Some(field) = multipart.next_field().await? {
        let name = field
            .name()
            .ok_or(BizError::custom("formpart filed name is required!"))?;
        if name == "file" {
            // let mut buf = Vec::new();
            let bytes = field.bytes().await?;

            let langs = serde_json::from_slice::<IndexMap<String, String>>(&bytes)?;
            let mut deps = state.deps.deps.write().await;
            if let Some(index) = deps.lang_index.as_ref() {
                index
                    .rebuild(langs.iter().map(|(k, v)| (k.as_str(), v.as_str())))
                    .map_err(|e| BizError::custom(e.to_string()))?;
            }
            deps.languages = langs;
            save_langs("data", &deps.languages)?;
            return Ok(BizResponse::ok());
        }
    }

    Err(BizError::custom("required field \"file\" not exist!"))
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
