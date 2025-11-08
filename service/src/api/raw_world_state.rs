use axum::Router;
use axum::routing::get;

use crate::AppState;
use crate::api::{BizResponse, BizResult};
use axum::extract::State;
use model::RawWorldState;
pub use model::*;
use std::collections::BTreeSet;
use std::time::Instant;
use tracing::info;

mod helper;
mod model;
mod parse;

#[derive(Debug)]
pub struct Res {
    pub raw: RawWorldState,
    pub ignored: BTreeSet<String>,
}

static WG_URL: &str = "https://content-zhpt.wf.qq.com/dynamic/worldState.php";
static ST_URL: &str = "https://content.warframe.com/dynamic/worldState.php";
async fn fetch_raw_world_state(http_client: &reqwest::Client, url: &str) -> BizResult<Res> {
    let time = Instant::now();

    let resp = http_client.get(url).send().await?;
    info!(
        "fetch_raw_world_state time: {} ms.",
        time.elapsed().as_millis()
    );

    let body = resp.text().await?;
    // dbg!(&body);

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
    info!(
        "parse_raw_world_state time: {} ms.",
        time.elapsed().as_millis()
    );

    Ok(Res {
        raw,
        ignored: ignored_set,
    })
}

pub fn raw_world_state_routes() -> Router<AppState> {
    Router::new().route("/current", get(get_raw_world_state))
}

async fn get_raw_world_state(
    State(state): State<AppState>,
) -> BizResult<BizResponse<RawWorldState>> {
    // let client = reqwest::Client::new();
    let d = fetch_raw_world_state(&state.http_client, WG_URL).await?;
    dbg!(d.ignored);
    // let deps = load_deps("../data", Some(d.raw.time))?;
    // let data = parse_raw_world_state(d.raw, &deps)?;
    Ok(BizResponse::ok_with(d.raw))
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
