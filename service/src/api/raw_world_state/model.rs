#![allow(unused)]

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{DefaultOnError, FromInto, TryFromInto, serde_as};
use std::collections::HashMap;

use super::helper::{Date, Oid};

#[derive(Debug, Deserialize, Clone)]
pub struct RawWorldState {
    #[serde(rename = "WorldSeed")]
    pub world_seed: String,

    #[serde(rename = "Version")]
    pub version: i8,

    #[serde(rename = "MobileVersion")]
    pub mobile_version: String,

    #[serde(rename = "BuildLabel")]
    pub build_label: String,

    #[serde(rename = "Time", deserialize_with = "ts_seconds::deserialize")]
    pub time: DateTime<Utc>,

    /// 公告
    #[serde(rename = "Events")]
    pub events: Vec<Event>,

    #[serde(rename = "Goals")]
    pub goals: Vec<Goal>,

    #[serde(rename = "Alerts")]
    pub alerts: Vec<Alert>,

    #[serde(rename = "Sorties")]
    pub sorties: Vec<Sortie>,

    #[serde(rename = "LiteSorties")]
    pub lite_sorties: Vec<LiteSortie>,

    #[serde(rename = "SyndicateMissions")]
    pub syndicate_missions: Vec<SyndicateMission>,

    #[serde(rename = "ActiveMissions")]
    pub active_missions: Vec<ActiveMission>,

    #[serde(rename = "GlobalUpgrades")]
    pub global_upgrades: Vec<GlobalUpgrade>,

    #[serde(rename = "FlashSales")]
    pub flash_sale: Vec<FlashSale>,

    #[serde(rename = "InGameMarket")]
    pub in_game_market: InGameMarket,

    #[serde(rename = "Invasions")]
    pub invasions: Vec<Invasion>,

    #[serde(rename = "HubEvents")]
    pub hub_events: Vec<()>,

    #[serde(rename = "NodeOverrides")]
    pub node_overrides: Vec<NodeOverride>,

    #[serde(rename = "VoidTraders")]
    pub void_traders: Vec<VoidTrader>,

    #[serde(rename = "PrimeVaultTraders")]
    pub prime_vault_traders: Vec<PrimeVaultTrader>,

    #[serde(rename = "VoidStorms")]
    pub void_storms: Vec<VoidStorm>,

    #[serde(rename = "PrimeAccessAvailability")]
    pub prime_access_availability: PrimeAccessAvailability,

    #[serde(rename = "PrimeVaultAvailabilities")]
    pub prime_vault_availability: Vec<bool>,

    #[serde(rename = "PrimeTokenAvailability")]
    pub prime_token_availability: bool,

    #[serde(rename = "DailyDeals")]
    pub daily_deals: Vec<DailyDeal>,

    #[serde(rename = "LibraryInfo")]
    pub library_info: LibraryInfo,

    #[serde(rename = "PVPChallengeInstances")]
    pub pvp_challenge_instances: Vec<PvpChallengeInstance>,

    #[serde(rename = "PersistentEnemies")]
    pub persistent_enemies: Vec<()>,

    #[serde(rename = "PVPAlternativeModes")]
    pub pvp_alternative_modes: Vec<()>,

    #[serde(rename = "PVPActiveTournaments")]
    pub pvp_active_tournaments: Vec<()>,

    #[serde(rename = "ProjectPct")]
    pub project_pct: Vec<f64>,

    #[serde(rename = "ConstructionProjects")]
    pub construction_projects: Vec<ConstructionProject>,

    #[serde(rename = "TwitchPromos")]
    pub twitch_promos: Vec<()>,

    #[serde(rename = "ExperimentRecommended")]
    pub experiment_recommended: Vec<()>,

    /// 无尽回廊
    #[serde(rename = "EndlessXpChoices")]
    pub endless_xp_choices: Vec<EndlessXpChoice>,

    #[serde(rename = "ForceLogoutVersion")]
    pub force_logout_version: u64,

    #[serde(rename = "FeaturedGuilds")]
    pub featured_guilds: Vec<FeaturedGuild>,

    #[serde(rename = "SeasonInfo")]
    pub season_info: SeasonInfo,

    #[serde(rename = "KnownCalendarSeasons")]
    pub known_calendar_seasons: Vec<KnownCalendarSeason>,

