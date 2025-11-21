use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::BufReader,
    ops::Deref,
    path::Path,
    sync::LazyLock,
    time::Instant,
};

use chrono::{DateTime, Days, Duration, TimeZone, Utc};
use export::{PE, PublicExports, ReverseMap, get_opt_array, get_str, get_u64};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::{FromInto, TryFromInto, serde_as};
use tokio::sync::{Mutex, RwLock};
use tracing::{info, warn};

use crate::api::{BizResult, raw_world_state::model::RawWorldState};

/// global deps
static G_DEPS: LazyLock<RwLock<Deps>> = LazyLock::new(|| RwLock::new(Deps::default()));

// 全局准入锁：只用于同步 load_deps 的执行
static LOAD_DEPS_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

fn translate_reward(reward: &mut super::model::Reward, deps: &Deps) -> BizResult<()> {
    if let Some(items) = reward.items.as_mut() {
        let names = items.iter().map(|i| deps.find_game_item(i)).collect();
        reward.item_names = Some(names);
    }

    if let Some(counted_items) = reward.counted_items.as_mut() {
        for item in counted_items.iter_mut() {
            let (name, count) = deps.find_game_item_counted(&item.item_type);
            item.item_name = Some(name);
            item.item_count_sum = Some(count * item.item_count);
        }
    }
    Ok(())
}

/// add fields to raw world state.
/// mostly for display
pub async fn fill_raw_world_state(raw: &mut RawWorldState) -> BizResult<()> {
    let instant = Instant::now();

    let deps = G_DEPS.read().await;

    // goals
    for goal in raw.goals.iter_mut() {
        // goal's desc actually is goal's name
        goal.desc = deps.get_language(&goal.desc);

        // goal's node
        if let Some(node) = goal.node.as_mut()
            && let Some(region) = deps.get_region(node)
        {
            *node = region.with_plant();
        };

        // goal's tooltip actually is goal's desc
        if let Some(tooltip) = goal.tooltip.as_mut() {
            *tooltip = deps.get_language(tooltip);
        }
    }

    // alerts
    for alert in raw.alerts.iter_mut() {
        // alert.mission_info.location
        if let Some(region) = deps.get_region(&alert.mission_info.location) {
            alert.mission_info.location = region.with_plant();
        };

        // alert.mission_info.mission_type
        alert.mission_info.mission_type = deps.get_mission_type(&alert.mission_info.mission_type);

        if let Some(desc_text) = alert.mission_info.desc_text.as_mut() {
            *desc_text = deps.get_language(desc_text);
        }
        // translate alert.mission_info.rewards
        translate_reward(&mut alert.mission_info.mission_reward, &deps)?;
    }

    // ActiveMissions
    for mission in raw.active_missions.iter_mut() {
        // mission.node
        if let Some(region) = deps.get_region(&mission.node) {
            mission.node = region.with_plant();
        };

        // mission.mission_type
        mission.mission_type = deps.get_mission_type(&mission.mission_type);
    }

    // void storm
    for void_storm in raw.void_storms.iter_mut() {
        // void_storm.node
        if let Some(region) = deps.get_region(&void_storm.node) {
            void_storm.node = region.with_plant();
            void_storm.mission_type = Some(region.mission_index.name().to_owned())
        };
    }

    // add steel path weekly rewards
    raw.steel_path = Some(from_time(raw.time, &deps));

    // fill daily deals
    for daily_deal in raw.daily_deals.iter_mut() {
        daily_deal.store_item_name = Some(deps.find_game_item(&daily_deal.store_item));
    }

    // TODO: add boss info and reward info
    // lite sortie
    for lite_sortie in raw.lite_sorties.iter_mut() {
        lite_sortie.boss_name = Some(deps.get_sortie_boss_name(&lite_sortie.boss));
        for mission in lite_sortie.missions.iter_mut() {
            // mission.node
            if let Some(region) = deps.get_region(&mission.node) {
                mission.node = region.with_plant();
            };
            mission.mission_type = deps.get_mission_type(&mission.mission_type);
        }
    }

    // TODO: add sentient outpost info

    // sorties
    for sortie in raw.sorties.iter_mut() {
        for variant in sortie.variants.iter_mut() {
            if let Some(region) = deps.get_region(&variant.node) {
                variant.node = region.with_plant();
            };
            variant.mission_type = deps.get_mission_type(&variant.mission_type);

            // TODO: translate modifiers
        }
    }

    // void traders
    for trader in raw.void_traders.iter_mut() {
        // trader.node
        if let Some(region) = deps.get_region(&trader.node) {
            trader.node = region.with_plant();
        };

        // TODO: translate manifest
    }

    // prime traders
    for trader in raw.prime_vault_traders.iter_mut() {
        // trader.node
        if let Some(region) = deps.get_region(&trader.node) {
            trader.node = region.with_plant();
        };

        for item in trader.manifest.iter_mut() {
            item.item_name = Some(deps.find_game_item(&item.item_type));
        }

        for item in trader.evergreen_manifest.iter_mut() {
            item.item_name = Some(deps.find_game_item(&item.item_type));
        }
    }

    // night wave
    for challenge in raw.season_info.active_challenges.iter_mut() {
        // TODO: add more info
        challenge.challenge = deps.find_game_item(&challenge.challenge);
    }

    // invasions
    for invasion in raw.invasions.iter_mut() {
        // invasion.node
        if let Some(region) = deps.get_region(&invasion.node) {
            invasion.node = region.with_plant();
        };

        if let Some(reward) = invasion.attacker_reward.as_mut() {
            translate_reward(reward, &deps)?;
        }
        // if let Some(reward) = invasion.defender_reward.as_mut() {

        translate_reward(&mut invasion.defender_reward, &deps)?;

        invasion.attacker_mission_info.faction =
            deps.get_faction(&invasion.attacker_mission_info.faction);
        invasion.defender_mission_info.faction =
            deps.get_faction(&invasion.defender_mission_info.faction);
        // }

        // TODO: translate modifiers
    }

    info!("fill_raw_world_state took {:?}", instant.elapsed());
    Ok(())
}

