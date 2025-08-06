use super::helper::{BoolFromAny, DeDate, DurationFromStr, NumberFromStringLoose, bool_false};
use chrono::{DateTime, Duration, Utc};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::{DisplayFromStr, FromInto};

#[derive(Deserialize, Debug)]
pub struct Err(pub String);

impl Err {
    pub fn message(&self) -> &str {
        match self.0.as_str() {
            "0" => "哎呀，出错啦，请刷新后再试吧！",
            "-1" => "抱歉，该账户操作太快了，歇息几秒再重试",
            "-2" => "抱歉，请重试",
            "-3" => "抱歉，抢礼包的人太多了，您没挤进去，请重试",
            "-4" => "抱歉，您来晚了，今天礼包已被抢光了",
            "-5" => "抱歉，您来晚了，礼包已被抢光了<br>欢迎继续关注WeGame的其他活动",
            "-6" => "抱歉，活动还未开始",
            "-7" => "抱歉，本活动已经结束<br>欢迎继续关注WeGame的其他活动",
            "-8" => "抱歉，操作失败，请重试",
            "-9" => "抱歉，登录验证已过期<br />请点击确定，注销之后重新登录",
            "-13" => "抱歉，非Ajax请求",
            "-14" => "抱歉，该账户操作太快了，歇息几秒再重试",
            "-15" => "抱歉，该账户操作太快了，歇息几秒再重试",
            "-11" => "抱歉，您还不是WeGame用户<br>请先下载并且登录 WeGame",
            // "-16" => "抱歉，请更换网络环境重试!",
            "-18" => "抱歉，您今天领取该礼包的数量已达上限",
            "-19" => "抱歉，您领取该礼包的数量已达上限",
            "-21" => "抱歉，请先绑定大区",
            "-30" => "验证码预留",
            // "-40" => "抱歉，活动太火爆了，请稍后重试",
            "-41" => "抱歉，本活动只支持QQ帐号登录",
            "-42" => "抱歉，Q币已经发放至您已登记的QQ号，无需重复操作",
            "-51" => "抱歉，您的账号存在部分异常情况，暂时不能参加活动。",
            "-99" => "抱歉，您已经领取过该礼包<br />当前礼包单Q仅限领取一次哟",
            "4000052-1" => "抱歉，您的账号存在安全风险，风控平台检查本次Q币发放不通过",

            "-16" => "抱歉，登录过期，请刷新一下重新登录!",
            "-40" => "抱歉，活动太火爆了，请稍后重试",
            // "-51" => "抱歉，您的账号存在部分异常情况，暂时不能参加活动。",
            "-110" => "抱歉，该活动只支持QQ登录",
            "-500" => "领取失败，请重试",
            "-101" => "抱歉，您的抽奖机会不足",
            "-102" => "抱歉，您的游戏时长还未达到2小时，无法领取，请先进行游戏",
            "-301" => "抱歉，您没有可以领取的抽奖机会",
            // "-14" | "-15" => "抱歉，该账户操作太快了，歇息几秒再重试",
            _ => "哎呀，人太多了，您没挤进去，请稍后重试",
        }
        // return "error".to_string();
    }
}

/// 状态码 1 成功 0 失败
#[derive(Deserialize, Debug)]
#[serde(tag = "status", content = "data")]
pub enum WorldStateRes {
    #[serde(rename = "1")]
    Ok(Box<WorldStateData>),
    #[serde(rename = "0")]
    Err(Err),
}

/// Warframe world state on WeGame
#[derive(Deserialize, Serialize, Debug)]
pub struct WorldStateData {
    pub timestamp: DateTime<Utc>,
    /// 活动事件
    pub events: Vec<Event>,
    /// 战术警报
    pub alerts: Vec<Alert>,
    /// 突击
    pub sortie: Sortie,
    /// 集团任务
    #[serde(rename = "syndicateMissions")]
    pub syndicate_missions: Vec<SyndicateMission>,
    /// 虚空裂缝
    pub fissures: Vec<Fissure>,

