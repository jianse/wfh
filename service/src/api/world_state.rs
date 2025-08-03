// #![allow(dead_code)]

mod helper;

use std::{
    collections::HashMap,
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    AppState,
    api::{BizError, BizResponse, BizResult},
    entities::{prelude::WorldState, world_state},
};
use axum::{Router, extract::State, routing::get};
use chrono::{DateTime, Duration, Utc};
use helper::{BoolFromAny, DeDate, DurationFromStr, NumberFromStringLoose, bool_false};
use hex::ToHex;
use http::HeaderMap;
use log::debug;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::{DisplayFromStr, FromInto};
use tracing::info;

#[derive(Deserialize, Debug)]
struct WorldStateRes {
    /// 状态码 1 成功 0 失败
    #[allow(unused)]
    status: String,
    data: WorldStateData,
}

/// Warframe world state on WeGame
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct WorldStateData {
    timestamp: DateTime<Utc>,
    /// 活动事件
    events: Vec<Event>,
    /// 战术警报
    alerts: Vec<Alert>,
    /// 突击
    sortie: Sortie,
    /// 集团任务
    #[serde(rename = "syndicateMissions")]
    syndicate_missions: Vec<SyndicateMission>,
    /// 虚空裂缝
    fissures: Vec<Fissure>,
    // globalUpgrades flashSales
    /// 入侵
    invasions: Vec<Invasion>,
    // darkSectors
    /// 虚空商人
    #[serde(rename = "voidTrader")]
    void_trader: VoidTrader,
    /// 每日特惠
    #[serde(rename = "dailyDeals")]
    daily_deals: Vec<DailyDeal>,
    /// 结合仪式
    simaris: Simaris,

    /// 武形秘仪 挑战
    #[serde(rename = "conclaveChallenges")]
    conclave_challenges: Vec<ConclaveChallenge>,

    /// unknown
    #[serde(rename = "persistentEnemies")]
    persistent_enemies: Vec<()>,

    /// 地球时间
    #[serde(rename = "earthCycle")]
    earth_cycle: EarthCycle,

    /// 希图斯（夜灵平野）时间
    #[serde(rename = "cetusCycle")]
    cetus_cycle: CetusCycle,

    /// 殁世幽都（魔胎之境）时间
    #[serde(rename = "cambionCycle")]
    cambion_cycle: CambionCycle,

    /// 扎里曼 时间
    #[serde(rename = "zarimanCycle")]
    zariman_cycle: ZarimanCycle,

    // unknown
    #[serde(rename = "weeklyChallenges")]
    weekly_challenges: Vec<()>,
    /// 福尔图纳（奥布山谷） 时间
    #[serde(rename = "vallisCycle")]
    vallis_cycle: VallisCycle,

    /// 午夜电波
    nightwave: Nightwave,

    /// 异常区域
    #[serde(rename = "sentientOutposts")]
    sentient_outposts: SentientOutposts,

    /// 钢铁之路
    #[serde(rename = "steelPath")]
    steel_path: SteelPath,

    /// 圣装重生
    #[serde(rename = "vaultTrader")]
    vault_trader: VaultTrader,

    /// 执刑官猎杀
    #[serde(rename = "archonHunt")]
    archon_hunt: ArchonHunt,

    /// 虚空风暴
    #[serde(rename = "voidStorms")]
    void_storms: Vec<VoidStorm>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Oid {
    #[serde(rename = "$oid")]
    oid: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct VoidStorm {
    #[serde(rename = "_id")]
    id: Oid,
    #[serde(rename = "Node")]
    node: String,

    #[serde_as(deserialize_as = "FromInto<DeDate>")]
    #[serde(rename = "Activation")]
    activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "FromInto<DeDate>")]
    #[serde(rename = "Expiry")]
    expiry: DateTime<Utc>,

    #[serde(rename = "ActiveMissionTier")]
    active_mission_tier: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct ArchonHunt {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,

    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    #[serde(rename = "rewardPool")]
    reward_pool: String,
    variants: Vec<()>,
    missions: Vec<ArchonHuntMission>,
    boss: String,
    faction: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    expired: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct ArchonHuntMission {
    node: String,
    #[serde(rename = "nodeKey")]
    node_key: String,
    r#type: String,
    #[serde(rename = "typeKey")]
    type_key: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    nightmare: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "archwingRequired")]
    archwing_required: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isSharkwing")]
    is_sharkwing: bool,
    #[serde(rename = "advancedSpawners")]
    advanced_spawners: Vec<()>,
    #[serde(rename = "requiredItems")]
    required_items: Vec<()>,
    #[serde(rename = "levelAuras")]
    level_auras: Vec<()>,
}