pub fn from_time(t: DateTime<Utc>, deps: &Deps) -> super::model::SteelPath {
    let start_time = Utc.with_ymd_and_hms(2020, 11, 16, 0, 0, 0).unwrap();
    let d = t - start_time;
    let sd = d.as_seconds_f64() as u64;
    let remain = sd % (8 * 7 * 24 * 60 * 60);
    let s_over = remain % (7 * 24 * 60 * 60);
    let index = (remain / (7 * 24 * 60 * 60)) as usize;
    let current_reward = deps.steel_path.rotation.get(index).unwrap().clone();

    let activation = t - Duration::seconds(s_over as i64);

    let expiry = activation + Days::new(7);

    let evergreen = deps.steel_path.evergreen.clone();

    super::model::SteelPath {
        activation,
        expiry,
        current_reward,
        evergreen,
    }
}

pub async fn load_deps<P1: AsRef<Path> + Display, P2: AsRef<Path> + Display>(
    dir_path: P1,
    cache_dir: P2,
    client: &Client,
    force_reload: bool,
) -> BizResult<()> {
    {
        let deps = G_DEPS.read().await;
        if !force_reload && deps.is_initialized {
            return Ok(());
        }
    }

    let _guard = LOAD_DEPS_MUTEX.lock().await;

    {
        let deps = G_DEPS.read().await;
        if !force_reload && deps.is_initialized {
            return Ok(());
        }
    };

    let dir = dir_path.as_ref();

    let languages = load_languages(format!("{dir_path}/lang.json"))?;
    let factions = load_factions(format!("{dir_path}/factions.json"))?;
    let mission_types = load_mission_types(format!("{dir_path}/mission_types.json"))?;
    let regions = load_regions(&dir_path)?;
    let steel_path = load_steel_path(format!("{dir_path}/steel_path.json"))?;

    let sortie_bosses = load_sortie_bosses(&dir_path)?;

    let pe = PE::new(client, cache_dir, dir.join("wire.json"), &languages);
    let (pe_data, pe_index) = pe.fetch_transform_all().await?;

    {
        let mut deps = G_DEPS.write().await;
        *deps = Deps {
            regions,
            mission_types,
            factions,
            languages,
            steel_path,
            sortie_bosses,
            pe_index,
            pe_data,
            is_initialized: true,
        };
    }
    Ok(())
}

fn load_sortie_bosses<P: AsRef<Path>>(dir_path: P) -> BizResult<SortieBosses> {
    let file = dir_path.as_ref().join("sortie_bosses.json");
    if file.exists() {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);
        let json: Vec<SortieBoss> = serde_json::from_reader(&mut reader)?;
        let map = json
            .into_iter()
            .map(|k| (k.unique_name.clone(), k))
            .collect();
        Ok(SortieBosses(map))
    } else {
        Ok(SortieBosses::default())
    }
}