    /// 全局加成
    #[serde(rename = "globalUpgrades")]
    pub global_upgrades: Vec<GlobalUpgrade>,

    /// 闪购
    #[serde(rename = "flashSales")]
    pub flash_sales: Vec<FlashSale>,

    /// 入侵
    pub invasions: Vec<Invasion>,
    // darkSectors
    /// 虚空商人
    #[serde(rename = "voidTrader")]
    pub void_trader: VoidTrader,
    /// 每日特惠pub
    #[serde(rename = "dailyDeals")]
    pub daily_deals: Vec<DailyDeal>,
    /// 结合仪式
    pub simaris: Simaris,

    /// 武形秘仪 挑战
    #[serde(rename = "conclaveChallenges")]
    pub conclave_challenges: Vec<ConclaveChallenge>,

    /// unknown
    #[serde(rename = "persistentEnemies")]
    pub persistent_enemies: Vec<()>,

    /// 地球时间
    #[serde(rename = "earthCycle")]
    pub earth_cycle: EarthCycle,

    /// 希图斯（夜灵平野）时间
    #[serde(rename = "cetusCycle")]
    pub cetus_cycle: CetusCycle,

    /// 殁世幽都（魔胎之境）时间
    #[serde(rename = "cambionCycle")]
    pub cambion_cycle: CambionCycle,

    /// 扎里曼 时间
    #[serde(rename = "zarimanCycle")]
    pub zariman_cycle: ZarimanCycle,

    // unknown
    #[serde(rename = "weeklyChallenges")]
    pub weekly_challenges: Vec<()>,
    /// 福尔图纳（奥布山谷） 时间
    #[serde(rename = "vallisCycle")]
    pub vallis_cycle: VallisCycle,

    /// 午夜电波
    pub nightwave: Nightwave,

    /// 异常区域
    #[serde(rename = "sentientOutposts")]
    pub sentient_outposts: SentientOutposts,

    /// 钢铁之路
    #[serde(rename = "steelPath")]
    pub steel_path: SteelPath,

    /// 圣装重生
    #[serde(rename = "vaultTrader")]
    pub vault_trader: VaultTrader,

    /// 执刑官猎杀
    #[serde(rename = "archonHunt")]
    pub archon_hunt: ArchonHunt,

