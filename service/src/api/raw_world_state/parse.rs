use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    path::Path,
};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{FromInto, TryFromInto, serde_as};
use tracing::info;

use crate::api::{
    BizError, BizResult,
    raw_world_state::model::RawWorldState,
    world_state::models::{
        Alert, AlertMission, CountedItem, EarthCycle, EventReward, WorldStateData,
    },
};

pub fn parse_raw_world_state(raw: RawWorldState, deps: &Deps) -> BizResult<WorldStateData> {
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

pub fn load_deps<P: AsRef<Path> + Display>(
    dir_path: P,
    option_time: Option<DateTime<Utc>>,
) -> BizResult<Deps> {
    let languages = load_languages(format!("{dir_path}/languages.json"))?;
    // let nodes = load_nodes(format!("{dir_path}/nodes.json"))?;
    let factions = load_factions(format!("{dir_path}/factions.json"))?;
    let mission_types = load_mission_types(format!("{dir_path}/mission_types.json"))?;
    let regions = load_regions(format!("{dir_path}/Regions.json"))?;
    Ok(Deps {
        time: option_time.unwrap_or_else(Utc::now),
        // nodes,
        regions,
        mission_types,
        factions,
        languages,
        // ..Default::default()
    })
}

fn load_mission_types<P: AsRef<Path>>(file: P) -> BizResult<HashMap<String, MissionType>> {
    // dbg!(std::path::absolute(&file));
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let res: Vec<MissionType> = serde_json::from_reader(&mut file)?;
        let res = res.into_iter().map(|k| (k.key.clone(), k)).collect();
        Ok(res)
    } else {
        info!("no mission_types.json found, using default");
        Ok(HashMap::new())
    }
}

fn load_factions<P: AsRef<Path>>(file: P) -> BizResult<HashMap<String, FactionInfo>> {
    // dbg!(std::path::absolute(&file));
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let res: Vec<FactionInfo> = serde_json::from_reader(&mut file)?;
        let res = res.into_iter().map(|k| (k.key.clone(), k)).collect();
        Ok(res)
    } else {
        info!("no factions.json found, using default");
        Ok(HashMap::new())
    }
}

fn load_regions<P: AsRef<Path>>(file: P) -> BizResult<HashMap<String, Region>> {
    // dbg!(std::path::absolute(&file));
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let res: ExportRegions = serde_json::from_reader(&mut file)?;
        let res = res
            .regions
            .into_iter()
            .map(|k| (k.unique_name.clone(), k))
            .collect();
        Ok(res)
    } else {
        info!("no Regions.json found, using default");
        Ok(HashMap::new())
    }
}
fn load_languages<P: AsRef<Path>>(file: P) -> BizResult<HashMap<String, String>> {
    // dbg!(std::path::absolute(&file));
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let res: HashMap<String, String> = serde_json::from_reader(&mut file)?;
        Ok(res)
    } else {
        info!("no languages.json found, using default");
        Ok(HashMap::new())
    }

    // let mut result = HashMap::new();
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
    let description = raw.desc_text.clone().map(|k| deps.get_language(&k));

    let node = deps
        .get_region(&raw.location)
        .map(|n| n.name.clone())
        .unwrap_or(raw.location.clone());

    let r#type = deps.get_mission_type(&raw.mission_type);

    let faction = deps.get_faction(&raw.faction);

    let reward = reward_from_raw(&raw.mission_reward, deps);

    AlertMission {
        description,
        node,
        node_key: raw.location.clone(),
        r#type,
        type_key: raw.mission_type.clone(),
        faction,
        faction_key: raw.faction.clone(),
        min_enemy_level: raw.min_enemy_level,
        max_enemy_level: raw.max_enemy_level,
        nightmare: raw.nightmare.unwrap_or(false),
        archwing_required: raw.archwing_required.unwrap_or(false),
        is_sharkwing: raw.is_sharkwing_mission.unwrap_or(false),
        level_override: raw.level_override.clone(),
        enemy_spec: raw.enemy_spec.clone(),
        advanced_spawners: raw.advanced_spawners.clone().unwrap_or_default(),
        required_items: Vec::new(),
        level_auras: Vec::new(),
        reward,
    }
}