fn load_mission_types<P: AsRef<Path>>(file: P) -> BizResult<HashMap<String, MissionType>> {
    // dbg!(std::path::absolute(&file));
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let mut reader = BufReader::new(&mut file);
        let res: Vec<MissionType> = serde_json::from_reader(&mut reader)?;
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
        let mut reader = BufReader::new(&mut file);
        let res: Vec<FactionInfo> = serde_json::from_reader(&mut reader)?;
        let res = res.into_iter().map(|k| (k.key.clone(), k)).collect();
        Ok(res)
    } else {
        info!("no factions.json found, using default");
        Ok(HashMap::new())
    }
}

fn load_regions<P: AsRef<Path>>(dir: P) -> BizResult<HashMap<String, Region>> {
    let mut res = HashMap::new();

    // regions from public export
    let regions_file = dir.as_ref().join("regions.json");
    if fs::exists(&regions_file)? {
        let mut file = File::open(regions_file)?;
        let mut reader = BufReader::new(&mut file);
        let json: ExportRegions = serde_json::from_reader(&mut reader)?;
        let iter = json.regions.into_iter().map(|k| (k.unique_name.clone(), k));
        res.extend(iter);
    } else {
        info!("no regions.json found!");
    }

    // regions that not included in public export
    // we add them manually
    let ex_file = dir.as_ref().join("regions_ex.json");
    if fs::exists(&ex_file)? {
        let mut file = File::open(ex_file)?;
        let mut reader = BufReader::new(&mut file);
        let json: ExportRegions = serde_json::from_reader(&mut reader)?;
        let iter = json.regions.into_iter().map(|k| (k.unique_name.clone(), k));
        res.extend(iter);
    } else {
        info!("no regions_ex.json found!");
    }
    Ok(res)
}
fn load_languages<P: AsRef<Path>>(file: P) -> BizResult<HashMap<String, String>> {
    // dbg!(std::path::absolute(&file));
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let mut reader = BufReader::new(&mut file);
        let res: HashMap<String, String> = serde_json::from_reader(&mut reader)?;
        Ok(res)
    } else {
        info!("no languages.json found, using default");
        Ok(HashMap::new())
    }

    // let mut result = HashMap::new();
}

fn load_steel_path<P: AsRef<Path>>(file: P) -> BizResult<SteelPath> {
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let mut reader = BufReader::new(&mut file);
        let res: SteelPath = serde_json::from_reader(&mut reader)?;
        Ok(res)
    } else {
        info!("no steel_path.json found, using default");
        Ok(SteelPath::default())
    }
}

#[derive(Debug, Deserialize)]
pub struct SortieBoss {
    unique_name: String,
    name: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct SortieBosses(HashMap<String, SortieBoss>);

impl Deref for SortieBosses {
    type Target = HashMap<String, SortieBoss>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct Deps {
    /// Mission types
    mission_types: HashMap<String, MissionType>,

    /// Factions
    factions: HashMap<String, FactionInfo>,

    /// Languages exported from game assets
    languages: HashMap<String, String>,

    /// regions from public export
    regions: HashMap<String, Region>,

    /// steel path rewards manually collected
    steel_path: SteelPath,

    /// reverse map for public exports
    pe_index: ReverseMap,

    /// public exports
    pe_data: PublicExports,

    /// sortie bosses manually collected
    sortie_bosses: SortieBosses,

    /// flag for initialization
    /// used to prevent reloading
    is_initialized: bool,
}

impl Deps {
    /// Get a language translation, return the key if no translation is found
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

    /// Get a region info, return None if no region is found
    pub fn get_region(&self, key: &str) -> Option<&Region> {
        let region = self.regions.get(key);

        if region.is_none() {
            info!("no region for {}", key);
        }
        region
    }

    /// Get a sortie boss name, return the key if no boss is found
    pub fn get_sortie_boss_name(&self, key: &str) -> String {
        if let Some(boss) = self.sortie_bosses.get(key) {
            boss.name.clone()
        } else {
            info!("no sortie boss for {}", key);
            key.to_owned()
        }
    }

    /// Find a game item name by unique name
    pub fn find_game_item(&self, unique_name: &str) -> String {
        let rei = if let Some(idx) = self.pe_index.get(unique_name) {
            Some(idx)
        } else {
            let unique_name = unique_name.replace("/Lotus/StoreItems", "/Lotus");
            self.pe_index.get(&unique_name)
        };

        if let Some(rei) = rei
            && let Some(obj) = self.pe_data.get(&rei.category)
            && let Some(arr) = get_opt_array(obj, &rei.prop).unwrap()
            && let Some(item) = arr.get(rei.index)
        {
            let s = get_str(item, "name").unwrap();
            return s.to_owned();
        }
        warn!(unique_name, "item not found!");

        unique_name.to_owned()
    }