    #[serde(rename = "Tmp")]
    pub tmp: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct Day {
    pub day: u16,
    pub events: Vec<CalendarEvent>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct CalendarEvent {
    r#type: String,
    pub reward: Option<String>,
    pub upgrade: Option<String>,
    pub challenge: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct Challenge {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "Daily")]
    pub daily: Option<bool>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "Challenge")]
    pub challenge: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct FeaturedGuild {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Tier")]
    pub tier: u8,
    #[serde_as(deserialize_as = "Option<FromInto<Oid>>")]
    #[serde(rename = "AllianceId")]
    pub alliance_id: Option<String>,

    #[serde(rename = "Icon")]
    pub icon: Option<u8>,

    #[serde(rename = "Emblem")]
    pub emblem: Option<bool>,

    #[serde(rename = "HiddenPlatforms")]
    pub hidden_platforms: Option<HashMap<String, bool>>,

    #[serde(rename = "IconOverride")]
    pub icon_override: Option<u8>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct EndlessXpChoice {
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Choices")]
    pub choices: Vec<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct PvpChallengeInstance {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "challengeTypeRefID")]
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

#[derive(Debug, Deserialize, Clone)]
pub struct Param {
    pub n: String,
    pub v: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LibraryInfo {
    #[serde(rename = "LastCompletedTargetType")]
    pub last_completed_target_type: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct DailyDeal {
    #[serde(rename = "StoreItem")]
    pub store_item: String,

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

#[derive(Debug, Deserialize, Clone)]
pub struct PrimeAccessAvailability {
    #[serde(rename = "State")]
    pub state: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct ScheduleInfo {
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "PreviewHiddenUntil")]
    pub preview_hidden_until: Option<DateTime<Utc>>,

    #[serde(rename = "FeaturedItem")]
    pub featured_item: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct ManifestItem {
    #[serde(rename = "ItemType")]
    pub item_type: String,
    #[serde(rename = "PrimePrice")]
    pub prime_price: Option<u32>,

    #[serde(rename = "RegularPrice")]
    pub regular_price: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct VoidTrader {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Activation")]
    pub activation: Option<DateTime<Utc>>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Expiry")]
    pub expiry: Option<DateTime<Utc>>,

    #[serde(rename = "Character")]
    pub character: String,

    #[serde(rename = "Node")]
    pub node: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct NodeOverride {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "Node")]
    pub node: String,
    #[serde(rename = "Hide")]
    pub hide: Option<bool>,
    #[serde(rename = "Seed")]
    pub seed: Option<u32>,

    #[serde(rename = "LevelOverride")]
    pub level_override: Option<String>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Activation")]
    pub activation: Option<DateTime<Utc>>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Expiry")]
    pub expiry: Option<DateTime<Utc>>,

    #[serde(rename = "CustomNpcEncounters")]
    pub custom_npc_encounters: Option<Vec<String>>,

    #[serde(rename = "Faction")]
    pub faction: Option<String>,

    #[serde(rename = "EnemySpec")]
    pub enemy_spec: Option<String>,

    #[serde(rename = "ExtraEnemySpec")]
    pub extra_enemy_spec: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
    #[serde(rename = "AttackerReward")]
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

#[derive(Debug, Deserialize, Clone)]
pub struct InvasionMissionInfo {
    pub seed: i32,
    pub faction: String,
    #[serde(rename = "missionReward")]
    pub mission_reward: Option<Vec<()>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct InGameMarket {
    #[serde(rename = "LandingPage")]
    pub landing_page: LandingPage,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct LandingPage {
    #[serde(rename = "Categories")]
    pub categories: Vec<Category>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Category {
    #[serde(rename = "CategoryName")]
    pub category_name: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Icon")]
    pub icon: String,

    #[serde(rename = "Items")]
    pub items: Vec<String>,

    #[serde(rename = "AddToMenu")]
    pub add_to_menu: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct FlashSale {
    #[serde(rename = "TypeName")]
    pub type_name: String,
    #[serde(rename = "ShowInMarket")]
    pub show_in_market: bool,
    #[serde(rename = "HideFromMarket")]
    pub hide_from_market: bool,
    #[serde(rename = "SupporterPack")]
    pub supporter_pack: bool,
    #[serde(rename = "Discount")]
    pub discount: f64,
    #[serde(rename = "BogoBuy")]
    pub bogo_buy: f64,
    #[serde(rename = "BogoGet")]
    pub bogo_get: f64,
    #[serde(rename = "UrlOverride")]
    pub url_override: Option<String>,
    #[serde(rename = "PremiumOverride")]
    pub premium_override: f64,
    #[serde(rename = "RegularOverride")]
    pub regular_override: f64,
    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "StartDate")]
    pub start_date: DateTime<Utc>,

    #[serde_as(deserialize_as = "TryFromInto<Date>")]
    #[serde(rename = "EndDate")]
    pub end_date: DateTime<Utc>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "ProductExpiryOverride")]
    pub product_expiry_override: Option<DateTime<Utc>>,

    #[serde(rename = "DailySaleGenerated")]
    pub daily_sale_generated: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
    #[serde(rename = "LocalizeTag")]
    pub localize_tag: Option<String>,
    #[serde(rename = "LocalizeDescTag")]
    pub localize_desc_tag: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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

    #[serde(rename = "Hard")]
    pub hard: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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

    #[serde(rename = "Jobs")]
    pub jobs: Option<Vec<Job>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Job {
    #[serde(rename = "jobType")]
    pub job_type: Option<String>,
    // #[serde(rename = "rewards")]
    pub rewards: String,
    #[serde(rename = "masteryReq")]
    pub mastery_req: u8,
    #[serde(rename = "minEnemyLevel")]
    pub min_enemy_level: u16,
    #[serde(rename = "maxEnemyLevel")]
    pub max_enemy_level: u16,
    #[serde(rename = "xpAmounts")]
    pub xp_amounts: Vec<u16>,
    pub endless: Option<bool>,

    #[serde(rename = "isVault")]
    pub is_vault: Option<bool>,

    #[serde(rename = "locationTag")]
    pub location_tag: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
    #[serde(rename = "Missions")]
    pub missions: Vec<Mission>,
}
#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Mission {
    #[serde(rename = "missionType")]
    pub mission_type: String,
    pub node: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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

    #[serde(rename = "Twitter")]
    pub twitter: Option<bool>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
    #[serde(rename = "missionType")]
    pub mission_type: String,
    #[serde(rename = "modifierType")]
    pub modifier_type: String,
    pub node: String,
    pub tileset: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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

    #[serde(rename = "descText")]
    pub desc_text: Option<String>,

    /// 曲翼任务
    #[serde(rename = "archwingRequired")]
    pub archwing_required: Option<bool>,

    #[serde(rename = "vipAgent")]
    pub vip_agent: Option<String>,

    pub nightmare: Option<bool>,

    #[serde(rename = "isSharkwingMission")]
    pub is_sharkwing_mission: Option<bool>,

    #[serde(rename = "advancedSpawners")]
    pub advanced_spawners: Option<Vec<String>>,

    #[serde(rename = "requiredItems")]
    pub required_items: Option<Vec<String>>,

    #[serde (rename = "consumeRequiredItems")]
    pub consume_required_items: Option<bool>,

}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
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

    #[serde(rename = "Node")]
    pub node: Option<String>,

    #[serde(rename = "ScoreVar")]
    pub score_var: Option<String>,
    #[serde(rename = "ScoreLocTag")]
    pub score_loc_tag: Option<String>,
    #[serde(rename = "Count")]
    pub count: Option<u32>,
    #[serde(rename = "HealthPct")]
    pub health_pct: Option<f64>,
    #[serde(rename = "Regions")]
    pub regions: Option<Vec<u32>>,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "ToolTip")]
    pub tooltip: Option<String>,
    #[serde(rename = "OptionalInMission")]
    pub optional_in_mission: Option<bool>,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde_as(deserialize_as = "Option<Vec<FromInto<Oid>>>")]
    #[serde(rename = "UpgradeIds")]
    pub upgrade_ids: Option<Vec<Oid>>,

    #[serde(rename = "Personal")]
    pub personal: Option<bool>,

    #[serde(rename = "Community")]
    pub community: Option<bool>,
    #[serde(rename = "Goal")]
    pub goal: Option<u32>,
    #[serde(rename = "Reward")]
    pub reward: Option<Reward>,

    #[serde(rename = "InterimGoals")]
    pub interim_goals: Option<Vec<u32>>,
    #[serde(rename = "InterimRewards")]
    pub interim_rewards: Option<Vec<Reward>>,

    #[serde(rename = "BonusGoal")]
    pub bonus_goal: Option<u32>,

    #[serde(rename = "BonusReward")]
    pub bonus_reward: Option<Reward>,

    #[serde(rename = "Bounty")]
    pub bounty: Option<bool>,

    #[serde(rename = "ClampNodeScores")]
    pub clamp_node_scores: Option<bool>,

    #[serde(rename = "ConcurrentMissionKeyNames")]
    pub concurrent_mission_key_names: Option<Vec<String>>,

    #[serde(rename = "ConcurrentNodeReqs")]
    pub concurrent_node_reqs: Option<Vec<u32>>,

    #[serde(rename = "ConcurrentNodes")]
    pub concurrent_nodes: Option<Vec<String>>,

    #[serde(rename = "Faction")]
    pub faction: Option<String>,

    #[serde(rename = "Icon")]
    pub icon: Option<String>,

    #[serde(rename = "MissionKeyName")]
    pub mission_key_name: Option<String>,

    #[serde(rename = "NightLevel")]
    pub night_level: Option<String>,

    #[serde(rename = "Success")]
    pub success: Option<u32>,

    #[serde(rename = "JobAffiliationTag")]
    pub job_affiliation_tag: Option<String>,

    #[serde_as(deserialize_as = "Option<FromInto<Oid>>")]
    #[serde(rename = "JobCurrentVersion")]
    pub job_current_version: Option<String>,

    #[serde_as(deserialize_as = "Option<FromInto<Oid>>")]
    #[serde(rename = "JobPreviousVersion")]
    pub job_previous_version: Option<String>,
    #[serde(rename = "Jobs")]
    pub jobs: Option<Vec<Job>>,
    #[serde(rename = "PreviousJobs")]
    pub previous_jobs: Option<Vec<Job>>,
    #[serde(rename = "VictimNode")]
    pub victim_node: Option<String>,

    #[serde(rename = "ArchwingDrops")]
    pub archwing_drops: Option<Vec<String>>,

    #[serde(rename = "Best")]
    pub best: Option<bool>,

    #[serde(rename = "Fomorian")]
    pub fomorian: Option<bool>,

    #[serde(rename = "MissionInfo")]
    pub mission_info: Option<MissionInfo>,


    #[serde(rename = "ScoreMaxTag")]
    pub score_max_tag: Option<String>,

    #[serde(rename = "ScoreTagBlocksGuildTierChanges")]
    pub score_tag_blocks_guild_tier_changes: Option<bool>,

    #[serde(rename = "RegionDrops")]
    pub region_drops: Option<Vec<String>>,
}

// #[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Reward {
    pub credits: Option<u32>,
    pub xp: Option<u32>,
    pub items: Option<Vec<String>>,
    #[serde(rename = "countedItems")]
    pub counted_items: Option<Vec<CountedItem>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CountedItem {
    #[serde(rename = "ItemType")]
    pub item_type: String,
    #[serde(rename = "ItemCount")]
    pub item_count: u32,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Event {
    #[serde_as(deserialize_as = "FromInto<Oid>")]
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "Messages")]
    pub message: Vec<Message>,

    #[serde(rename = "Prop")]
    pub prop: String,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "Date")]
    pub date: Option<DateTime<Utc>>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "EventStartDate")]
    pub event_start_date: Option<DateTime<Utc>>,

    #[serde_as(deserialize_as = "Option<TryFromInto<Date>>")]
    #[serde(rename = "EventEndDate")]
    pub event_end_date: Option<DateTime<Utc>>,

    #[serde(rename = "Priority")]
    pub priority: bool,
    #[serde(rename = "MobileOnly")]
    pub mobile_only: bool,
    #[serde(rename = "Community")]
    pub community: Option<bool>,

    /// icon name
    #[serde(rename = "Icon")]
    pub icon: Option<String>,

    #[serde(rename = "ImageUrl")]
    pub image_url: Option<String>,

    #[serde(rename = "HideEndDateModifier")]
    pub hide_end_date_modifier: Option<bool>,

    #[serde(rename = "Links")]
    pub links: Option<Vec<Link>>,

    #[serde(rename = "EventLiveUrl")]
    pub event_live_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    #[serde(rename = "Link")]
    pub link: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Message {
    #[serde(rename = "LanguageCode")]
    pub language_code: String,

    #[serde(rename = "Message")]
    pub message: String,
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test_decode_raw() -> eyre::Result<()> {
        let file = std::fs::File::open("../ui/cache/ws_st_org-20250905.json")?;
        let jd = &mut serde_json::Deserializer::from_reader(file);
        let mut unused = BTreeSet::new();
        let model: RawWorldState = serde_ignored::deserialize(jd, |path| {
            unused.insert(path.to_string());
        })?;
        // let model = serde_json::from_reader::<_, RawWorldState>(file)?;
        // dbg!(model);
        dbg!(unused);
        Ok(())
    }
}