    /// 虚空风暴
    #[serde(rename = "voidStorms")]
    pub void_storms: Vec<VoidStorm>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct FlashSale {
    pub item: String,
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,
    pub discount: String,
    #[serde(rename = "regularOverride")]
    pub regular_override: String,
    #[serde(rename = "premiumOverride")]
    pub premium_override: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isShownInMarket")]
    pub is_shown_in_market: bool,
    pub id: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub expired: bool,

    #[serde_as(deserialize_as = "DurationFromStr")]
    pub eta: Duration,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct GlobalUpgrade {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub upgrade: String,
    pub operation: String,
    #[serde(rename = "operationSymbol")]
    pub operation_symbol: String,
    #[serde(rename = "upgradeOperationValue")]
    pub upgrade_operation_value: String,

    #[serde_as(deserialize_as = "BoolFromAny")]
    pub expired: bool,

    #[serde_as(deserialize_as = "DurationFromStr")]
    pub eta: Duration,
    pub desc: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Oid {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct VoidStorm {
    #[serde(rename = "_id")]
    pub id: Oid,
    #[serde(rename = "Node")]
    pub node: String,

    #[serde_as(deserialize_as = "FromInto<DeDate>")]
    #[serde(rename = "Activation")]
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "FromInto<DeDate>")]
    #[serde(rename = "Expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "ActiveMissionTier")]
    pub active_mission_tier: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ArchonHunt {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,

    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    #[serde(rename = "rewardPool")]
    pub reward_pool: String,
    pub variants: Vec<()>,
    pub missions: Vec<ArchonHuntMission>,
    pub boss: String,
    pub faction: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub expired: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub eta: Duration,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ArchonHuntMission {
    pub node: String,
    #[serde(rename = "nodeKey")]
    pub node_key: String,
    r#type: String,
    #[serde(rename = "typeKey")]
    pub type_key: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub nightmare: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "archwingRequired")]
    pub archwing_required: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isSharkwing")]
    pub is_sharkwing: bool,
    #[serde(rename = "advancedSpawners")]
    pub advanced_spawners: Vec<()>,
    #[serde(rename = "requiredItems")]
    pub required_items: Vec<()>,
    #[serde(rename = "levelAuras")]
    pub level_auras: Vec<()>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VaultTraderSchedule {
    pub expiry: DateTime<Utc>,
    pub item: String,
}
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct VaultTraderInventory {
    pub item: String,
    #[serde_as(deserialize_as = "NumberFromStringLoose")]
    pub ducats: i32,
    #[serde_as(deserialize_as = "NumberFromStringLoose")]
    pub credits: i32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct VaultTrader {
    pub id: String,
    pub activation: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    pub character: String,
    pub location: String,
    pub inventory: Vec<VaultTraderInventory>,
    #[serde(rename = "psId")]
    pub ps_id: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "endString")]
    pub end_string: Duration,
    #[serde(rename = "initialStart")]
    pub initial_start: DateTime<Utc>,
    pub schedule: Vec<VaultTraderSchedule>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct SteelPathReward {
    pub name: String,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub cost: i32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct SteelPath {
    #[serde(rename = "currentReward")]
    pub current_reward: SteelPathReward,
    pub activation: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub remaining: Duration,
    pub rotation: Vec<SteelPathReward>,
    pub evergreens: Vec<SteelPathReward>,
    pub incursions: SteelPathIncursions,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SteelPathIncursions {
    pub id: String,
    pub activation: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SentientOutpostsMission {
    pub node: String,
    pub faction: String,
    r#type: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct SentientOutposts {
    pub mission: SentientOutpostsMission,
    pub activation: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    pub id: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Nightwave {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,

    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub season: i32,
    pub tag: String,
    pub phase: String,
    pub params: Vec<()>,
    #[serde(rename = "possibleChallenges")]
    pub possible_challenges: Vec<()>,
    #[serde(rename = "activeChallenges")]
    pub active_challenges: Vec<Challenge>,
    #[serde(rename = "rewardTypes")]
    pub reward_types: Vec<String>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Challenge {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isDaily", default = "bool_false")]
    pub is_daily: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isElite")]
    pub is_elite: bool,

    pub desc: String,
    pub title: String,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub reputation: u32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct VallisCycle {
    pub id: String,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isWarm")]
    pub is_warm: bool,
    pub state: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    pub time_left: Duration,
    #[serde(rename = "shortString")]
    pub short_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ZarimanCycle {
    pub id: String,
    #[serde(rename = "bountiesEndDate")]
    pub bounties_end_date: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isCorpus")]
    pub is_corpus: bool,
    pub state: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    pub time_left: Duration,

    #[serde(rename = "shortString")]
    pub short_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct CambionCycle {
    pub id: String,
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    pub time_left: Duration,
    pub active: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct CetusCycle {
    pub id: String,
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,

    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isDay")]
    pub is_day: bool,
    pub state: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "timeLeft")]
    pub time_left: Duration,

    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isCetus")]
    pub is_cetus: bool,

    #[serde(rename = "shortString")]
    pub short_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct EarthCycle {
    pub id: String,
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isDay")]
    pub is_day: bool,
    pub state: String,
    #[serde(rename = "timeLeft")]
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub time_left: Duration,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ConclaveChallenge {
    pub id: String,
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub amount: i32,
    pub mode: String,
    pub category: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub eta: Duration,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub expired: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub daily: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "rootChallenge")]
    pub root_challenge: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "endString")]
    pub end_string: Duration,
    pub description: String,
    pub title: String,
    #[serde(rename = "asString")]
    pub as_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Simaris {
    pub target: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isTargetActive")]
    pub is_target_active: bool,
    #[serde(rename = "asString")]
    pub as_string: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct DailyDeal {
    pub item: String,
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "originalPrice")]
    pub original_price: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "salePrice")]
    pub sale_price: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub total: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub sold: i32,
    pub id: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub eta: Duration,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub discount: i32,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct VoidTrader {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    pub character: String,
    pub location: String,
    pub inventory: Vec<()>,
    #[serde(rename = "psId")]
    pub ps_id: String,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "endString")]
    pub end_string: Duration,
    #[serde(rename = "initialStart")]
    pub initial_start: DateTime<Utc>,
    pub schedule: Vec<()>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InvasionSide {
    pub reward: EventReward,
    pub faction: String,
    #[serde(rename = "factionKey")]
    pub faction_key: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Invasion {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub node: String,
    #[serde(rename = "nodeKey")]
    pub node_key: String,
    pub desc: String,
    #[serde(rename = "attackerReward")]
    pub attacker_reward: EventReward,
    #[serde(rename = "attackingFaction")]
    pub attacking_faction: String,
    pub attacker: InvasionSide,
    #[serde(rename = "defenderReward")]
    pub defender_reward: EventReward,
    #[serde(rename = "defendingFaction")]
    pub defending_faction: String,
    pub defender: InvasionSide,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "vsInfestation")]
    pub vs_infestation: bool,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub count: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "requiredRuns")]
    pub required_runs: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub completion: f64,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub completed: bool,

    // ETA has no sense here
    // if the count reached requiredRuns, then the event is completed

    // #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    // eta: Duration,
    #[serde(rename = "rewardTypes")]
    pub reward_types: Vec<String>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct SyndicateMissionJob {
    pub id: String,
    #[serde(rename = "rewardPool")]
    pub reward_pool: Vec<()>,
    r#type: String,
    #[serde_as(deserialize_as = "Vec<DisplayFromStr>")]
    #[serde(rename = "enemyLevels")]
    pub enemy_levels: Vec<i32>,
    #[serde(rename = "standingStages")]
    pub standing_stages: Vec<String>,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "minMR")]
    pub min_mr: i32,
    pub expiry: DateTime<Utc>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct SyndicateMission {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,

    pub syndicate: String,
    #[serde(rename = "syndicateKey")]
    pub syndicate_key: String,

    pub nodes: Vec<String>,
    pub jobs: Vec<SyndicateMissionJob>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub eta: Duration,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SortieVariant {
    #[serde(rename = "missionType")]
    pub mission_type: String,
    pub modifier: String,
    #[serde(rename = "modifierDescription")]
    pub modifier_description: String,
    pub node: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Sortie {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    #[serde(rename = "rewardPool")]
    pub reward_pool: String,
    pub variants: Vec<SortieVariant>,
    pub missions: Vec<()>,
    pub boss: String,
    pub faction: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub expired: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub eta: Duration,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct AlertMission {
    pub description: String,
    pub node: String,
    #[serde(rename = "nodeKey")]
    pub node_key: String,
    r#type: String,
    #[serde(rename = "typeKey")]
    pub type_key: String,
    pub faction: String,
    #[serde(rename = "factionKey")]
    pub faction_key: String,
    pub reward: EventReward,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "minEnemyLevel")]
    pub min_enemy_level: i32,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "maxEnemyLevel")]
    pub max_enemy_level: i32,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub nightmare: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "archwingRequired")]
    pub archwing_required: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isSharkwing")]
    pub is_sharkwing: bool,
    #[serde(rename = "levelOverride")]
    pub level_override: String,
    #[serde(rename = "enemySpec")]
    pub enemy_spec: String,
    #[serde(rename = "advancedSpawners")]
    pub advanced_spawners: Vec<()>,
    #[serde(rename = "requiredItems")]
    pub required_items: Vec<()>,
    #[serde(rename = "levelAuras")]
    pub level_auras: Vec<()>,
}

