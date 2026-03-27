use axum::Router;
use axum::routing::get;
use axum::routing::post;
use chrono::DateTime;
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::prelude::DateTimeUtc;

use crate::AppState;
use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::api::load_deps;
use crate::api::raw_world_state::deps::deps_routes;
use crate::api::raw_world_state::parse::fill_raw_world_state;
use crate::entities;
use crate::entities::prelude::RawWorldState as RawWorldStateEntity;
use axum::extract::State;
use model::RawWorldState;
pub use model::*;
use std::collections::BTreeSet;
use std::time::Instant;
use tracing::info;

mod deps;
mod helper;
mod model;
pub(super) mod parse;

#[derive(Debug)]
pub struct Res {
    pub raw: RawWorldState,
    pub ignored: BTreeSet<String>,
}

static WG_URL: &str = "https://content-zhpt.wf.qq.com/dynamic/worldState.php";
static ST_URL: &str = "https://content.warframe.com/dynamic/worldState.php";

async fn fetch_st_prime_vault_trader(
    http_client: &reqwest::Client,
) -> BizResult<Vec<PrimeVaultTrader>> {
    let resp = http_client.get(ST_URL).send().await?;

    let body = resp.text().await?;

    let mut de = serde_json::Deserializer::from_str(body.as_str());

    let mut ignored_set = BTreeSet::new();
    let res: Result<RawWorldState, _> = serde_ignored::deserialize(&mut de, |path| {
        ignored_set.insert(path.to_string());
    });

    let raw = match res {
        Ok(raw) => raw,
        Err(_) => {
            let mut de = serde_json::Deserializer::from_str(body.as_str());
            serde_path_to_error::deserialize(&mut de)?
        }
    };

    Ok(raw.prime_vault_traders)
}

/// get prime vault traders from the global real game server
async fn get_st_prime_vault_trader(
    State(state): State<AppState>,
) -> BizResult<BizResponse<Vec<PrimeVaultTrader>>> {
    let res = fetch_st_prime_vault_trader(&state.http_client).await?;
    Ok(BizResponse::ok_with(res))
}

async fn fetch_raw_world_state(http_client: &reqwest::Client, url: &str) -> BizResult<Res> {
    let time = Instant::now();

    let resp = http_client.get(url).send().await?;
    info!("fetch_raw_world_state time: {:?}.", time.elapsed());

    let body = resp.text().await?;
    std::fs::write("cache/raw_world_state.json", &body)?;

    let time = Instant::now();
    let mut de = serde_json::Deserializer::from_str(body.as_str());

    let mut ignored_set = BTreeSet::new();
    let res: Result<RawWorldState, _> = serde_ignored::deserialize(&mut de, |path| {
        ignored_set.insert(path.to_string());
    });

    let raw = match res {
        Ok(raw) => raw,
        Err(_) => {
            let mut de = serde_json::Deserializer::from_str(body.as_str());
            serde_path_to_error::deserialize(&mut de)?
        }
    };
    info!("parse_raw_world_state time: {:?}", time.elapsed());

    Ok(Res {
        raw,
        ignored: ignored_set,
    })
}

pub fn raw_world_state_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/raw", get(get_raw_world_state))
        .route("/parsed", get(get_parsed_world_state))
        .route("/reload", post(force_reload_world_state))
        .route("/st_pvt", get(get_st_prime_vault_trader))
        .nest("/deps", deps_routes(state))
}

/// force reload world state
#[axum::debug_handler]
async fn force_reload_world_state(
    State(state): State<AppState>,
) -> BizResult<BizResponse<RawWorldState>> {
    let time = Instant::now();
    let sync_time = Utc::now();
    match fetch_raw_world_state(&state.http_client, WG_URL).await {
        Ok(Res { mut raw, ignored }) => {
            load_deps("data", "cache", &state.http_client, false).await?;
            fill_raw_world_state(&mut raw).await?;
            let content = serde_json::to_string(&raw)?;
            std::fs::write("cache/current_world_state.json", content)?;
            let nearest_expiry = calc_nearest_expiry(&raw);
            let time_spent_ms = time.elapsed().as_millis() as i32;
            let record = entities::raw_world_state::ActiveModel {
                sync_time: Set(sync_time),
                time_spent_ms: Set(time_spent_ms),
                error_msg: Set(None),
                is_success: Set(true),
                // text: Set(Some(serde_json::to_string(&raw)?)),
                warnings: if ignored.is_empty() {
                    NotSet
                } else {
                    dbg!(&ignored);
                    Set(Some(serde_json::to_string(&ignored)?))
                },
                nearest_expiry: Set(nearest_expiry),
                ..Default::default()
            };

            record.insert(&state.conn).await?;
            Ok(BizResponse::ok_with(raw))
        }
        Err(e) => {
            let time_spent_ms = time.elapsed().as_millis();
            let record = entities::raw_world_state::ActiveModel {
                sync_time: Set(sync_time),
                time_spent_ms: Set(time_spent_ms as i32),
                error_msg: Set(Some(e.to_string())),
                ..Default::default()
            };
            record.insert(&state.conn).await?;
            let err_msg = e.to_string();

            Err(BizError::custom(format!(
                "error on sync world state! {err_msg}"
            )))
        }
    }
}

