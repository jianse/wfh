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
use sea_orm::TransactionTrait;
use sea_orm::prelude::DateTimeUtc;

use crate::AppState;
use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::api::Page;
use crate::api::Pager;
use crate::api::raw_world_state::parse::fill_raw_world_state;
use crate::entities;
use crate::entities::prelude::RawWorldState as RawWorldStateEntity;
use crate::entities::prelude::SyncRecordStats;
use axum::extract::Query;
use axum::extract::State;
use model::RawWorldState;
pub use model::*;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::time::Duration;
use std::time::Instant;
use tracing::info;

mod deps;
mod helper;
mod model;
pub(super) mod parse;

pub use parse::DepState;

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

/// public raw world state endpoints (no auth)
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/raw", get(get_raw_world_state))
        .route("/parsed", get(get_parsed_world_state))
        .route("/st_pvt", get(get_st_prime_vault_trader))
        .route("/bounty_cycle", get(get_bounty_cycle))
        .nest("/deps", deps::public_routes())
}

/// authed raw world state endpoints
pub fn authed_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/reload", post(force_reload_world_state))
        .route("/sync_records", get(list_sync_records))
        .route("/sync_records/stats", get(get_sync_stats))
        .nest("/deps", deps::authed_routes(state))
}

/// Sync record response model
#[derive(Debug, Serialize)]
pub struct SyncRecordResponse {
    pub id: i32,
    pub sync_time: DateTimeUtc,
    pub is_success: bool,
    pub time_spent_ms: i32,
    pub error_msg: Option<String>,
    pub warnings: Option<String>,
    pub nearest_expiry: Option<DateTimeUtc>,
}

impl From<entities::raw_world_state::Model> for SyncRecordResponse {
    fn from(model: entities::raw_world_state::Model) -> Self {
        Self {
            id: model.id,
            sync_time: model.sync_time,
            is_success: model.is_success,
            time_spent_ms: model.time_spent_ms,
            error_msg: model.error_msg,
            warnings: model.warnings,
            nearest_expiry: model.nearest_expiry,
        }
    }
}

/// List sync records with pagination
#[axum::debug_handler]
async fn list_sync_records(
    State(state): State<AppState>,
    Query(pager): Query<Pager>,
) -> BizResult<BizResponse<Page<SyncRecordResponse>>> {
    use sea_orm::PaginatorTrait;

    let paginator = RawWorldStateEntity::find()
        .order_by_desc(entities::raw_world_state::Column::SyncTime)
        .paginate(&state.conn, pager.page_size);

    // prefer the O(1) aggregate; fall back to a count query if it is missing
    let total = match SyncRecordStats::find_by_id(1).one(&state.conn).await? {
        Some(s) => s.total_count.max(0) as u64,
        None => paginator.num_items().await?,
    };
    let items = paginator.fetch_page(pager.page - 1).await?;

    let records: Vec<SyncRecordResponse> = items.into_iter().map(|m| m.into()).collect();

    Ok(BizResponse::ok_page(
        pager.page,
        pager.page_size,
        total,
        records,
    ))
}

/// Aggregate statistics over all sync records.
#[derive(Debug, Serialize)]
pub struct SyncStatsResponse {
    pub total: i64,
    pub success: i64,
    pub fail: i64,
    pub success_rate: f64,
    pub avg_time_ms: f64,
    pub min_time_ms: Option<i32>,
    pub max_time_ms: Option<i32>,
    pub last_sync_time: Option<DateTimeUtc>,
    pub last_success_time: Option<DateTimeUtc>,
    pub last_fail_time: Option<DateTimeUtc>,
    pub last_error_msg: Option<String>,
    pub updated_at: Option<DateTimeUtc>,
}

/// Statistics of all sync records.
///
/// Served from the `sync_record_stats` aggregate table in O(1); the table is
/// kept in sync transactionally on every insert, so the numbers cover every
/// record and not just the current page.
#[axum::debug_handler]
async fn get_sync_stats(State(state): State<AppState>) -> BizResult<BizResponse<SyncStatsResponse>> {
    let s = SyncRecordStats::find_by_id(1)
        .one(&state.conn)
        .await?
        .ok_or_else(|| BizError::custom("sync statistics not initialized"))?;

    let total = s.total_count;
    let success_rate = if total > 0 {
        s.success_count as f64 / total as f64
    } else {
        0.0
    };
    let avg_time_ms = if total > 0 {
        s.total_time_ms as f64 / total as f64
    } else {
        0.0
    };

    Ok(BizResponse::ok_with(SyncStatsResponse {
        total,
        success: s.success_count,
        fail: s.fail_count,
        success_rate,
        avg_time_ms,
        min_time_ms: s.min_time_ms,
        max_time_ms: s.max_time_ms,
        last_sync_time: s.last_sync_time,
        last_success_time: s.last_success_time,
        last_fail_time: s.last_fail_time,
        last_error_msg: s.last_error_msg,
        updated_at: s.updated_at,
    }))
}

/// Insert a sync record and update the aggregate statistics in one transaction.
async fn insert_sync_record(
    conn: &sea_orm::DatabaseConnection,
    record: entities::raw_world_state::ActiveModel,
) -> Result<entities::raw_world_state::Model, DbErr> {
    let txn = conn.begin().await?;
    let inserted = record.insert(&txn).await?;
    bump_sync_stats(&txn, &inserted).await?;
    txn.commit().await?;
    Ok(inserted)
}

