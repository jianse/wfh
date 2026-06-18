#![allow(unused)]

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use chrono::serde::ts_seconds;
use serde::Deserialize;
use serde::Serialize;
use serde_with::DefaultOnError;
use serde_with::FromInto;
use serde_with::TryFromInto;
use serde_with::serde_as;
use std::collections::HashMap;

use crate::api::raw_world_state::parse::SentientOutpost;
use crate::api::raw_world_state::parse::SteelPathStoreItem;

use super::helper::Date;
use super::helper::Oid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawWorldState {
    /// a string that represents the seed of the world
    #[serde(rename = "WorldSeed")]
    pub world_seed: String,

    /// version
    #[serde(rename = "Version")]
    pub version: i8,

    /// 移动端版本
    #[serde(rename = "MobileVersion")]
    pub mobile_version: String,

    #[serde(rename = "BuildLabel")]
    pub build_label: String,

    #[serde(rename = "Time", deserialize_with = "ts_seconds::deserialize")]
    pub time: DateTime<Utc>,

    /// 公告
    #[serde(rename = "Events")]
    pub events: Vec<Event>,

    /// 活动 例如：热美亚
    #[serde(rename = "Goals")]
    pub goals: Vec<Goal>,

    /// 战术警报 例如：莲的馈赠
    #[serde(rename = "Alerts")]
    pub alerts: Vec<Alert>,

    /// 突击
    #[serde(rename = "Sorties")]
    pub sorties: Vec<Sortie>,

    /// 执刑官猎杀
    #[serde(rename = "LiteSorties")]
    pub lite_sorties: Vec<LiteSortie>,

    /// 集团任务
    #[serde(rename = "SyndicateMissions")]
    pub syndicate_missions: Vec<SyndicateMission>,

    /// 虚空裂缝
    #[serde(rename = "ActiveMissions")]
    pub active_missions: Vec<ActiveMission>,

    /// 全局加成
    #[serde(rename = "GlobalUpgrades")]
    pub global_upgrades: Vec<GlobalUpgrade>,

    /// 特卖
    #[serde(rename = "FlashSales")]
    pub flash_sale: Vec<FlashSale>,

    #[serde(rename = "SkuSales")]
    pub sku_sales: Vec<()>,

    /// 商城
    #[serde(rename = "InGameMarket")]
    pub in_game_market: InGameMarket,

    /// 入侵
    #[serde(rename = "Invasions")]
    pub invasions: Vec<Invasion>,

    #[serde(rename = "HubEvents")]
    pub hub_events: Vec<()>,

    /// 节点状态覆盖
    #[serde(rename = "NodeOverrides")]
    pub node_overrides: Vec<NodeOverride>,

    /// 虚空商人
    #[serde(rename = "VoidTraders")]
    pub void_traders: Vec<VoidTrader>,

    /// 圣装重生
    #[serde(rename = "PrimeVaultTraders")]
    pub prime_vault_traders: Vec<PrimeVaultTrader>,

    /// 虚空风暴
    #[serde(rename = "VoidStorms")]
    pub void_storms: Vec<VoidStorm>,

    #[serde(rename = "PrimeAccessAvailability")]
    pub prime_access_availability: PrimeAccessAvailability,

    #[serde(rename = "PrimeVaultAvailabilities")]
    pub prime_vault_availability: Vec<bool>,

    #[serde(rename = "PrimeTokenAvailability")]
    pub prime_token_availability: bool,

    /// 日常特惠
    #[serde(rename = "DailyDeals")]
    pub daily_deals: Vec<DailyDeal>,

    /// 每日结合目标
    #[serde(rename = "LibraryInfo")]
    pub library_info: LibraryInfo,

    /// pvp 任务
    #[serde(rename = "PVPChallengeInstances")]
    pub pvp_challenge_instances: Vec<PvpChallengeInstance>,

    #[serde(rename = "PersistentEnemies")]
    pub persistent_enemies: Vec<()>,

    #[serde(rename = "PVPAlternativeModes")]
    pub pvp_alternative_modes: Vec<()>,

    #[serde(rename = "PVPActiveTournaments")]
    pub pvp_active_tournaments: Vec<()>,

    /// 战舰建造进度
    #[serde(rename = "ProjectPct")]
    pub project_pct: Vec<f64>,

    #[serde(rename = "ConstructionProjects")]
    pub construction_projects: Vec<ConstructionProject>,

    #[serde(rename = "TwitchPromos")]
    pub twitch_promos: Vec<()>,

    #[serde(rename = "ExperimentRecommended")]
    pub experiment_recommended: Vec<()>,

    // /// 无尽回廊
    // #[serde(rename = "EndlessXpChoices")]
    // // pub endless_xp_choices: Vec<EndlessXpChoice>,
    /// 无尽回廊 added at version 42.0.0
    #[serde(rename = "EndlessXpSchedule")]
    pub endless_xp_schedule: Vec<EndlessXpSchedule>,

    #[serde(rename = "ForceLogoutVersion")]
    pub force_logout_version: u64,

    /// 特色道场
    #[serde(rename = "FeaturedGuilds", skip_serializing_if = "Option::is_none")]
    pub featured_guilds: Option<Vec<FeaturedGuild>>,

    /// 午夜电波
    #[serde(rename = "SeasonInfo")]
    pub season_info: SeasonInfo,

    /// 1999日历
    #[serde(rename = "KnownCalendarSeasons")]
    pub known_calendar_seasons: Vec<KnownCalendarSeason>,

    /// 科研
    #[serde(rename = "Conquests")]
    pub conquests: Vec<Conquest>,

    #[serde(rename = "Tmp")]
    pub tmp: String,

    /// this field is not part of raw api
    /// but added by me
    #[serde(rename = "SteelPath", skip_serializing_if = "Option::is_none")]
    pub steel_path: Option<SteelPath>,

    /// sentient outpost, add by me
    #[serde(
        rename = "SentientOutpost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sentient_outpost: Option<SentientOutpost>,

    /// 沉沦之地
    #[serde(rename = "Descents")]
    pub descents: Vec<Descent>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndlessXpSchedule {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "CategoryChoices")]
    pub category_choices: Vec<EndlessXpChoice>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Descent {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "RandSeed")]
    pub rand_seed: u64,

    #[serde(rename = "Challenges")]
    pub challenges: Vec<DescentChallenge>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DescentChallenge {
    #[serde(rename = "Index")]
    pub index: u8,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Challenge")]
    pub challenge: String,

    #[serde(rename = "Level")]
    pub level: String,

    #[serde(rename = "Specs")]
    pub specs: Vec<String>,

    #[serde(rename = "Auras")]
    pub auras: Vec<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SteelPath {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    pub current_reward: SteelPathStoreItem,

    pub evergreen: Vec<SteelPathStoreItem>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Conquest {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Missions")]
    pub missions: Vec<ConquestMission>,

    #[serde(rename = "Variables")]
    pub variables: Vec<String>,

    /// added by me
    #[serde(rename = "TVariables", skip_serializing_if = "Option::is_none")]
    pub t_variables: Option<Vec<TVariable>>,

    #[serde(rename = "RandomSeed")]
    pub random_seed: u64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TVariable {
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConquestMission {
    pub faction: String,
    #[serde(rename = "missionType")]
    pub mission_type: String,
    pub difficulties: Vec<Difficulty>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Difficulty {
    #[serde(rename = "type")]
    pub r#type: String,
    pub deviation: String,

    /// added by me
    #[serde(rename = "deviationDesc", skip_serializing_if = "Option::is_none")]
    pub deviation_desc: Option<String>,

    pub risks: Vec<String>,

    /// added by me
    #[serde(
        rename = "tRisks",
        skip_serializing_if = "Option::is_none",
        default = "Option::default"
    )]
    pub t_risks: Option<Vec<TRisk>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TRisk {
    pub name: String,
    pub desc: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KnownCalendarSeason {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Days")]
    pub days: Vec<Day>,

    #[serde(rename = "Season")]
    pub season: String,

    #[serde(rename = "YearIteration")]
    pub year_iteration: u16,

    #[serde(rename = "Version")]
    pub version: u16,

    #[serde(rename = "UpgradeAvaliabilityRequirements")]
    pub upgrade_availability_requirements: Vec<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Day {
    pub day: u16,
    pub events: Vec<CalendarEvent>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum CalendarEvent {
    #[serde(rename = "CET_CHALLENGE")]
    Challenge {
        challenge: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        desc: Option<String>,
    },
    #[serde(rename = "CET_REWARD")]
    Reward {
        reward: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "CET_UPGRADE")]
    Upgrade {
        upgrade: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        desc: Option<String>,
    },
    #[serde(rename = "CET_BIRTHDAY")]
    Birthday { title: String, desc: String },
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SeasonInfo {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "AffiliationTag")]
    pub affiliation_tag: String,

    #[serde(rename = "Season")]
    pub season: u8,

    #[serde(rename = "Phase")]
    pub phase: u8,

    #[serde(rename = "Params")]
    pub params: String,

    #[serde(rename = "ActiveChallenges")]
    pub active_challenges: Vec<Challenge>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Challenge {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "Daily", skip_serializing_if = "Option::is_none")]
    pub daily: Option<bool>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Challenge")]
    pub challenge: String,

    /// challenge name, added by me
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// challenge description, added by me
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// challenge xp, added by me
    #[serde(rename = "Xp", skip_serializing_if = "Option::is_none")]
    pub xp: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FeaturedGuild {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Tier")]
    pub tier: u8,
    #[serde_as(deserialize_as = "Option<FromInto<Oid>>")]
    #[serde(rename = "AllianceId", skip_serializing_if = "Option::is_none")]
    pub alliance_id: Option<String>,

    #[serde(rename = "Icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<u8>,

    #[serde(rename = "Emblem", skip_serializing_if = "Option::is_none")]
    pub emblem: Option<bool>,

    #[serde(rename = "HiddenPlatforms", skip_serializing_if = "Option::is_none")]
    pub hidden_platforms: Option<HashMap<String, bool>>,

    #[serde(rename = "IconOverride", skip_serializing_if = "Option::is_none")]
    pub icon_override: Option<u8>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndlessXpChoice {
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Choices")]
    pub choices: Vec<String>,

    #[serde(rename = "ChoicesEx", skip_serializing_if = "Option::is_none")]
    pub choices_ex: Option<Vec<String>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConstructionProject {
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "Node")]
    pub node: String,
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde(rename = "Tasks")]
    pub tasks: Vec<String>,
    #[serde(rename = "ToolTips")]
    pub tooltips: Vec<String>,
    #[serde(rename = "HunterBoostTask")]
    pub hunter_boost_task: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PvpChallengeInstance {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "challengeTypeRefID", skip_serializing_if = "Option::is_none")]
    pub challenge_type_ref_id: Option<String>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "startDate")]
    pub state_date: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,

    pub params: Vec<Param>,

    #[serde(rename = "isGenerated")]
    pub is_generated: bool,

    #[serde(rename = "PVPMode")]
    pub pvp_mode: String,

    #[serde_as(deserialize_as = "Vec<FromInto<Oid>>")]
    #[serde(rename = "subChallenges")]
    pub sub_challenges: Vec<String>,
    #[serde(rename = "Category")]
    pub category: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Param {
    pub n: String,
    pub v: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LibraryInfo {
    #[serde(rename = "LastCompletedTargetType")]
    pub last_completed_target_type: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DailyDeal {
    #[serde(rename = "StoreItem")]
    pub store_item: String,

    /// added by me
    #[serde(rename = "StoreItemName", skip_serializing_if = "Option::is_none")]
    pub store_item_name: Option<String>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Discount")]
    pub discount: f64,

    #[serde(rename = "OriginalPrice")]
    pub original_price: u32,

    #[serde(rename = "SalePrice")]
    pub sale_price: u32,

    #[serde(rename = "AmountTotal")]
    pub amount_total: u32,

    #[serde(rename = "AmountSold")]
    pub amount_sold: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrimeAccessAvailability {
    #[serde(rename = "State")]
    pub state: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VoidStorm {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "Node")]
    pub node: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "ActiveMissionTier")]
    pub active_mission_tier: String,

    #[serde(
        rename = "MissionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mission_type: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrimeVaultTrader {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "InitialStartDate")]
    pub initial_start_date: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Node")]
    pub node: String,

    #[serde(rename = "Manifest")]
    pub manifest: Vec<ManifestItem>,

    #[serde(rename = "EvergreenManifest")]
    pub evergreen_manifest: Vec<ManifestItem>,

    #[serde(rename = "ScheduleInfo")]
    pub schedule_info: Vec<ScheduleInfo>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScheduleInfo {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "PreviewHiddenUntil", skip_serializing_if = "Option::is_none")]
    pub preview_hidden_until: Option<DateTime<Utc>>,

    #[serde(rename = "FeaturedItem", skip_serializing_if = "Option::is_none")]
    pub featured_item: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ManifestItem {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    /// added by me
    #[serde(rename = "ItemName", skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,

    #[serde(rename = "PrimePrice", skip_serializing_if = "Option::is_none")]
    pub prime_price: Option<u32>,

    #[serde(rename = "RegularPrice", skip_serializing_if = "Option::is_none")]
    pub regular_price: Option<u32>,

    #[serde(rename = "Limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VoidTrader {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    /// 虚空商人出现的时间
    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<DateTime<Utc>>,

    /// 虚空商人离开的时间
    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<DateTime<Utc>>,

    /// "Baro'Ki Teel", "EvilBaroWeek4"
    #[serde(rename = "Character")]
    pub character: String,

    /// 虚空商人出现的节点，一般是中继站
    #[serde(rename = "Node")]
    pub node: String,

    /// 虚空商人带的商品
    #[serde(rename = "Manifest", skip_serializing_if = "Option::is_none")]
    pub manifest: Option<Vec<ManifestItem>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeOverride {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "Node")]
    pub node: String,
    #[serde(rename = "Hide", skip_serializing_if = "Option::is_none")]
    pub hide: Option<bool>,
    #[serde(rename = "Seed", skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    #[serde(rename = "LevelOverride", skip_serializing_if = "Option::is_none")]
    pub level_override: Option<String>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<DateTime<Utc>>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<DateTime<Utc>>,

    #[serde(
        rename = "CustomNpcEncounters",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_npc_encounters: Option<Vec<String>>,

    #[serde(rename = "Faction", skip_serializing_if = "Option::is_none")]
    pub faction: Option<String>,

    #[serde(rename = "EnemySpec", skip_serializing_if = "Option::is_none")]
    pub enemy_spec: Option<String>,

    #[serde(rename = "ExtraEnemySpec", skip_serializing_if = "Option::is_none")]
    pub extra_enemy_spec: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Invasion {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "DefenderFaction")]
    pub defender_faction: String,

    #[serde(rename = "Node")]
    pub node: String,

    #[serde(rename = "Count")]
    pub count: i32,

    #[serde(rename = "Goal")]
    pub goal: i32,

    #[serde(rename = "LocTag")]
    pub loc_tag: String,

    #[serde(rename = "Completed")]
    pub completed: bool,

    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "ChainID")]
    pub chain_id: String,

    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(rename = "AttackerReward", skip_serializing_if = "Option::is_none")]
    pub attacker_reward: Option<Reward>,

    #[serde(rename = "AttackerMissionInfo")]
    pub attacker_mission_info: InvasionMissionInfo,

    #[serde(rename = "DefenderReward")]
    pub defender_reward: Reward,

    #[serde(rename = "DefenderMissionInfo")]
    pub defender_mission_info: InvasionMissionInfo,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InvasionMissionInfo {
    pub seed: i32,
    pub faction: String,
    #[serde(rename = "missionReward", skip_serializing_if = "Option::is_none")]
    pub mission_reward: Option<Vec<()>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InGameMarket {
    #[serde(rename = "LandingPage")]
    pub landing_page: LandingPage,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LandingPage {
    #[serde(rename = "Categories")]
    pub categories: Vec<Category>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    #[serde(rename = "CategoryName")]
    pub category_name: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Icon")]
    pub icon: String,

    #[serde(rename = "Items")]
    pub items: Vec<String>,

    #[serde(rename = "AddToMenu", skip_serializing_if = "Option::is_none")]
    pub add_to_menu: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlashSale {
    // required
    #[serde(rename = "TypeName")]
    pub type_name: String,

    #[serde(rename = "ShowInMarket", skip_serializing_if = "Option::is_none")]
    pub show_in_market: Option<bool>,

    #[serde(rename = "HideFromMarket", skip_serializing_if = "Option::is_none")]
    pub hide_from_market: Option<bool>,

    #[serde(rename = "SupporterPack", skip_serializing_if = "Option::is_none")]
    pub supporter_pack: Option<bool>,

    #[serde(rename = "Discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,

    /// Buy one get one
    #[serde(rename = "BogoBuy", skip_serializing_if = "Option::is_none")]
    pub bogo_buy: Option<f64>,

    #[serde(rename = "BogoGet", skip_serializing_if = "Option::is_none")]
    pub bogo_get: Option<f64>,

    #[serde(rename = "UrlOverride", skip_serializing_if = "Option::is_none")]
    pub url_override: Option<String>,

    #[serde(rename = "PremiumOverride", skip_serializing_if = "Option::is_none")]
    pub premium_override: Option<f64>,

    #[serde(rename = "RegularOverride", skip_serializing_if = "Option::is_none")]
    pub regular_override: Option<f64>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "StartDate")]
    pub start_date: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "EndDate")]
    pub end_date: DateTime<Utc>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(
        rename = "ProductExpiryOverride",
        skip_serializing_if = "Option::is_none"
    )]
    pub product_expiry_override: Option<DateTime<Utc>>,

    #[serde(rename = "DailySaleGenerated", skip_serializing_if = "Option::is_none")]
    pub daily_sale_generated: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GlobalUpgrade {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "ExpiryDate")]
    pub expiry_date: DateTime<Utc>,

    #[serde(rename = "UpgradeType")]
    pub upgrade_type: String,
    #[serde(rename = "OperationType")]
    pub operation_type: String,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "LocalizeTag", skip_serializing_if = "Option::is_none")]
    pub localize_tag: Option<String>,
    #[serde(rename = "LocalizeDescTag", skip_serializing_if = "Option::is_none")]
    pub localize_desc_tag: Option<String>,

    #[serde(rename = "Nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,

    #[serde(rename = "ValidType", skip_serializing_if = "Option::is_none")]
    pub valid_type: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActiveMission {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Region")]
    pub region: u16,

    #[serde(rename = "Seed")]
    pub seed: u32,
    #[serde(rename = "Node")]
    pub node: String,

    #[serde(rename = "MissionType")]
    pub mission_type: String,
    #[serde(rename = "Modifier")]
    pub modifier: String,

    #[serde(rename = "Hard", skip_serializing_if = "Option::is_none")]
    pub hard: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyndicateMission {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "Seed")]
    pub seed: u32,
    #[serde(rename = "Nodes")]
    pub nodes: Vec<String>,

    #[serde(rename = "Jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job {
    #[serde(rename = "jobType", skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    // #[serde(rename = "rewards")]
    pub rewards: String,
    #[serde(rename = "masteryReq", skip_serializing_if = "Option::is_none")]
    pub mastery_req: Option<u8>,
    #[serde(rename = "minEnemyLevel")]
    pub min_enemy_level: u16,
    #[serde(rename = "maxEnemyLevel")]
    pub max_enemy_level: u16,
    #[serde(rename = "xpAmounts")]
    pub xp_amounts: Vec<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endless: Option<bool>,

    #[serde(rename = "isVault", skip_serializing_if = "Option::is_none")]
    pub is_vault: Option<bool>,

    #[serde(rename = "locationTag", skip_serializing_if = "Option::is_none")]
    pub location_tag: Option<String>,

    #[serde(rename = "requiredItems", skip_serializing_if = "Option::is_none")]
    pub required_items: Option<Vec<String>>,

    #[serde(
        rename = "useRequiredItemsAsMiscItemFee",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_required_items_as_misc_item_fee: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LiteSortie {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Reward")]
    pub reward: String,
    #[serde(rename = "Seed")]
    pub seed: u32,
    #[serde(rename = "Boss")]
    pub boss: String,

    /// added by me
    #[serde(rename = "BossName", skip_serializing_if = "Option::is_none")]
    pub boss_name: Option<String>,

    #[serde(rename = "Missions")]
    pub missions: Vec<Mission>,
}
#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mission {
    #[serde(rename = "missionType")]
    pub mission_type: String,
    pub node: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sortie {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Reward")]
    pub reward: String,

    #[serde(rename = "Seed")]
    pub seed: u32,
    #[serde(rename = "Boss")]
    pub boss: String,
    #[serde(rename = "ExtraDrops")]
    pub extra_drops: Vec<String>,
    #[serde(rename = "Variants")]
    pub variants: Vec<Variant>,

    #[serde(rename = "Twitter", skip_serializing_if = "Option::is_none")]
    pub twitter: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Variant {
    #[serde(rename = "missionType")]
    pub mission_type: String,
    #[serde(rename = "modifierType")]
    pub modifier_type: String,

    #[serde(rename = "modifierName", skip_serializing_if = "Option::is_none")]
    pub modifier_name: Option<String>,

    #[serde(
        rename = "modifierDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub modifier_description: Option<String>,

    pub node: String,

    pub tileset: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Alert {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "MissionInfo")]
    pub mission_info: MissionInfo,

    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "ForceUnlock", skip_serializing_if = "Option::is_none")]
    pub force_unlock: Option<bool>,

    #[serde(rename = "Icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MissionInfo {
    pub location: String,

    #[serde(rename = "missionType")]
    pub mission_type: String,

    pub faction: String,

    pub difficulty: f32,

    #[serde(rename = "missionReward")]
    pub mission_reward: Reward,

    #[serde(rename = "levelOverride")]
    pub level_override: String,

    #[serde(rename = "minEnemyLevel")]
    pub min_enemy_level: i32,

    #[serde(rename = "maxEnemyLevel")]
    pub max_enemy_level: i32,

    #[serde(rename = "enemySpec")]
    pub enemy_spec: String,

    #[serde(rename = "descText", skip_serializing_if = "Option::is_none")]
    pub desc_text: Option<String>,

    /// 曲翼任务
    #[serde(rename = "archwingRequired", skip_serializing_if = "Option::is_none")]
    pub archwing_required: Option<bool>,

    #[serde(rename = "vipAgent", skip_serializing_if = "Option::is_none")]
    pub vip_agent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nightmare: Option<bool>,

    #[serde(rename = "isSharkwingMission", skip_serializing_if = "Option::is_none")]
    pub is_sharkwing_mission: Option<bool>,

    #[serde(rename = "advancedSpawners", skip_serializing_if = "Option::is_none")]
    pub advanced_spawners: Option<Vec<String>>,

    #[serde(rename = "requiredItems", skip_serializing_if = "Option::is_none")]
    pub required_items: Option<Vec<String>>,

    #[serde(
        rename = "consumeRequiredItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_required_items: Option<bool>,

    #[serde(rename = "goalTag", skip_serializing_if = "Option::is_none")]
    pub goal_tag: Option<String>,

    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(
        rename = "leadersAlwaysAllowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub leaders_always_allowed: Option<bool>,

    #[serde(rename = "levelAuras", skip_serializing_if = "Option::is_none")]
    pub level_auras: Option<Vec<String>>,

    #[serde(rename = "extraEnemySpec", skip_serializing_if = "Option::is_none")]
    pub extra_enemy_spec: Option<String>,

    #[serde(rename = "maxWaveNum", skip_serializing_if = "Option::is_none")]
    pub max_wave_num: Option<u32>,

    /// 前置任务需求
    #[serde(rename = "questReq", skip_serializing_if = "Option::is_none")]
    pub quest_req: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Goal {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Node", skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    #[serde(rename = "ScoreVar", skip_serializing_if = "Option::is_none")]
    pub score_var: Option<String>,

    #[serde(rename = "ScoreLocTag", skip_serializing_if = "Option::is_none")]
    pub score_loc_tag: Option<String>,

    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    #[serde(rename = "HealthPct", skip_serializing_if = "Option::is_none")]
    pub health_pct: Option<f64>,

    #[serde(rename = "Regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<u32>>,

    #[serde(rename = "Desc")]
    pub desc: String,

    #[serde(rename = "ToolTip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,

    #[serde(rename = "OptionalInMission", skip_serializing_if = "Option::is_none")]
    pub optional_in_mission: Option<bool>,

    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde_as(deserialize_as = "Option<Vec<FromInto<Oid>>>")]
    #[serde(rename = "UpgradeIds", skip_serializing_if = "Option::is_none")]
    pub upgrade_ids: Option<Vec<String>>,

    #[serde(rename = "Personal", skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,

    #[serde(rename = "Community", skip_serializing_if = "Option::is_none")]
    pub community: Option<bool>,

    #[serde(rename = "Goal", skip_serializing_if = "Option::is_none")]
    pub goal: Option<u32>,

    #[serde(rename = "Reward", skip_serializing_if = "Option::is_none")]
    pub reward: Option<Reward>,

    #[serde(rename = "InterimGoals", skip_serializing_if = "Option::is_none")]
    pub interim_goals: Option<Vec<u32>>,

    #[serde(rename = "InterimRewards", skip_serializing_if = "Option::is_none")]
    pub interim_rewards: Option<Vec<Reward>>,

    #[serde(rename = "BonusGoal", skip_serializing_if = "Option::is_none")]
    pub bonus_goal: Option<u32>,

    #[serde(rename = "BonusReward", skip_serializing_if = "Option::is_none")]
    pub bonus_reward: Option<Reward>,

    #[serde(rename = "Bounty", skip_serializing_if = "Option::is_none")]
    pub bounty: Option<bool>,

    #[serde(rename = "ClampNodeScores", skip_serializing_if = "Option::is_none")]
    pub clamp_node_scores: Option<bool>,

    #[serde(
        rename = "ConcurrentMissionKeyNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub concurrent_mission_key_names: Option<Vec<String>>,

    #[serde(rename = "ConcurrentNodeReqs", skip_serializing_if = "Option::is_none")]
    pub concurrent_node_reqs: Option<Vec<u32>>,

    #[serde(rename = "ConcurrentNodes", skip_serializing_if = "Option::is_none")]
    pub concurrent_nodes: Option<Vec<String>>,

    #[serde(rename = "Faction", skip_serializing_if = "Option::is_none")]
    pub faction: Option<String>,

    #[serde(rename = "Icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(rename = "MissionKeyName", skip_serializing_if = "Option::is_none")]
    pub mission_key_name: Option<String>,

    #[serde(rename = "NightLevel", skip_serializing_if = "Option::is_none")]
    pub night_level: Option<String>,

    #[serde(rename = "Success", skip_serializing_if = "Option::is_none")]
    pub success: Option<u32>,

    #[serde(rename = "JobAffiliationTag", skip_serializing_if = "Option::is_none")]
    pub job_affiliation_tag: Option<String>,

    #[serde_as(deserialize_as = "Option<FromInto<Oid>>")]
    #[serde(rename = "JobCurrentVersion", skip_serializing_if = "Option::is_none")]
    pub job_current_version: Option<String>,

    #[serde_as(deserialize_as = "Option<FromInto<Oid>>")]
    #[serde(rename = "JobPreviousVersion", skip_serializing_if = "Option::is_none")]
    pub job_previous_version: Option<String>,

    #[serde(rename = "Jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,

    #[serde(rename = "PreviousJobs", skip_serializing_if = "Option::is_none")]
    pub previous_jobs: Option<Vec<Job>>,

    #[serde(rename = "VictimNode", skip_serializing_if = "Option::is_none")]
    pub victim_node: Option<String>,

    #[serde(rename = "ArchwingDrops", skip_serializing_if = "Option::is_none")]
    pub archwing_drops: Option<Vec<String>>,

    #[serde(rename = "Best", skip_serializing_if = "Option::is_none")]
    pub best: Option<bool>,

    #[serde(rename = "Fomorian", skip_serializing_if = "Option::is_none")]
    pub fomorian: Option<bool>,

    #[serde(rename = "MissionInfo", skip_serializing_if = "Option::is_none")]
    pub mission_info: Option<MissionInfo>,

    #[serde(rename = "ScoreMaxTag", skip_serializing_if = "Option::is_none")]
    pub score_max_tag: Option<String>,

    #[serde(
        rename = "ScoreTagBlocksGuildTierChanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub score_tag_blocks_guild_tier_changes: Option<bool>,

    #[serde(rename = "RegionDrops", skip_serializing_if = "Option::is_none")]
    pub region_drops: Option<Vec<String>>,

    #[serde(rename = "ContinuousHubEvent", skip_serializing_if = "Option::is_none")]
    pub continuous_hub_event: Option<ContinuousHubEvent>,

    #[serde(rename = "InstructionalItem", skip_serializing_if = "Option::is_none")]
    pub instructional_item: Option<String>,

    #[serde(rename = "RegionIdx", skip_serializing_if = "Option::is_none")]
    pub region_idx: Option<u32>,

    #[serde(rename = "Transmission", skip_serializing_if = "Option::is_none")]
    pub transmission: Option<String>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "GracePeriod", skip_serializing_if = "Option::is_none")]
    pub grace_period: Option<DateTime<Utc>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContinuousHubEvent {
    #[serde(rename = "Transmissions")]
    pub transmissions: Vec<String>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "RepeatInterval", skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<u32>,
}