/// 战术警报
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Alert {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    pub mission: AlertMission,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub eta: Duration,
    #[serde(rename = "rewardTypes")]
    pub reward_types: Vec<String>,
    pub tag: String,
}

/// 虚空裂缝
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Fissure {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,
    pub node: String,
    #[serde(rename = "missionType")]
    pub mission_type: String,
    #[serde(rename = "missionKey")]
    pub mission_key: String,
    pub enemy: String,
    #[serde(rename = "enemyKey")]
    pub enemy_key: String,
    #[serde(rename = "nodeKey")]
    pub node_key: String,

    pub tier: String,

    /// this could be a number but it's a string
    /// 1,2,3,4,5,VoidT6
    // #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "tierNum")]
    pub tier_num: String,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub expired: bool,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    pub eta: Duration,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isStorm")]
    pub is_storm: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isHard")]
    pub is_hard: bool,
}

/// 活动事件
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub id: String,
    pub activation: DateTime<Utc>,
    #[serde_as(deserialize_as = "DurationFromStr", serialize_as = "DisplayFromStr")]
    #[serde(rename = "startString")]
    pub start_string: Duration,
    pub expiry: DateTime<Utc>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub active: bool,

    #[serde_as(deserialize_as = "NumberFromStringLoose")]
    #[serde(rename = "maximumScore")]
    pub maximum_score: i32,

    #[serde_as(deserialize_as = "NumberFromStringLoose")]
    #[serde(rename = "currentScore", default = "Default::default")]
    pub current_score: i32,
    #[serde(rename = "smallInterval")]
    pub small_interval: String,
    #[serde(rename = "largeInterval")]
    pub large_interval: String,
    pub description: String,
    pub tooltip: String,

    // this field is not exist in 净化尸鬼 event
    #[serde(default = "Default::default")]
    pub node: String,
    #[serde(rename = "concurrentNodes")]
    pub concurrent_nodes: Vec<()>,
    #[serde(rename = "scoreLocTag")]
    pub score_loc_tag: Option<String>,
    pub rewards: Vec<EventReward>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    pub expired: bool,
    #[serde_as(deserialize_as = "NumberFromStringLoose")]
    #[serde(default = "Default::default")]
    pub health: f64,
    #[serde(rename = "interimSteps")]
    pub interim_steps: Vec<InterimStep>,
    #[serde(rename = "progressSteps")]
    pub progress_steps: Vec<()>,
    #[serde_as(deserialize_as = "BoolFromAny")]
    // this field is not exist in 净化尸鬼 event
    #[serde(rename = "isPersonal", default = "bool_false")]
    pub is_personal: bool,
    #[serde_as(deserialize_as = "BoolFromAny")]
    #[serde(rename = "isCommunity", default = "bool_false")]
    pub is_community: bool,
    #[serde(rename = "regionDrops")]
    pub region_drops: Vec<()>,
    #[serde(rename = "archwingDrops")]
    pub archwing_drops: Vec<()>,
    pub metadata: Vec<()>,
    #[serde(rename = "completionBonuses")]
    pub completion_bonuses: Vec<()>,
    #[serde(rename = "scoreVar")]
    pub score_var: Option<String>,
    #[serde(rename = "altExpiry")]
    pub alt_expiry: DateTime<Utc>,
    #[serde(rename = "altActivation")]
    pub alt_activation: DateTime<Utc>,
    #[serde(rename = "nextAlt")]
    pub next_alt: NextAlt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NextAlt {
    pub expiry: DateTime<Utc>,
    pub activation: DateTime<Utc>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct InterimStep {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub goal: i32,
    pub reward: EventReward,
    pub message: Vec<String>,
}

/// 活动奖励 data -> events[*] -> rewards[*]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct EventReward {
    pub items: Vec<String>,
    #[serde(rename = "countedItems")]
    pub counted_items: Vec<CountedItem>,

    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub credits: i32,
    #[serde(rename = "asString")]
    pub as_string: String,
    #[serde(rename = "itemString")]
    pub item_string: String,
    pub thumbnail: String,
    pub color: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct CountedItem {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub count: i32,
    r#type: String,
    pub key: String,
}