#[derive(Deserialize, Serialize, Debug)]
struct VaultTraderSchedule {
    expiry: DateTime<Utc>,
    item: String,
}
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct VaultTraderInventory {
    item: String,
    #[serde_as(deserialize_as = "NumberFromStringLoose")]
    ducats: i32,
    #[serde_as(deserialize_as = "NumberFromStringLoose")]
    credits: i32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct VaultTrader {
    id: String,
    activation: DateTime<Utc>,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    character: String,
    location: String,
    inventory: Vec<VaultTraderInventory>,
    #[serde(rename = "psId")]
    ps_id: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "endString")]
    end_string: Duration,
    #[serde(rename = "initialStart")]
    initial_start: DateTime<Utc>,
    schedule: Vec<VaultTraderSchedule>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct SteelPathReward {
    name: String,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    cost: i32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct SteelPath {
    #[serde(rename = "currentReward")]
    current_reward: SteelPathReward,
    activation: DateTime<Utc>,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    remaining: Duration,
    rotation: Vec<SteelPathReward>,
    evergreens: Vec<SteelPathReward>,
    incursions: SteelPathIncursions,
}

#[derive(Deserialize, Serialize, Debug)]
struct SteelPathIncursions {
    id: String,
    activation: DateTime<Utc>,
    expiry: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SentientOutpostsMission {
    node: String,
    faction: String,
    r#type: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct SentientOutposts {
    mission: SentientOutpostsMission,
    activation: DateTime<Utc>,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    id: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Nightwave {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,

    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    season: i32,
    tag: String,
    phase: String,
    params: Vec<()>,
    #[serde(rename = "possibleChallenges")]
    possible_challenges: Vec<()>,
    #[serde(rename = "activeChallenges")]
    active_challenges: Vec<Challenge>,
    #[serde(rename = "rewardTypes")]
    reward_types: Vec<String>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Challenge {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isDaily", default = "bool_false")]
    is_daily: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isElite")]
    is_elite: bool,

    desc: String,
    title: String,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    reputation: u32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct VallisCycle {
    id: String,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isWarm")]
    is_warm: bool,
    state: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    time_left: Duration,
    #[serde(rename = "shortString")]
    short_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct ZarimanCycle {
    id: String,
    #[serde(rename = "bountiesEndDate")]
    bounties_end_date: DateTime<Utc>,
    expiry: DateTime<Utc>,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isCorpus")]
    is_corpus: bool,
    state: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    time_left: Duration,

    #[serde(rename = "shortString")]
    short_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct CambionCycle {
    id: String,
    expiry: DateTime<Utc>,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    time_left: Duration,
    active: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct CetusCycle {
    id: String,
    expiry: DateTime<Utc>,
    activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isDay")]
    is_day: bool,
    state: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    time_left: Duration,

    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isCetus")]
    is_cetus: bool,

    #[serde(rename = "shortString")]
    short_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct EarthCycle {
    id: String,
    expiry: DateTime<Utc>,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isDay")]
    is_day: bool,
    state: String,
    #[serde(rename = "timeLeft")]
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    time_left: Duration,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct ConclaveChallenge {
    id: String,
    expiry: DateTime<Utc>,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    amount: i32,
    mode: String,
    category: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
    #[serde_as(deserialize_as = "BoolFromAny")]
    expired: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    daily: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "rootChallenge")]
    root_challenge: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "endString")]
    end_string: Duration,
    description: String,
    title: String,
    #[serde(rename = "asString")]
    as_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Simaris {
    target: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isTargetActive")]
    is_target_active: bool,
    #[serde(rename = "asString")]
    as_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct DailyDeal {
    item: String,
    expiry: DateTime<Utc>,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "originalPrice")]
    original_price: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "salePrice")]
    sale_price: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    total: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    sold: i32,
    id: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    discount: i32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct VoidTrader {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    character: String,
    location: String,
    inventory: Vec<()>,
    #[serde(rename = "psId")]
    ps_id: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "endString")]
    end_string: Duration,
    #[serde(rename = "initialStart")]
    initial_start: DateTime<Utc>,
    schedule: Vec<()>,
}

#[derive(Deserialize, Serialize, Debug)]
struct InvasionSide {
    reward: EventReward,
    faction: String,
    #[serde(rename = "factionKey")]
    faction_key: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Invasion {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    node: String,
    #[serde(rename = "nodeKey")]
    node_key: String,
    desc: String,
    #[serde(rename = "attackerReward")]
    attacker_reward: EventReward,
    #[serde(rename = "attackingFaction")]
    attacking_faction: String,
    attacker: InvasionSide,
    #[serde(rename = "defenderReward")]
    defender_reward: EventReward,
    #[serde(rename = "defendingFaction")]
    defending_faction: String,
    defender: InvasionSide,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "vsInfestation")]
    vs_infestation: bool,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    count: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "requiredRuns")]
    required_runs: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    completion: f64,
    #[serde_as(deserialize_as = "BoolFromAny")]
    completed: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
    #[serde(rename = "rewardTypes")]
    reward_types: Vec<String>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct SyndicateMissionJob {
    id: String,
    #[serde(rename = "rewardPool")]
    reward_pool: Vec<()>,
    r#type: String,
    #[serde_as(deserialize_as = "Vec<DisplayFromStr>")]
    #[serde(rename = "enemyLevels")]
    enemy_levels: Vec<i32>,
    #[serde(rename = "standingStages")]
    standing_stages: Vec<String>,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "minMR")]
    min_mr: i32,
    expiry: DateTime<Utc>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct SyndicateMission {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,

    syndicate: String,
    #[serde(rename = "syndicateKey")]
    syndicate_key: String,

    nodes: Vec<String>,
    jobs: Vec<SyndicateMissionJob>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
}

