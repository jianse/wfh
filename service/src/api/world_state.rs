mod helper;
pub mod models;

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    AppState,
    api::{BizError, BizResponse, BizResult},
    common::brotli_decompress,
    entities::{
        prelude::{WgWorldStateReplacements, WorldState2},
        world_state2,
    },
};
use axum::{Router, extract::State, routing::get};
use chrono::{DateTime, Utc};
use hex::ToHex;
use http::HeaderMap;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use models::{WorldStateData, WorldStateRes};

use tracing::{debug, info, warn};

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

pub async fn get_world_state(state: AppState) -> Result<WorldStateOut, WorldStateError> {
    // let mut res = WorldStateOut::default();
    let start = std::time::Instant::now();

    let url = build_url()?;

    let resp = state.http_client.get(url).headers(headers()).send().await?;

    debug!("{resp:?}");
    let raw = resp.text().await?;

    info!(
        "query world state wegame took: {} ms",
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

    pub fn add_item(&mut self, path: String, desc: String) {
        self.warnings.push(RewriteWarningItem { path, desc });
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

    expiries.push(Some(data.void_trader.activation));
    expiries.push(Some(data.void_trader.expiry));

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

    // dbg!(&expiries);

    let t = expiries
        .into_iter()
        .collect::<Option<Vec<DateTime<Utc>>>>()
        .and_then(|v| v.into_iter().min());

    if let Some(t) = t
        && t < data.timestamp
    {
        warn!("expiry time is in the past, this should not happen");
    }

    t
}

async fn load_rewrite_item_map(state: AppState) -> Result<HashMap<String, Option<String>>, DbErr> {
    let reps = WgWorldStateReplacements::find().all(&state.conn).await?;
    Ok(reps
        .into_iter()
        .map(|r| (r.origin, r.replacement))
        .collect())
}

fn rewrite_one(
    val: &mut String,
    warnings: &mut RewriteWarning,
    path: String,
    replacements: &HashMap<String, Option<String>>,
) {
    if let Some(ni) = replacements.get(val) {
        if let Some(ni) = ni {
            *val = ni.to_string();
        }
    } else {
        warnings.add_item(path, format!("unexpected item: {val} "));
    }
}

pub async fn rewrite_world_state(
    state: AppState,
    data: Box<WorldStateData>,
) -> Result<(Box<WorldStateData>, Option<RewriteWarning>), DbErr> {
    let replacements = load_rewrite_item_map(state).await?;
    let mut warnings = RewriteWarning::new();
    let mut data = data;
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
            })
    });
    Ok((data, warnings.into_option()))
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

pub(crate) fn world_state_routes() -> Router<AppState> {
    Router::new().route("/current", get(world_state))
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
                        dbg!(&data.timestamp);

                        let expiry_time = get_nearest_expiry(&data);
                        // let (_res, warn) = rewrite_world_state(s.data);
                        // dbg!(&s);
                        dbg!(&expiry_time);
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