fn reward_from_raw(raw: &super::Reward, deps: &Deps) -> EventReward {
    dbg!(raw);
    let items = raw.items.clone().unwrap_or_default();
    let credits = raw.credits.unwrap_or(0);
    let counted_items = raw
        .counted_items
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|i| CountedItem {
            count: i.item_count as i32,
            r#type: "".to_owned(),
            key: i.item_type,
        })
        .collect();
    EventReward {
        items,
        counted_items,
        credits,
        as_string: "".to_owned(),
        item_string: "".to_owned(),
        thumbnail: "".to_owned(),
        color: "".to_owned(),
    }
}

#[derive(Debug, Default)]
pub struct Deps {
    time: DateTime<Utc>,
    // sortie_data: (),
    // nodes: HashMap<String, NodeInfo>,
    mission_types: HashMap<String, MissionType>,
    factions: HashMap<String, FactionInfo>,
    languages: HashMap<String, String>,
    regions: HashMap<String, Region>,
}

impl Deps {
    pub fn get_language(&self, key: &str) -> String {
        self.languages.get(key).cloned().unwrap_or_else(|| {
            info!("no translation for {}", key);
            key.to_owned()
        })
    }

    // pub fn get_node_info(&self, key: &str) -> Option<&NodeInfo> {
    //     let n = self.nodes.get(key);
    //     if n.is_none() {
    //         info!("no node info for {}", key);
    //     }
    //     n
    // }

    pub fn get_mission_type(&self, key: &str) -> String {
        if let Some(mt) = self.mission_types.get(key) {
            mt.name.clone()
        } else {
            info!("no mission type for {}", key);
            key.to_owned()
        }
    }

    pub fn get_faction(&self, key: &str) -> String {
        if let Some(f) = self.factions.get(key) {
            f.name.clone()
        } else {
            info!("no faction for {}", key);
            key.to_owned()
        }
    }

    pub fn get_region(&self, key: &str) -> Option<&Region> {
        self.regions.get(key)
    }
}

#[derive(Debug, Deserialize)]
struct FactionInfo {
    /// 派系内部唯一标识
    key: String,
    /// 派系名称
    name: String,
}

#[derive(Debug, Deserialize)]
struct MissionType {
    /// 任务类型内部唯一标识
    key: String,
    /// 任务类型名称
    name: String,
}

#[derive(Debug, Deserialize)]
struct ExportRegions {
    #[serde(rename = "ExportRegions")]
    regions: Vec<Region>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Region {
    #[serde(rename = "uniqueName")]
    unique_name: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "systemIndex")]
    system_index: u8,
    #[serde(rename = "systemName")]
    system_name: String,

    #[serde_as(as = "TryFromInto<u8>")]
    #[serde(rename = "nodeType")]
    node_type: NodeType,
    #[serde(rename = "masteryReq")]
    mastery_req: u8,

    #[serde_as(as = "FromInto<u8>")]
    #[serde(rename = "missionIndex")]
    mission_index: MissionIndex,
    #[serde_as(as = "TryFromInto<u8>")]
    #[serde(rename = "factionIndex")]
    faction_index: FactionIndex,
    #[serde(rename = "minEnemyLevel")]
    min_enemy_level: u32,
    #[serde(rename = "maxEnemyLevel")]
    max_enemy_level: u32,

    #[serde(rename = "isArchwing", default = "Default::default")]
    is_archwing: bool,
}

#[derive(Debug)]
pub enum MissionIndex {
    Assassination,
    Exterminate,
    Survival,
    Rescue,
    Sabotage,
    Capture,
    Spy,
    Defense,
    MobileDefense,
    Interception,
    Hijack,
    HiveSabotage,
    Excavation,
    InfestedSalvage,
    Rathuum,
    Pursuit,
    Rush,
    Assault,
    Defection,
    Landscape,
    TheCircuit,
    Disruption,
    VoidFlood,
    VoidCascade,
    VoidArmageddon,
    Alchemy,
    LegacyteHarvest,
    SharineDefense,
    Faceoff,
    Unknown(u8),
}