#[derive(Deserialize, Serialize, Debug)]
struct SortieVariant {
    #[serde(rename = "missionType")]
    mission_type: String,
    modifier: String,
    #[serde(rename = "modifierDescription")]
    modifier_description: String,
    node: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Sortie {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    #[serde(rename = "rewardPool")]
    reward_pool: String,
    variants: Vec<SortieVariant>,
    missions: Vec<()>,
    boss: String,
    faction: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    expired: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct AlertMission {
    description: String,
    node: String,
    #[serde(rename = "nodeKey")]
    node_key: String,
    r#type: String,
    #[serde(rename = "typeKey")]
    type_key: String,
    faction: String,
    #[serde(rename = "factionKey")]
    faction_key: String,
    reward: EventReward,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "minEnemyLevel")]
    min_enemy_level: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "maxEnemyLevel")]
    max_enemy_level: i32,
    #[serde_as(deserialize_as = "BoolFromAny")]
    nightmare: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "archwingRequired")]
    archwing_required: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isSharkwing")]
    is_sharkwing: bool,
    #[serde(rename = "levelOverride")]
    level_override: String,
    #[serde(rename = "enemySpec")]
    enemy_spec: String,
    #[serde(rename = "advancedSpawners")]
    advanced_spawners: Vec<()>,
    #[serde(rename = "requiredItems")]
    required_items: Vec<()>,
    #[serde(rename = "levelAuras")]
    level_auras: Vec<()>,
}

/// 战术警报
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Alert {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    mission: AlertMission,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
    #[serde(rename = "rewardTypes")]
    reward_types: Vec<String>,
    tag: String,
}

/// 虚空裂缝
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Fissure {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,
    node: String,
    #[serde(rename = "missionType")]
    mission_type: String,
    #[serde(rename = "missionKey")]
    mission_key: String,
    enemy: String,
    #[serde(rename = "enemyKey")]
    enemy_key: String,
    #[serde(rename = "nodeKey")]
    node_key: String,

    tier: String,

    /// this could be a number but it's a string
    /// 1,2,3,4,5,VoidT6
    // #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "tierNum")]
    tier_num: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    expired: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    eta: Duration,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isStorm")]
    is_storm: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isHard")]
    is_hard: bool,
}

/// 活动事件
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Event {
    id: String,
    activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    start_string: Duration,
    expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    active: bool,

    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "maximumScore")]
    maximum_score: i32,

    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "currentScore")]
    current_score: i32,
    #[serde(rename = "smallInterval")]
    small_interval: String,
    #[serde(rename = "largeInterval")]
    large_interval: String,
    description: String,
    tooltip: String,
    node: String,
    #[serde(rename = "concurrentNodes")]
    concurrent_nodes: Vec<()>,
    #[serde(rename = "scoreLocTag")]
    score_loc_tag: Option<String>,
    rewards: Vec<EventReward>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    expired: bool,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(default = "Default::default")]
    health: i32,
    #[serde(rename = "interimSteps")]
    interim_steps: Vec<InterimStep>,
    #[serde(rename = "progressSteps")]
    progress_steps: Vec<()>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isPersonal")]
    is_personal: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isCommunity", default = "bool_false")]
    is_community: bool,
    #[serde(rename = "regionDrops")]
    region_drops: Vec<()>,
    #[serde(rename = "archwingDrops")]
    archwing_drops: Vec<()>,
    metadata: Vec<()>,
    #[serde(rename = "completionBonuses")]
    completion_bonuses: Vec<()>,
    #[serde(rename = "scoreVar")]
    score_var: Option<String>,
    #[serde(rename = "altExpiry")]
    alt_expiry: DateTime<Utc>,
    #[serde(rename = "altActivation")]
    alt_activation: DateTime<Utc>,
    #[serde(rename = "nextAlt")]
    next_alt: NextAlt,
}

