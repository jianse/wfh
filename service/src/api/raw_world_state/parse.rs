use chrono::{DateTime, Utc};

use crate::api::{
    raw_world_state::model::RawWorldState, world_state::models::{Alert, AlertMission, EarthCycle, EventReward, WorldStateData}, BizError, BizResult
};

pub fn parse_raw_world_state(raw: RawWorldState) -> BizResult<WorldStateData> {
    let deps = load_deps(Some(raw.time))?;

    // time that this world state was generated
    let timestamp = raw.time;

    let alerts: Vec<Alert> = raw
        .alerts
        .iter()
        .map(|alert| alert_from_raw(alert, &deps))
        .collect();

    dbg!(&alerts);

    // let news =

    // earth cycle
    let earth_cycle = EarthCycle::from_time(raw.time);
    // dbg!(&earth_cycle);
    // WorldStateData
    // todo!();
    Err(BizError::custom("unimplemented"))
}

fn load_deps(option_time: Option<DateTime<Utc>>) -> BizResult<Deps> {
    Ok(Deps {
        time: option_time.unwrap_or(Utc::now()),
        sortie_data: (),
    })
}

fn alert_from_raw(raw: &super::Alert, deps: &Deps) -> Alert {
    let start_string = raw.activation - deps.time;

    Alert {
        id: raw.id.clone(),
        activation: raw.activation,
        start_string,
        expiry: raw.expiry,
        active: raw.expiry >= deps.time && deps.time >= raw.activation,
        mission: alert_mission_from_raw(&raw.mission_info, deps),
        eta: raw.expiry - deps.time,
        reward_types: Vec::new(),
        tag: raw.tag.clone(),
    }
}

fn alert_mission_from_raw(raw: &super::MissionInfo, deps: &Deps) -> AlertMission {
    // todo!()
    AlertMission {
        description: raw.desc_text.clone().unwrap_or("".to_owned()),
        node: raw.location.clone(),
        node_key: raw.location.clone(),
        r#type: raw.mission_type.clone(),
        type_key: raw.mission_type.clone(),
        faction: raw.faction.clone(),
        faction_key: raw.faction.clone(),
        min_enemy_level: raw.min_enemy_level,
        max_enemy_level: raw.max_enemy_level,
        nightmare: raw.nightmare.unwrap_or(false),
        archwing_required: raw.archwing_required.unwrap_or(false),
        is_sharkwing: raw.is_sharkwing_mission.unwrap_or(false),
        level_override: raw.level_override.clone(),
        enemy_spec: raw.enemy_spec.clone(),
        advanced_spawners: raw.advanced_spawners.clone().unwrap_or(Vec::new()),
        required_items: Vec::new(),
        level_auras: Vec::new(),
        reward: EventReward {
            items: Vec::new(),
            counted_items:  Vec::new(),
            credits: 10000,
            as_string: "".to_owned(),
            item_string: "".to_owned(),
            thumbnail: "".to_owned(),
            color: "".to_owned(),
        },
    }
}

#[derive(Debug)]
struct Deps {
    time: DateTime<Utc>,
    sortie_data: (),
}

#[cfg(test)]
mod tests{
    use std::fs::File;

    use super::*;

    #[test]
    fn test_parse_raw_world_state() {
        let mut file = File::open("./tests/samples/full/ws_wg_org-20250907.json").unwrap();
        let raw = serde_json::from_reader::<_,RawWorldState>(&mut file).unwrap();
        let result = parse_raw_world_state(raw);
        dbg!(&result);
        // assert!(result.is_ok());
    }
}