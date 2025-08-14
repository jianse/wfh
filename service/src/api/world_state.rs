mod helper;
pub mod models;
mod rep;

use std::{
    collections::HashMap,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use crate::{
    AppState,
    api::{BizError, BizResponse, BizResult, world_state::rep::rep_routes},
    common::{brotli_compress, brotli_decompress},
    entities::{
        prelude::{WgWorldStateReplacements, WorldState2},
        wg_world_state_replacements, world_state2,
    },
};
use axum::{Router, extract::State, routing::get};
use chrono::{DateTime, Utc};
use hex::ToHex;
use http::HeaderMap;
use regex::Regex;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    prelude::DateTimeUtc,
    sea_query,
};
use serde::{Deserialize, Serialize};

use models::{WorldStateData, WorldStateRes};

use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct WorldStateOut {
    pub raw: String,
    pub data: Box<WorldStateData>,
}

pub struct WorldStateError {
    pub raw: Option<String>,
    // pub data: Option<WorldStateData>,
    pub err: BizError,
}

impl From<BizError> for WorldStateError {
    fn from(err: BizError) -> Self {
        WorldStateError {
            raw: None,
            // data: None,
            err,
        }
    }
}

impl From<reqwest::Error> for WorldStateError {
    fn from(err: reqwest::Error) -> Self {
        WorldStateError {
            raw: Some(err.to_string()),
            // data: None,
            err: BizError::CallPeer(err),
        }
    }
}