impl From<MissionIndex> for u8 {
    fn from(value: MissionIndex) -> Self {
        match value {
            MissionIndex::Assassination => 0,
            MissionIndex::Exterminate => 1,
            MissionIndex::Survival => 2,
            MissionIndex::Rescue => 3,
            MissionIndex::Sabotage => 4,
            MissionIndex::Capture => 5,
            MissionIndex::Spy => 7,
            MissionIndex::Defense => 8,
            MissionIndex::MobileDefense => 9,
            MissionIndex::Interception => 13,
            MissionIndex::Hijack => 14,
            MissionIndex::HiveSabotage => 15,
            MissionIndex::Excavation => 17,
            MissionIndex::InfestedSalvage => 21,
            MissionIndex::Rathuum => 22,
            MissionIndex::Pursuit => 24,
            MissionIndex::Rush => 25,
            MissionIndex::Assault => 26,
            MissionIndex::Defection => 27,
            MissionIndex::Landscape => 28,
            MissionIndex::TheCircuit => 31,
            MissionIndex::Disruption => 33,
            MissionIndex::VoidFlood => 34,
            MissionIndex::VoidCascade => 35,
            MissionIndex::VoidArmageddon => 36,
            MissionIndex::Alchemy => 38,
            MissionIndex::LegacyteHarvest => 40,
            MissionIndex::SharineDefense => 41,
            MissionIndex::Faceoff => 42,
            MissionIndex::Unknown(v) => v,
        }
    }
}

impl From<u8> for MissionIndex {
    fn from(value: u8) -> Self {
        match value {
            0 => MissionIndex::Assassination,
            1 => MissionIndex::Exterminate,
            2 => MissionIndex::Survival,
            3 => MissionIndex::Rescue,
            4 => MissionIndex::Sabotage,
            5 => MissionIndex::Capture,
            7 => MissionIndex::Spy,
            8 => MissionIndex::Defense,
            9 => MissionIndex::MobileDefense,
            13 => MissionIndex::Interception,
            14 => MissionIndex::Hijack,
            15 => MissionIndex::HiveSabotage,
            17 => MissionIndex::Excavation,
            21 => MissionIndex::InfestedSalvage,
            22 => MissionIndex::Rathuum,
            24 => MissionIndex::Pursuit,
            25 => MissionIndex::Rush,
            26 => MissionIndex::Assault,
            27 => MissionIndex::Defection,
            28 => MissionIndex::Landscape,
            31 => MissionIndex::TheCircuit,
            33 => MissionIndex::Disruption,
            34 => MissionIndex::VoidFlood,
            35 => MissionIndex::VoidCascade,
            36 => MissionIndex::VoidArmageddon,
            38 => MissionIndex::Alchemy,
            40 => MissionIndex::LegacyteHarvest,
            41 => MissionIndex::SharineDefense,
            42 => MissionIndex::Faceoff,
            _ => MissionIndex::Unknown(value),
        }
    }
}