/// Fold a freshly inserted sync record into the single-row statistics table.
async fn bump_sync_stats(
    txn: &sea_orm::DatabaseTransaction,
    rec: &entities::raw_world_state::Model,
) -> Result<(), DbErr> {
    const STATS_ID: i32 = 1;
    let now = Utc::now();

    match SyncRecordStats::find_by_id(STATS_ID).one(txn).await? {
        Some(s) => {
            let mut am = entities::sync_record_stats::ActiveModel {
                id: Set(s.id),
                total_count: Set(s.total_count + 1),
                success_count: Set(s.success_count),
                fail_count: Set(s.fail_count),
                total_time_ms: Set(s.total_time_ms + rec.time_spent_ms as i64),
                min_time_ms: Set(Some(
                    s.min_time_ms
                        .map_or(rec.time_spent_ms, |m| m.min(rec.time_spent_ms)),
                )),
                max_time_ms: Set(Some(
                    s.max_time_ms
                        .map_or(rec.time_spent_ms, |m| m.max(rec.time_spent_ms)),
                )),
                last_sync_time: Set(Some(rec.sync_time)),
                last_success_time: Set(s.last_success_time),
                last_fail_time: Set(s.last_fail_time),
                last_error_msg: Set(s.last_error_msg.clone()),
                updated_at: Set(Some(now)),
            };
            if rec.is_success {
                am.success_count = Set(s.success_count + 1);
                am.last_success_time = Set(Some(rec.sync_time));
            } else {
                am.fail_count = Set(s.fail_count + 1);
                am.last_fail_time = Set(Some(rec.sync_time));
                am.last_error_msg = Set(rec.error_msg.clone());
            }
            am.update(txn).await?;
        }
        None => {
            let am = entities::sync_record_stats::ActiveModel {
                id: Set(STATS_ID),
                total_count: Set(1),
                success_count: Set(if rec.is_success { 1 } else { 0 }),
                fail_count: Set(if rec.is_success { 0 } else { 1 }),
                total_time_ms: Set(rec.time_spent_ms as i64),
                min_time_ms: Set(Some(rec.time_spent_ms)),
                max_time_ms: Set(Some(rec.time_spent_ms)),
                last_sync_time: Set(Some(rec.sync_time)),
                last_success_time: Set(if rec.is_success {
                    Some(rec.sync_time)
                } else {
                    None
                }),
                last_fail_time: Set(if rec.is_success {
                    None
                } else {
                    Some(rec.sync_time)
                }),
                last_error_msg: Set(if rec.is_success {
                    None
                } else {
                    rec.error_msg.clone()
                }),
                updated_at: Set(Some(now)),
            };
            am.insert(txn).await?;
        }
    }
    Ok(())
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
            // state.deps.load_deps().await?;
            let deps = state.deps.deps.read().await;
            fill_raw_world_state(&mut raw, &deps).await?;
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

            insert_sync_record(&state.conn, record).await?;
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
            insert_sync_record(&state.conn, record).await?;
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
            // load_deps("data", "cache", &state.http_client, false).await?;
            let deps = state.deps.get_deps().await;
            fill_raw_world_state(&mut raw, &deps).await?;
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

            insert_sync_record(&state.conn, record).await?;
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
            insert_sync_record(&state.conn, record).await?;
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

#[axum::debug_handler]
async fn get_bounty_cycle() -> BizResult<BizResponse<serde_json::Value>> {
    let res = std::fs::read_to_string(BOUNTY_CYCLE_SAVE_PATH)?;
    let res: serde_json::Value = serde_json::from_str(&res)?;
    Ok(BizResponse::ok_with(res))
}

const BOUNTY_CYCLE_SAVE_PATH: &str = "cache/bounty_cycle.json";

/// get bounty-cycle from https://oracle.browse.wf/bounty-cycle
pub async fn download_bounty_cycle(state: AppState) -> BizResult<Option<Duration>> {
    let res: serde_json::Value = state
        .http_client
        .get("https://oracle.browse.wf/bounty-cycle")
        .timeout(Duration::from_secs(20))
        .send()
        .await?
        .json()
        .await?;
    // Ok(res)
    let expiry = res
        .as_object()
        .ok_or(BizError::custom("result is not an object"))?
        .get("expiry")
        .ok_or(BizError::custom("expiry not found"))?
        .as_i64()
        .ok_or(BizError::custom("expiry not int64"))?;

    // let expiry = Duration::from_secs(expiry as u64);
    // let time = Utc::from_timestamp_millis(expiry );
    let e_time =
        DateTime::from_timestamp_millis(expiry).ok_or(BizError::custom("expiry not datetime"))?;
    let d = (e_time - Utc::now()).to_std().ok();
    std::fs::write(BOUNTY_CYCLE_SAVE_PATH, res.to_string())?;
    Ok(d)
}

pub fn get_bounty_cycle_expiry() -> BizResult<Option<Duration>> {
    if fs::exists(BOUNTY_CYCLE_SAVE_PATH)? {
        let res = std::fs::read_to_string(BOUNTY_CYCLE_SAVE_PATH)?;
        let res: serde_json::Value = serde_json::from_str(&res)?;
        let expiry = res
            .as_object()
            .ok_or(BizError::custom("result is not an object"))?
            .get("expiry")
            .ok_or(BizError::custom("expiry not found"))?
            .as_i64()
            .ok_or(BizError::custom("expiry not int64"))?;

        // let expiry = Duration::from_secs(expiry as u64);
        // let time = Utc::from_timestamp_millis(expiry );
        let e_time = DateTime::from_timestamp_millis(expiry)
            .ok_or(BizError::custom("expiry not datetime"))?;
        let d = (e_time - Utc::now()).to_std().ok();
        std::fs::write(BOUNTY_CYCLE_SAVE_PATH, res.to_string())?;
        Ok(d)
    } else {
        Ok(None)
    }
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