async fn get_world_state(state: AppState) -> Result<WorldStateOut, WorldStateError> {
    // let mut res = WorldStateOut::default();
    let start = std::time::Instant::now();

    let url = build_url()?;

    let resp = state.http_client.get(url).headers(headers()).send().await?;

    debug!("{resp:?}");
    let raw = resp.text().await?;

    info!(
        "query world state wegame network took: {} ms",
        start.elapsed().as_millis()
    );

    let mut res = serde_json::Deserializer::from_str(raw.as_str());
    let res: WorldStateRes =
        serde_path_to_error::deserialize(&mut res).map_err(|e| WorldStateError {
            raw: Some(raw.clone()),
            err: e.into(),
        })?;

    let data = match res {
        WorldStateRes::Ok(data) => Ok(data),
        WorldStateRes::Err(e) => Err(WorldStateError {
            raw: Some(raw.clone()),
            err: BizError::PeerResponse(e.0.clone(), e.message().to_owned()),
        }),
    }?;

    Ok(WorldStateOut { raw, data })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RewriteWarning {
    warnings: Vec<RewriteWarningItem>,
}

impl RewriteWarning {
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
        }
    }

    pub fn add_item(&mut self, path: String, value: String, desc: String) {
        self.warnings.push(RewriteWarningItem { path, value, desc });
    }

    pub fn into_option(self) -> Option<Self> {
        if self.warnings.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RewriteWarningItem {
    pub path: String,
    pub value: String,
    pub desc: String,
}

pub fn get_nearest_expiry(data: &WorldStateData) -> Option<DateTime<Utc>> {
    let mut expiries = Vec::new();

    let event_expiry = data
        .events
        .iter()
        .filter(|e| !e.expired)
        .map(|e| e.expiry)
        .min();
    expiries.push(event_expiry);

    let alert_expiry = data.alerts.iter().map(|a| a.expiry).min();
    expiries.push(alert_expiry);

    let sortie_expiry = data.sortie.expiry;
    expiries.push(Some(sortie_expiry));

    let syndicate_missions_expiry = data.syndicate_missions.iter().map(|a| a.expiry).min();
    expiries.push(syndicate_missions_expiry);

    let fissures_expiry = data
        .fissures
        .iter()
        .filter(|f| !f.expired)
        .map(|a| a.expiry)
        .min();
    expiries.push(fissures_expiry);

    let global_upgrades_expiry = data.global_upgrades.iter().map(|a| a.end).min();
    expiries.push(global_upgrades_expiry);

    let flash_sales_expiry = data.flash_sales.iter().map(|a| a.expiry).min();
    expiries.push(flash_sales_expiry);

    let void_trader_expiry = [data.void_trader.activation, data.void_trader.expiry]
        .into_iter()
        .filter(|t| t > &data.timestamp)
        .min();

    expiries.push(void_trader_expiry);

    let daily_deals_expiry = data.daily_deals.iter().map(|a| a.expiry).min();
    expiries.push(daily_deals_expiry);

    let conclave_challenges_expiry = data.conclave_challenges.iter().map(|a| a.expiry).min();
    expiries.push(conclave_challenges_expiry);

    expiries.push(Some(data.earth_cycle.expiry));
    expiries.push(Some(data.cetus_cycle.expiry));
    expiries.push(Some(data.vallis_cycle.expiry));
    expiries.push(Some(data.cambion_cycle.expiry));
    expiries.push(Some(data.zariman_cycle.expiry));

    expiries.push(Some(data.nightwave.expiry));

    let nightwave_challenges_expiry = data
        .nightwave
        .active_challenges
        .iter()
        .map(|a| a.expiry)
        .min();
    expiries.push(nightwave_challenges_expiry);

    expiries.push(Some(data.steel_path.expiry));

    expiries.push(Some(data.vault_trader.expiry));

    expiries.push(Some(data.archon_hunt.expiry));

    let void_storms_expiry = data
        .void_storms
        .iter()
        .filter(|v| v.expiry > data.timestamp)
        .map(|a| a.expiry)
        .min();
    expiries.push(void_storms_expiry);

    #[cfg(test)]
    {
        println!("expiries");
        dbg!(&expiries);
        println!("timestamp");
        dbg!(data.timestamp);
    }

    let mut e_list = Vec::new();
    for i in expiries.into_iter().flatten() {
        e_list.push(i);
    }

    let t = e_list.into_iter().min();

    #[cfg(test)]
    {
        println!("min expiry");
        dbg!(t);
    }

    if let Some(t) = t
        && t < data.timestamp
    {
        warn!("expiry time is in the past, this should not happen");
    }

    t
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RewriteKey(String, String);

impl<S> From<(S, S)> for RewriteKey
where
    S: Into<String>,
{
    fn from(value: (S, S)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

// impl Eq for RewriteKey {}

async fn load_rewrite_item_map(
    state: AppState,
) -> Result<HashMap<RewriteKey, Option<String>>, DbErr> {
    let reps = WgWorldStateReplacements::find().all(&state.conn).await?;
    Ok(reps
        .into_iter()
        .map(|r| ((r.path, r.origin).into(), r.replacement))
        .collect())
}

fn rewrite_one(
    val: &mut String,
    warnings: &mut RewriteWarning,
    path: String,
    replacements: &HashMap<RewriteKey, Option<String>>,
) {
    let re = Regex::new(r"\[\d+\]").unwrap();
    let cp = re.replace_all(&path, "[*]").into_owned();

    if let Some(ni) = replacements.get(&(cp.clone(), val.to_owned()).into()) {
        if let Some(ni) = ni {
            *val = ni.to_string();
        }
    } else {
        warnings.add_item(path, val.to_owned(), format!("unexpected item: {val} "));
    }
}

async fn rewrite_world_state(
    state: AppState,
    data: Box<WorldStateData>,
) -> Result<(Box<WorldStateData>, Option<RewriteWarning>), DbErr> {
    let replacements = load_rewrite_item_map(state.clone()).await?;
    let mut warnings = RewriteWarning::new();
    let mut data = data;

    // rewrite events
    data.events.iter_mut().enumerate().for_each(|(i, event)| {
        let path = format!("$.events[{i}]");

        rewrite_one(
            &mut event.node,
            &mut warnings,
            format!("{path}.node"),
            &replacements,
        );

        event.rewards.iter_mut().enumerate().for_each(|(i, r)| {
            let path = format!("{path}.rewards[{i}]");

            r.items.iter_mut().enumerate().for_each(|(i, it)| {
                let path = format!("{path}.items[{i}]");
                rewrite_one(it, &mut warnings, path, &replacements);
            });
        });

        event
            .interim_steps
            .iter_mut()
            .enumerate()
            .for_each(|(i, s)| {
                let path = format!("{path}.interimSteps[{i}]");

                s.reward.items.iter_mut().enumerate().for_each(|(i, it)| {
                    let path = format!("{path}.reward.items[{i}]");

                    rewrite_one(it, &mut warnings, path, &replacements);
                })
            });

        // Ok(())
    });

    // rewrite fissures
    data.fissures
        .iter_mut()
        .enumerate()
        .for_each(|(i, fissure)| {
            let path = format!("$.fissures[{i}]");
            rewrite_one(
                &mut fissure.node,
                &mut warnings,
                format!("{path}.node"),
                &replacements,
            );

            rewrite_one(
                &mut fissure.mission_type,
                &mut warnings,
                format!("{path}.missionType"),
                &replacements,
            );
            rewrite_one(
                &mut fissure.tier,
                &mut warnings,
                format!("{path}.tier"),
                &replacements,
            );
            rewrite_one(
                &mut fissure.tier_num,
                &mut warnings,
                format!("{path}.tierNum"),
                &replacements,
            );
        });

    // archon hunt
    data.archon_hunt
        .missions
        .iter_mut()
        .enumerate()
        .for_each(|(i, m)| {
            let path = format!("$.archonHunt.missions[{i}]");
            rewrite_one(
                &mut m.node,
                &mut warnings,
                format!("{path}.node"),
                &replacements,
            );
            rewrite_one(
                &mut m.r#type,
                &mut warnings,
                format!("{path}.type"),
                &replacements,
            );
        });

    // sentient outposts
    rewrite_one(
        &mut data.sentient_outposts.mission.faction,
        &mut warnings,
        "$.sentientOutposts.mission.faction".to_owned(),
        &replacements,
    );
    rewrite_one(
        &mut data.sentient_outposts.mission.node,
        &mut warnings,
        "$.sentientOutposts.mission.node".to_owned(),
        &replacements,
    );
    rewrite_one(
        &mut data.sentient_outposts.mission.r#type,
        &mut warnings,
        "$.sentientOutposts.mission.type".to_owned(),
        &replacements,
    );

    // vault trader
    data.vault_trader
        .inventory
        .iter_mut()
        .enumerate()
        .for_each(|(idx, i)| {
            let path = format!("$.vaultTrader.inventory[{idx}]");
            rewrite_one(
                &mut i.item,
                &mut warnings,
                format!("{path}.item"),
                &replacements,
            );
        });

    // nightwave
    data.nightwave
        .active_challenges
        .iter_mut()
        .enumerate()
        .for_each(|(idx, c)| {
            let path = format!("$.nightwave.activeChallenges[{idx}]");
            rewrite_one(
                &mut c.title,
                &mut warnings,
                format!("{path}.title"),
                &replacements,
            );
            rewrite_one(
                &mut c.desc,
                &mut warnings,
                format!("{path}.desc"),
                &replacements,
            );
        });

    write_unchecked_replacements(state, &warnings).await?;

    Ok((data, warnings.into_option()))
}

async fn write_unchecked_replacements(
    state: AppState,
    warnings: &RewriteWarning,
) -> Result<(), DbErr> {
    // writes all warnings to db
    let rs = warnings
        .warnings
        .iter()
        .map(|w| {
            let re = Regex::new(r"\[\d+\]").unwrap();
            let cp = re.replace_all(&w.path, "[*]").into_owned();
            wg_world_state_replacements::ActiveModel {
                path: Set(cp),
                origin: Set(w.value.clone()),
                replacement: Set(None),
                remark: Set(Some("world state rewrite".to_owned())),
            }
        })
        .collect::<Vec<wg_world_state_replacements::ActiveModel>>();

    WgWorldStateReplacements::insert_many(rs)
        .on_empty_do_nothing()
        .on_conflict(
            sea_query::OnConflict::columns([
                wg_world_state_replacements::Column::Path,
                wg_world_state_replacements::Column::Origin,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec(&state.conn)
        .await?;
    Ok(())
}

#[axum::debug_handler]
async fn world_state(State(state): State<AppState>) -> BizResult<BizResponse<serde_json::Value>> {
    let r = WorldState2::find()
        .filter(world_state2::Column::IsSuccess.eq(true))
        .order_by_desc(world_state2::Column::SyncTime)
        .one(&state.conn)
        .await?;
    match r {
        Some(m) => {
            let t = m.text.ok_or(BizError::custom("no world state"))?;
            let t = brotli_decompress(t).await?;
            let s = serde_json::from_slice::<serde_json::Value>(&t)?;
            Ok(BizResponse::ok_with(s))
        }
        None => Err(BizError::custom("no world state"))?,
    }
}

/// build request header
fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        "application/json, text/javascript, */*; q=0.01"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "Accept-Encoding",
        "gzip, deflate, br, zstd".parse().unwrap(),
    );
    headers.insert(
        "Accept-Language",
        "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2"
            .parse()
            .unwrap(),
    );
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Host", "www.wegame.com.cn".parse().unwrap());
    headers.insert(
        "Referer",
        "https://www.wegame.com.cn/act/xjzj/xjzj20220330/index.html?hcfrom=WeGame.helper"
            .parse()
            .unwrap(),
    );
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0"
            .parse()
            .unwrap(),
    );
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());

    headers
}

/// build request url
fn build_url() -> Result<String, BizError> {
    let url =
        "https://www.wegame.com.cn/api/act/index.php/xjzj/xjzj20220330/index/ajax_get_worldState";
    let uuid = uuid::Uuid::new_v4().to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let u = format!(
        "xjzj/xjzj20220330/index/ajax_get_worldState?uuid={uuid}&version=2&appid=10003&t={timestamp}"
    );

    let sign = md5::compute(u).encode_hex::<String>();

    let res = format!("{url}?uuid={uuid}&version=2&appid=10003&t={timestamp}&sign={sign}");
    Ok(res)
}

/// update world state and return to client
async fn update_and_get(
    State(state): State<AppState>,
) -> BizResult<BizResponse<Box<WorldStateData>>> {
    // todo!("not implemented")
    let d = wf_world_state(state).await?;
    Ok(BizResponse::ok_with(d))
}

pub async fn wf_world_state(state: AppState) -> Result<Box<WorldStateData>, BizError> {
    let start = Instant::now();
    let res = get_world_state(state.clone()).await;

    match res {
        Ok(ws) => {
            let nearest_expiry = get_nearest_expiry(&ws.data);
            let (data, warn) = rewrite_world_state(state.clone(), ws.data).await?;
            let spent = start.elapsed();
            write_world_state_into_db(state, ws.raw, &data, warn, spent, nearest_expiry).await?;
            Ok(data)
        }
        Err(err) => {
            let spent = start.elapsed();
            log_error(state.clone(), &err, spent).await?;
            Err(err.err)
        }
    }
}

/// logging get warframe world state error into db.
async fn log_error(
    state: AppState,
    err: &WorldStateError,
    spent: Duration,
) -> Result<(), BizError> {
    let raw = match err.raw.as_ref() {
        Some(raw) => {
            let t = brotli_compress(raw.to_owned()).await?;
            Some(t)
        }
        None => None,
    };
    error!("{}", err.err.to_string());
    let row = world_state2::ActiveModel {
        sync_time: Set(Utc::now()),
        is_success: Set(false),
        error_msg: Set(Some(err.err.to_string())),
        time_spent_ms: Set(spent.as_millis() as i32),
        raw: Set(raw),
        ..Default::default()
    };
    WorldState2::insert(row).exec(&state.conn).await?;
    Ok(())
}

/// save warframe world state into db
async fn write_world_state_into_db(
    state: AppState,
    raw: String,
    data: &WorldStateData,
    warn: Option<RewriteWarning>,
    spent: Duration,
    nearest_expiry: Option<DateTime<Utc>>,
) -> BizResult<()> {
    let warn_str = serde_json::to_string(&warn)?;
    let t = serde_json::to_string(&data)?;
    let t = brotli_compress(t).await?;

    let raw = brotli_compress(raw).await?;

    let row = world_state2::ActiveModel {
        id: NotSet,
        sync_time: Set(Utc::now()),
        text: Set(Some(t)),
        is_success: Set(true),
        error_msg: Set(None),
        time_spent_ms: Set(spent.as_millis() as i32),
        raw: Set(Some(raw)),
        warnings: Set(Some(warn_str)),
        nearest_expiry: Set(nearest_expiry),
    };
    WorldState2::insert(row).exec(&state.conn).await?;
    Ok(())
}

/// get the last sync time
pub async fn get_nearest_expiry_from_db(state: AppState) -> Result<Option<DateTimeUtc>, DbErr> {
    let r = WorldState2::find()
        .filter(world_state2::Column::IsSuccess.eq(true))
        .order_by_desc(world_state2::Column::SyncTime)
        .one(&state.conn)
        .await?;
    Ok(r.and_then(|m| m.nearest_expiry))
}

pub(crate) fn world_state_routes() -> Router<AppState> {
    Router::new()
        .route("/current", get(world_state))
        .route("/update", get(update_and_get))
        .nest("/rep", rep_routes())
}

#[cfg(test)]
mod tests {

    use std::fs::File;

    use chrono::Duration;
    use reqwest::Client;

    use super::*;
    #[tokio::test]
    async fn test_get_world_state_url() -> eyre::Result<()> {
        let url = build_url()?;
        let resp = Client::builder()
            .build()?
            .get(url)
            .headers(headers())
            .send()
            .await?;
        let s = resp.text().await?;
        let s = serde_json::from_str::<WorldStateRes>(s.as_str())?;

        dbg!(&s);
        Ok(())
    }

    #[test]
    fn test_dur() {
        let d = Duration::new(-123456, 0).unwrap();
        println!("{}", d);
    }

    #[test]
    fn test_static_json() {
        let file = File::open("../ui/cache/worldState_wg.json");
        match file {
            Ok(f) => {
                let mut s = serde_json::Deserializer::from_reader(f);
                let s = serde_path_to_error::deserialize::<_, WorldStateRes>(&mut s);

                match s {
                    Ok(wsr) => {
                        dbg!(&wsr);
                    }
                    Err(e) => {
                        println!("{}", e.to_string());
                        assert!(false);
                    }
                };
            }
            Err(_) => {
                println!("file not found");
            }
        }
    }

    #[test]
    fn test_get_expiry_time() {
        let file = File::open("../ui/cache/worldState_wg.json");
        match file {
            Ok(f) => {
                let s = serde_json::from_reader::<_, WorldStateRes>(f).unwrap();
                match s {
                    WorldStateRes::Ok(data) => {
                        // dbg!(&data.timestamp);

                        let expiry_time = get_nearest_expiry(&data);
                        // dbg!(data);
                        // let (_res, warn) = rewrite_world_state(s.data);
                        // dbg!(&s);
                        // dbg!(&expiry_time);
                        assert!(expiry_time.is_some());
                        assert!(expiry_time.unwrap() > data.timestamp);
                    }
                    WorldStateRes::Err(_) => todo!(),
                }
            }
            Err(_) => {
                println!("file not found");
            }
        }
    }
}