impl MissionIndex {
    pub fn key(&self) -> &'static str {
        match self {
            MissionIndex::Assassination => "MT_ASSASSINATION",
            MissionIndex::Exterminate => "MT_EXTERMINATION",
            MissionIndex::Survival => "MT_SURVIVAL",
            MissionIndex::Rescue => "MT_RESCUE",
            MissionIndex::Sabotage => "MT_SABOTAGE",
            MissionIndex::Capture => "MT_CAPTURE",
            MissionIndex::Spy => "MT_INTEL",
            MissionIndex::Defense => "MT_DEFENSE",
            MissionIndex::MobileDefense => "MT_MOBILE_DEFENSE",
            MissionIndex::Interception => "MT_TERRITORY",
            MissionIndex::Hijack => "MT_RETRIEVAL",
            MissionIndex::HiveSabotage => todo!(),
            MissionIndex::Excavation => todo!(),
            MissionIndex::InfestedSalvage => todo!(),
            MissionIndex::Rathuum => "MT_ARENA",
            MissionIndex::Pursuit => todo!(),
            MissionIndex::Rush => todo!(),
            MissionIndex::Assault => todo!(),
            MissionIndex::Defection => todo!(),
            MissionIndex::Landscape => todo!(),
            MissionIndex::TheCircuit => todo!(),
            MissionIndex::Disruption => "MT_ARTIFACT",
            MissionIndex::VoidFlood => todo!(),
            MissionIndex::VoidCascade => "MT_VOID_CASCADE",
            MissionIndex::VoidArmageddon => "MT_ARMAGEDDON",
            MissionIndex::Alchemy => todo!(),
            MissionIndex::LegacyteHarvest => todo!(),
            MissionIndex::SharineDefense => todo!(),
            MissionIndex::Faceoff => todo!(),
            MissionIndex::Unknown(_) => "MT_UNKNOWN",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            MissionIndex::Assassination => "刺杀",
            MissionIndex::Exterminate => "歼灭",
            MissionIndex::Survival => "生存",
            MissionIndex::Rescue => "救援",
            MissionIndex::Sabotage => "破坏",
            MissionIndex::Capture => "捕获",
            MissionIndex::Spy => "间谍",
            MissionIndex::Defense => "防御",
            MissionIndex::MobileDefense => "移动防御",
            MissionIndex::Interception => "拦截",
            MissionIndex::Hijack => "劫持",
            MissionIndex::HiveSabotage => "清巢",
            MissionIndex::Excavation => "挖掘",
            MissionIndex::InfestedSalvage => "异变回收",
            MissionIndex::Rathuum => "竞技场",
            MissionIndex::Pursuit => "追击",
            MissionIndex::Rush => "突袭",
            MissionIndex::Assault => "强袭",
            MissionIndex::Defection => "叛逃",
            MissionIndex::Landscape => "开放地形",
            MissionIndex::TheCircuit => "无尽回廊",
            MissionIndex::Disruption => "干扰",
            MissionIndex::VoidFlood => "虚空洪流",
            MissionIndex::VoidCascade => "虚空覆涌",
            MissionIndex::VoidArmageddon => "虚空决战",
            MissionIndex::Alchemy => "元素转换",
            MissionIndex::LegacyteHarvest => "传承种收割",
            MissionIndex::SharineDefense => "祈运坛防御",
            MissionIndex::Faceoff => "小队对战",
            MissionIndex::Unknown(_) => "未知",
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum NodeType {
    Normal = 0,
    DarkSector = 4,
}

impl From<NodeType> for u8 {
    fn from(value: NodeType) -> Self {
        value as u8
    }
}

#[derive(Debug)]
pub struct ParseNodeTypeError(u8);

impl Display for ParseNodeTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid node type: {}", self.0)
    }
}

impl TryFrom<u8> for NodeType {
    type Error = ParseNodeTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NodeType::Normal),
            4 => Ok(NodeType::DarkSector),
            _ => Err(ParseNodeTypeError(value)),
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum FactionIndex {
    Grineer,
    Corpus,
    Infasted,
    Corrupted,
    Sentient = 5,
    TheMurmur = 7,
    Scaldra,
    Techrot,
    /// only in duviri
    Orokin,
}

impl From<FactionIndex> for u8 {
    fn from(value: FactionIndex) -> Self {
        value as u8
    }
}

pub struct ParseFactionError(u8);

impl Display for ParseFactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid faction: {}", self.0)
    }
}

impl TryFrom<u8> for FactionIndex {
    type Error = ParseFactionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FactionIndex::Grineer),
            1 => Ok(FactionIndex::Corpus),
            2 => Ok(FactionIndex::Infasted),
            3 => Ok(FactionIndex::Corrupted),
            5 => Ok(FactionIndex::Sentient),
            7 => Ok(FactionIndex::TheMurmur),
            8 => Ok(FactionIndex::Scaldra),
            9 => Ok(FactionIndex::Techrot),
            10 => Ok(FactionIndex::Orokin),
            _ => Err(ParseFactionError(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tracing::Level;

    use super::*;

    #[test]
    fn test_parse_raw_world_state() -> eyre::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        let mut file = File::open("./tests/samples/full/ws_wg_org-20250907.json").unwrap();
        let raw = serde_json::from_reader::<_, RawWorldState>(&mut file).unwrap();
        let deps = load_deps("../data", Some(raw.time))?;

        let result = parse_raw_world_state(raw, &deps)?;
        dbg!(&result);
        // assert!(result.is_ok());
        Ok(())
    }
}