/// get the last sync time
pub async fn get_nearest_expiry_from_db(state: AppState) -> Result<Option<DateTimeUtc>, DbErr> {
    let r = RawWorldStateEntity::find()
        .filter(entities::raw_world_state::Column::IsSuccess.eq(true))
        .order_by_desc(entities::raw_world_state::Column::SyncTime)
        .one(&state.conn)
        .await?;
    Ok(r.and_then(|m| m.nearest_expiry))
}

fn calc_nearest_expiry(world_state: &RawWorldState) -> Option<DateTime<Utc>> {
    let now = Utc::now();
    let mut expiries = Vec::new();

    // only most important items

    // 虚空裂缝
    let am_expiry = world_state
        .active_missions
        .iter()
        .map(|am| am.expiry)
        .filter(|e| *e > now)
        .min();
    expiries.push(am_expiry);

    // 虚空风暴
    let vs_expiry = world_state
        .void_storms
        .iter()
        .map(|v| v.expiry)
        .filter(|e| *e > now)
        .min();
    expiries.push(vs_expiry);

    expiries.into_iter().flatten().min()
}

pub async fn sync_raw_world_state(state: AppState) -> BizResult<Option<DateTime<Utc>>> {
    let time = Instant::now();
    let sync_time = Utc::now();
    match fetch_raw_world_state(&state.http_client, WG_URL).await {
        Ok(Res { mut raw, ignored }) => {
            load_deps("data", "cache", &state.http_client, false).await?;
            fill_raw_world_state(&mut raw).await?;
            let content = serde_json::to_string(&raw)?;
            std::fs::write("cache/current_world_state.json", content)?;
            let nearest_expiry = calc_nearest_expiry(&raw);
            let time_spent_ms = time.elapsed().as_millis() as i32;
            let record = entities::raw_world_state::ActiveModel {
                sync_time: Set(sync_time),
                time_spent_ms: Set(time_spent_ms),
                error_msg: Set(None),
                is_success: Set(true),
                // text: Set(Some(serde_json::to_string(&raw)?)),
                warnings: if ignored.is_empty() {
                    NotSet
                } else {
                    Set(Some(serde_json::to_string(&ignored)?))
                },
                nearest_expiry: Set(nearest_expiry),
                ..Default::default()
            };

            record.insert(&state.conn).await?;
            Ok(nearest_expiry)
        }
        Err(e) => {
            let time_spent_ms = time.elapsed().as_millis();
            let record = entities::raw_world_state::ActiveModel {
                sync_time: Set(sync_time),
                time_spent_ms: Set(time_spent_ms as i32),
                error_msg: Set(Some(e.to_string())),
                ..Default::default()
            };
            record.insert(&state.conn).await?;
            Ok(None)
        }
    }
}

/// get and parse raw world state from the CN real game server
/// then return to the client
#[axum::debug_handler]
async fn get_parsed_world_state() -> BizResult<BizResponse<serde_json::Value>> {
    let ws = std::fs::read_to_string("cache/current_world_state.json")?;
    let raw: serde_json::Value = serde_json::from_str(&ws)?;
    Ok(BizResponse::ok_with(raw))
}

#[axum::debug_handler]
async fn get_raw_world_state() -> BizResult<BizResponse<serde_json::Value>> {
    let ws = std::fs::read_to_string("cache/raw_world_state.json")?;
    let raw: serde_json::Value = serde_json::from_str(&ws)?;
    Ok(BizResponse::ok_with(raw))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_raw_world_state_wg() {
        let client = reqwest::Client::new();
        let res = fetch_raw_world_state(&client, WG_URL).await;
        if res.is_err() {
            dbg!(&res);
        }
        assert!(res.is_ok());
        let res = res.unwrap();
        dbg!(&res.ignored);
    }

    #[tokio::test]
    async fn test_get_raw_world_state_st() {
        let client = reqwest::Client::new();
        let res = fetch_raw_world_state(&client, ST_URL).await;
        if res.is_err() {
            dbg!(&res);
        }
        assert!(res.is_ok());

        let res = res.unwrap();
        dbg!(&res.ignored);
    }
}