// #[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reward {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xp: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,

    /// added by me
    #[serde(rename = "itemNames", skip_serializing_if = "Option::is_none")]
    pub item_names: Option<Vec<String>>,

    #[serde(rename = "countedItems", skip_serializing_if = "Option::is_none")]
    pub counted_items: Option<Vec<CountedItem>>,

    #[serde(rename = "randomizedItems", skip_serializing_if = "Option::is_none")]
    pub randomized_items: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CountedItem {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    /// added by me
    #[serde(rename = "ItemName", skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,

    #[serde(rename = "ItemCount")]
    pub item_count: u32,

    /// added by me
    #[serde(rename = "ItemCountSum", skip_serializing_if = "Option::is_none")]
    pub item_count_sum: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "Messages")]
    pub message: Vec<Message>,

    #[serde(rename = "Prop")]
    pub prop: String,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Date", skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "EventStartDate", skip_serializing_if = "Option::is_none")]
    pub event_start_date: Option<DateTime<Utc>>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "EventEndDate", skip_serializing_if = "Option::is_none")]
    pub event_end_date: Option<DateTime<Utc>>,

    #[serde(rename = "Priority", default = "bool::default")]
    pub priority: bool,

    #[serde(rename = "MobileOnly")]
    pub mobile_only: bool,

    #[serde(rename = "Community", skip_serializing_if = "Option::is_none")]
    pub community: Option<bool>,

    /// icon name
    #[serde(rename = "Icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(rename = "ImageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    #[serde(
        rename = "HideEndDateModifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_end_date_modifier: Option<bool>,

    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,

    #[serde(rename = "EventLiveUrl", skip_serializing_if = "Option::is_none")]
    pub event_live_url: Option<String>,

    #[serde(rename = "GameOnly", skip_serializing_if = "Option::is_none")]
    pub game_only: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    #[serde(rename = "LanguageCode")]
    pub language_code: String,

    #[serde(rename = "Link")]
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    #[serde(rename = "LanguageCode")]
    pub language_code: String,

    #[serde(rename = "Message")]
    pub message: String,
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeSet;
    use std::io::Seek;
    use std::ptr::read;

    use super::*;

    #[test]
    fn test_decode_raw() -> eyre::Result<()> {
        for entry in std::fs::read_dir("./tests/samples/full")? {
            let entry = entry?;
            let meta = entry.metadata()?;
            // let file = entry.file();
            if !meta.is_file() {
                continue;
            }
            dbg!(entry.path());
            let file = std::fs::File::open(entry.path())?;
            let mut reader = std::io::BufReader::new(file);

            let mut de = serde_json::Deserializer::from_reader(&mut reader);

            let mut ignored_set = BTreeSet::new();
            let res: Result<RawWorldState, _> = serde_ignored::deserialize(&mut de, |path| {
                ignored_set.insert(path.to_string());
            });

            let raw = match res {
                Ok(raw) => raw,
                Err(_) => {
                    reader.seek(std::io::SeekFrom::Start(0))?;
                    let mut de = serde_json::Deserializer::from_reader(&mut reader);
                    serde_path_to_error::deserialize(&mut de)?
                }
            };

            dbg!(ignored_set);
            dbg!(&raw.known_calendar_seasons);
        }
        Ok(())
    }
}