    /// Find a game item name by unique name
    /// Currently only fusion bundles has count property
    pub fn find_game_item_counted(&self, unique_name: &str) -> (String, u32) {
        let rei = if let Some(idx) = self.pe_index.get(unique_name) {
            Some(idx)
        } else {
            let unique_name = unique_name.replace("/Lotus/StoreItems", "/Lotus");
            self.pe_index.get(&unique_name)
        };

        if let Some(rei) = rei
            && let Some(obj) = self.pe_data.get(&rei.category)
            && let Some(arr) = get_opt_array(obj, &rei.prop).unwrap()
            && let Some(item) = arr.get(rei.index)
        {
            let s = get_str(item, "name").unwrap();
            // fusion bundle has a fusionPoints property
            if unique_name.starts_with("/Lotus/Upgrades/Mods/FusionBundles") {
                let count = get_u64(item, "fusionPoints").unwrap();
                (s.to_owned(), count as u32)
            } else {
                (s.to_owned(), 1)
            }
        } else {
            warn!(unique_name, "item not found!");
            (unique_name.to_owned(), 1)
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct SteelPath {
    rotation: Vec<SteelPathStoreItem>,
    evergreen: Vec<SteelPathStoreItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SteelPathStoreItem {
    unique_name: String,
    name: String,
    count: u32,
    cost: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_limit: Option<u32>,
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

#[allow(unused)]
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Region {
    /// region unique name
    #[serde(rename = "uniqueName")]
    unique_name: String,

    /// region display name
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

impl Region {
    pub fn with_plant(&self) -> String {
        format!("{}({})", self.name, self.system_name)
    }
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
    None,
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
            MissionIndex::None => 255,
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
            255 => MissionIndex::None,
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
            MissionIndex::HiveSabotage => "MT_HIVE",
            MissionIndex::Excavation => "MT_EXCAVATE",
            MissionIndex::InfestedSalvage => "MT_PURIFY",
            MissionIndex::Rathuum => "MT_ARENA",
            MissionIndex::Pursuit => todo!(),
            MissionIndex::Rush => "MT_RACE",
            MissionIndex::Assault => "MT_ASSAULT",
            MissionIndex::Defection => "MT_EVACUATION",
            MissionIndex::Landscape => "MT_LANDSCAPE",
            MissionIndex::TheCircuit => todo!(),
            MissionIndex::Disruption => "MT_ARTIFACT",
            MissionIndex::VoidFlood => "MT_CORRUPTION",
            MissionIndex::VoidCascade => "MT_VOID_CASCADE",
            MissionIndex::VoidArmageddon => "MT_ARMAGEDDON",
            MissionIndex::Alchemy => "MT_ALCHEMY",
            MissionIndex::LegacyteHarvest => "MT_ENDLESS_CAPTURE",
            MissionIndex::SharineDefense => todo!(),
            MissionIndex::Faceoff => todo!(),
            MissionIndex::None => todo!(),
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
            MissionIndex::Disruption => "中断",
            MissionIndex::VoidFlood => "虚空洪流",
            MissionIndex::VoidCascade => "虚空覆涌",
            MissionIndex::VoidArmageddon => "虚空决战",
            MissionIndex::Alchemy => "元素转换",
            MissionIndex::LegacyteHarvest => "传承种收割",
            MissionIndex::SharineDefense => "祈运坛防御",
            MissionIndex::Faceoff => "小队对战",
            MissionIndex::None => "中继站",
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
    /// 克隆尼 0
    Grineer,
    /// 科普斯 1
    Corpus,
    /// 感染者 2
    Infasted,
    /// 堕落者 3
    Corrupted,
    /// 灵煞 5
    Sentient = 5,
    /// 低语者 7
    TheMurmur = 7,
    /// 炽蛇军 8
    Scaldra,
    /// 科腐者 9
    Techrot,
    /// 禁卫 10 仅在双衍王境中出现
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

    #[tokio::test]
    async fn test_parse_raw_world_state() -> eyre::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        let mut file = File::open("./tests/samples/full/ws_wg_org-20251115.json").unwrap();
        let mut raw = serde_json::from_reader::<_, RawWorldState>(&mut file).unwrap();
        let client = Client::new();
        load_deps("../data", "../cache", &client, false)
            .await
            .unwrap();
        // let deps = G_DEPS.read().unwrap();
        let result = fill_raw_world_state(&mut raw).await?;
        dbg!(&result);
        // assert!(result.is_ok());
        Ok(())
    }
}