#[derive(Serialize, Deserialize, Debug)]
struct NextAlt {
    expiry: DateTime<Utc>,
    activation: DateTime<Utc>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct InterimStep {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    goal: i32,
    reward: EventReward,
    message: Vec<String>,
}

/// 活动奖励 data -> events[*] -> rewards[*]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct EventReward {
    items: Vec<String>,
    #[serde(rename = "countedItems")]
    counted_items: Vec<CountedItem>,

    #[serde_as(deserialize_as = "DisplayFromStr")]
    credits: i32,
    #[serde(rename = "asString")]
    as_string: String,
    #[serde(rename = "itemString")]
    item_string: String,
    thumbnail: String,
    color: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct CountedItem {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    count: i32,
    r#type: String,
    key: String,
}

#[derive(Debug)]
pub struct WorldStateOut {
    pub raw: String,
    pub data: WorldStateData,
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

    let res = serde_json::from_str::<WorldStateRes>(raw.as_str()).map_err(|e| WorldStateError {
        raw: Some(raw.clone()),
        err: e.into(),
    })?;

    Ok(WorldStateOut {
        raw,
        data: res.data,
    })
}

static REWARD_ITEM_MAP: LazyLock<HashMap<&str, Option<&str>>> = LazyLock::new(|| {
    HashMap::from([
        ("Opticor Vandal", Some("奥堤克光子枪破坏者")),
        ("Buried Debts Emblem", Some("雪藏的恩怨徽章")),
        ("Amalgam Shotgun Spazz", Some("并合 霰弹弹幕")),
        ("Amalgam Serration", Some("并合膛线")),
        ("Amalgam Barrel Diffusion", Some("并合弹头扩散")),
        ("Amalgam Organ Shatter", Some("并合肢解")),
        ("Buried Debts Sigil", Some("雪藏的恩怨纹章")),
        ("SolNode236", Some("双衍历程")),
    ])
});

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

fn rewrite_one(val: &mut String, warnings: &mut RewriteWarning, path: String) {
    if let Some(ni) = REWARD_ITEM_MAP.get(val.as_str()) {
        if let Some(ni) = ni {
            *val = ni.to_string();
        }
    } else {
        warnings.add_item(path, format!("unexpected item: {val} "));
    }
}

pub fn rewrite_world_state(data: WorldStateData) -> (WorldStateData, Option<RewriteWarning>) {
    let mut warnings = RewriteWarning::new();
    let mut data = data;
    data.events.iter_mut().enumerate().for_each(|(i, event)| {
        let path = format!("$.events[{i}]");

        rewrite_one(&mut event.node, &mut warnings, format!("{path}.node"));

        event.rewards.iter_mut().enumerate().for_each(|(i, r)| {
            let path = format!("{path}.rewards[{i}]");

            r.items.iter_mut().enumerate().for_each(|(i, it)| {
                let path = format!("{path}.items[{i}]");
                rewrite_one(it, &mut warnings, path);
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

                    rewrite_one(it, &mut warnings, path);
                })
            })
    });
    (data, warnings.into_option())
}

#[axum::debug_handler]
async fn world_state(State(state): State<AppState>) -> BizResult<BizResponse<serde_json::Value>> {
    let r = WorldState::find()
        .filter(world_state::Column::IsSuccess.eq(true))
        .order_by_desc(world_state::Column::SyncTime)
        .one(&state.conn)
        .await?;
    match r {
        Some(m) => {
            let t = m.text.ok_or(BizError::custom("no world state"))?;
            let s = serde_json::from_str::<serde_json::Value>(t.as_str())?;
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
                let s = serde_json::from_reader::<_, WorldStateRes>(f).unwrap();
                dbg!(&s);
            }
            Err(_) => {
                println!("file not found");
            }
        }
    }

    #[test]
    fn test_rewrite() {
        let file = File::open("../ui/cache/worldState_wg.json");
        match file {
            Ok(f) => {
                let s = serde_json::from_reader::<_, WorldStateRes>(f).unwrap();
                let (_res, warn) = rewrite_world_state(s.data);
                // dbg!(&s);
                dbg!(&warn);
            }
            Err(_) => {
                println!("file not found");
            }
        }
    }
}
