use axum::Router;
use axum::routing::get;

use crate::AppState;
use crate::api::raw_world_state::parse::load_deps;
use crate::api::world_state::models::WorldStateData;
use crate::api::{BizResponse, BizResult};
use model::RawWorldState;
use parse::parse_raw_world_state;
use std::collections::BTreeSet;

pub use model::*;

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
async fn fetch_raw_world_state(http_client: reqwest::Client, url: &str) -> BizResult<Res> {
    let resp = http_client.get(url).send().await?;

    let body = resp.text().await?;
    // dbg!(&body);

    let mut de = serde_json::Deserializer::from_str(body.as_str());

    let res: RawWorldState = serde_path_to_error::deserialize(&mut de)?;

    let mut de = serde_json::Deserializer::from_str(body.as_str());
    let mut ignored_set = BTreeSet::new();
    let _: RawWorldState = serde_ignored::deserialize(&mut de, |path| {
        ignored_set.insert(path.to_string());
    })?;

    Ok(Res {
        raw: res,
        ignored: ignored_set,
    })
}

pub fn raw_world_state_routes() -> Router<AppState> {
    Router::new().route("/current", get(get_raw_world_state))
}

async fn get_raw_world_state() -> BizResult<BizResponse<WorldStateData>> {
    let client = reqwest::Client::new();
    let d = fetch_raw_world_state(client, WG_URL).await?;
    dbg!(d.ignored);
    let deps = load_deps("../data", Some(d.raw.time))?;
    let data = parse_raw_world_state(d.raw, &deps)?;
    Ok(BizResponse::ok_with(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_raw_world_state_wg() {
        let client = reqwest::Client::new();
        let res = fetch_raw_world_state(client, WG_URL).await;
        // dbg!(&res);
        assert!(res.is_ok());

        let res = res.unwrap();
        dbg!(&res.ignored);
    }

    #[tokio::test]
    async fn test_get_raw_world_state_st() {
        let client = reqwest::Client::new();
        let res = fetch_raw_world_state(client, ST_URL).await;
        // dbg!(&res);
        assert!(res.is_ok());

        let res = res.unwrap();
        dbg!(&res.ignored);
    }
}
