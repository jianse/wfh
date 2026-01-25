use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::fs::{self};
use std::io::BufReader;
use std::ops::Deref;
use std::path::Path;
use std::sync::LazyLock;
use std::time::Instant;

use chrono::DateTime;
use chrono::Days;
use chrono::Duration;
use chrono::TimeZone;
use chrono::Utc;
use export::PE;
use export::PublicExports;
use export::ReverseMap;
use export::get_opt_array;
use export::get_opt_u64;
use export::get_str;
use export::get_u64;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_with::TryFromInto;
use serde_with::serde_as;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tracing::info;
use tracing::warn;

use crate::api::BizResult;
use crate::api::raw_world_state::TRisk;
use crate::api::raw_world_state::TVariable;
use crate::api::raw_world_state::model::RawWorldState;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentientOutpost {
    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub node: String,

    pub node_name: Option<String>,

    pub mission_type: Option<String>,
}

impl SentientOutpost {
    fn new(node_code: u64, time: DateTime<Utc>, deps: &Deps) -> Self {
        let offset = time.timestamp() % 1800;
        let activation = time - Duration::seconds(offset);
        let expiry = activation + Duration::minutes(30);
        let node = format!("CrewBattleNode{node_code}");
        // let  mut
        if let Some(region) = deps.get_region(&node) {
            Self {
                activation,
                expiry,
                node,
                node_name: Some(region.name.clone()),
                mission_type: Some(deps.get_mission_type_by_id(region.mission_index)),
            }
        } else {
            Self {
                activation,
                expiry,
                node,
                node_name: None,
                mission_type: None,
            }
        }
        // todo!("123456")
    }
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
        alert.mission_info.mission_type =
            deps.get_mission_type_by_tag(&alert.mission_info.mission_type);

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
        mission.mission_type = deps.get_mission_type_by_tag(&mission.mission_type);
    }

    // void storm
    for void_storm in raw.void_storms.iter_mut() {
        // void_storm.node
        if let Some(region) = deps.get_region(&void_storm.node) {
            void_storm.node = region.with_plant();
            void_storm.mission_type = Some(deps.get_mission_type_by_id(region.mission_index))
        };
    }

    // add steel path weekly rewards
    raw.steel_path = Some(from_time(raw.time, &deps));

    // fill daily deals
    for daily_deal in raw.daily_deals.iter_mut() {
        daily_deal.store_item_name = Some(deps.find_game_item(&daily_deal.store_item));
    }

    // lite sortie
    for lite_sortie in raw.lite_sorties.iter_mut() {
        lite_sortie.boss_name = Some(deps.get_sortie_boss_name(&lite_sortie.boss));
        for mission in lite_sortie.missions.iter_mut() {
            // mission.node
            if let Some(region) = deps.get_region(&mission.node) {
                mission.node = region.with_plant();
            };
            mission.mission_type = deps.get_mission_type_by_tag(&mission.mission_type);
        }
        // lite sortie rewards are bound to boss
        // add them in front end should be good
    }

    // add sentient outpost info
    let tmp: serde_json::Value = serde_json::from_str(&raw.tmp)?;
    if let Some(node_num) = get_opt_u64(&tmp, "sfn")? {
        raw.sentient_outpost = Some(SentientOutpost::new(node_num, raw.time, &deps));
    }

    // sorties
    for sortie in raw.sorties.iter_mut() {
        for variant in sortie.variants.iter_mut() {
            if let Some(region) = deps.get_region(&variant.node) {
                variant.node = region.with_plant();
            };
            variant.mission_type = deps.get_mission_type_by_tag(&variant.mission_type);

            // translate modifiers
            if let Some(modifier) = deps.get_sortie_modifier(&variant.modifier_type) {
                variant.modifier_name = Some(modifier.name(&deps));
                variant.modifier_description = Some(modifier.description(&deps));
            } else {
                warn!("unknown modifier type: {}", variant.modifier_type);
                variant.modifier_name = Some(variant.modifier_type.clone())
            }
        }
    }

    // void traders
    for trader in raw.void_traders.iter_mut() {
        // trader.node
        if let Some(region) = deps.get_region(&trader.node) {
            trader.node = region.with_plant();
        };

        // translate manifest
        if let Some(manifest) = trader.manifest.as_mut() {
            for item in manifest.iter_mut() {
                item.item_name = Some(deps.find_game_item(&item.item_type));
            }
        }
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
        let c = deps.get_nightwave_challenge(&challenge.challenge);
        if let Some(c) = c {
            challenge.name = Some(c.name.clone());
            challenge.description = Some(c.description.replace("|COUNT|", &c.required.to_string()));
            challenge.xp = Some(c.standing);
        }
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

        translate_reward(&mut invasion.defender_reward, &deps)?;

        invasion.attacker_mission_info.faction =
            deps.get_faction(&invasion.attacker_mission_info.faction);
        invasion.defender_mission_info.faction =
            deps.get_faction(&invasion.defender_mission_info.faction);

        // TODO: translate modifiers
    }

    // endless xp
    for exc in raw.endless_xp_choices.iter_mut() {
        match exc.category.as_str() {
            "EXC_NORMAL" => {
                for c in exc.choices.iter_mut() {
                    *c = deps.the_circuit.get_warframe_name(c, &deps)
                }
            }
            "EXC_HARD" => {
                for c in exc.choices.iter_mut() {
                    *c = deps.the_circuit.get_incarnon_name(c, &deps)
                }
            }

            _ => {
                warn!("unknown endless xp category: {}", exc.category);
            }
        }
    }

    // conquests
    for conquest in raw.conquests.iter_mut() {
        // 个人参数
        let mut t_variables = vec![];
        for var in conquest.variables.iter_mut() {
            let r = if let Some(pair) = deps.conquests.variables.get(var) {
                TVariable {
                    name: deps.get_language(&pair.name),
                    desc: deps.get_language(&pair.description),
                }
            } else {
                warn!("unknown conquest risk: {}", var);
                TVariable {
                    name: var.clone(),
                    desc: var.clone(),
                }
            };
            t_variables.push(r);
        }
        conquest.t_variables = Some(t_variables);

        // missions
        for m in conquest.missions.iter_mut() {
            m.mission_type = deps.get_mission_type_by_tag(&m.mission_type);
            m.faction = deps.get_faction(&m.faction);
            for d in m.difficulties.iter_mut() {
                // 偏差
                match conquest.r#type.as_str() {
                    "CT_LAB" => {
                        if let Some(pair) = deps.conquests.deviations.lab.get(&d.deviation) {
                            // *var = pair.clone();
                            d.deviation = deps.get_language(&pair.name);
                            d.deviation_desc = Some(deps.get_language(&pair.description));
                        } else {
                            warn!("unknown conquest deviation: {}", d.deviation);
                        }
                    }
                    "CT_HEX" => {
                        if let Some(pair) = deps.conquests.deviations.hex.get(&d.deviation) {
                            // *var = pair.clone();
                            d.deviation = deps.get_language(&pair.name);
                            d.deviation_desc = Some(deps.get_language(&pair.description));
                        } else {
                            warn!("unknown conquest deviation: {}", d.deviation);
                        }
                    }
                    _ => {
                        warn!("unknown conquest type: {}", conquest.r#type);
                    }
                };

                let mut t_risks = vec![];
                // 风险变量
                for risk in d.risks.iter_mut() {
                    let r = if let Some(pair) = deps.conquests.risks.get(risk) {
                        TRisk {
                            name: deps.get_language(&pair.name),
                            desc: deps.get_language(&pair.description),
                        }
                    } else {
                        warn!("unknown conquest risk: {}", risk);
                        TRisk {
                            name: risk.clone(),
                            desc: risk.clone(),
                        }
                    };
                    t_risks.push(r);
                }
                d.t_risks = Some(t_risks);
            }
        }
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

#[derive(Debug, Deserialize)]
pub struct Bundle {
    pub name: Option<String>,
    pub description: Option<String>,
    // icon: Vec<String>,
}

/// loads wf bundle data from local.
/// the data is borrowed from https://raw.githubusercontent.com/calamity-inc/warframe-public-export-plus/refs/heads/senpai/ExportBundles.json
pub fn load_bundles<P: AsRef<Path>>(dir_path: P) -> BizResult<HashMap<String, Bundle>> {
    let file = dir_path.as_ref().join("ExportBundles.json");
    if file.exists() {
        let json = std::fs::read_to_string(file)?;
        let bundles: HashMap<String, Bundle> = serde_json::from_str(&json)?;
        Ok(bundles)
    } else {
        Ok(HashMap::new())
    }

    // todo!("load_bundles")
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct OtherItem {
    pub unique_name: String,
    pub name: String,
    pub description: Option<String>,
}

/// other items for wegame wf
pub fn load_others<P: AsRef<Path>>(dir_path: P) -> BizResult<HashMap<String, OtherItem>> {
    let file = dir_path.as_ref().join("others.json");
    if file.exists() {
        let json = std::fs::read_to_string(file)?;
        let others: Vec<OtherItem> = serde_json::from_str(&json)?;
        let res = others
            .into_iter()
            .map(|o| (o.unique_name.clone(), o))
            .collect();
        Ok(res)
    } else {
        Ok(HashMap::new())
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

    let languages = load_languages(&dir_path)?;
    let factions = load_factions(&dir_path)?;
    let mission_types = load_mission_types(&dir_path)?;

    let mut regions = load_regions(&dir_path)?;
    let regions_ex = load_regions_ex(&dir_path, &languages)?;
    regions.extend(regions_ex);

    let steel_path = load_steel_path(&dir_path)?;

    let sortie_bosses = load_sortie_bosses(&dir_path)?;

    let sortie_modifiers = load_sortie_modifiers(&dir_path)?;

    let bundles = load_bundles(&dir_path)?;

    let others = load_others(&dir_path)?;

    let flavour_ex = load_flavour_ex(&dir_path)?;

    let the_circuit = load_the_circuit(&dir_path)?;

    let conquests = load_conquests(&dir_path)?;

    let pe = PE::new(client, cache_dir, dir.join("wire.json"), &languages);
    let (pe_data, pe_index) = pe.fetch_transform_all().await?;

    let nightwave_challenges = build_nw_challenges(&pe_data)?;
    {
        let mut deps = G_DEPS.write().await;
        *deps = Deps {
            regions,
            mission_types,
            factions,
            languages,
            steel_path,
            sortie_bosses,
            sortie_modifiers,
            pe_index,
            pe_data,
            nightwave_challenges,
            bundles,
            others,
            flavour_ex,
            the_circuit,
            conquests,
            is_initialized: true,
        };
    }
    Ok(())
}

#[derive(Debug, Deserialize, Default)]
struct Conquests {
    variables: HashMap<String, SimplePair>,
    risks: HashMap<String, SimplePair>,
    deviations: Deviations,
}

#[derive(Debug, Deserialize)]
struct SimplePair {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Default)]
struct Deviations {
    lab: HashMap<String, SimplePair>,
    hex: HashMap<String, SimplePair>,
}

fn load_conquests<P: AsRef<Path>>(dir_path: P) -> BizResult<Conquests> {
    let file = dir_path.as_ref().join("conquests.json");
    if file.exists() {
        let json = std::fs::read_to_string(file)?;
        let the_circuit: Conquests = serde_json::from_str(&json)?;
        Ok(the_circuit)
    } else {
        Ok(Conquests::default())
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct TheCircuit {
    #[serde(default = "Default::default")]
    warframes: HashMap<String, String>,

    #[serde(default = "Default::default")]
    incarnons: HashMap<String, String>,
}

impl TheCircuit {
    pub fn get_warframe_name(&self, name: &str, deps: &Deps) -> String {
        let unique_name = self.warframes.get(name);
        match unique_name {
            Some(unique_name) => {
                // deps.pe_data.get("Warframes");
                deps.find_game_item(unique_name)
            }
            None => {
                warn!("no such warframe: {name}");
                name.to_owned()
            }
        }
        // self.warframes.get(name).map(|s| s.as_str()).unwrap_or(name.to_owned())
    }

    pub fn get_incarnon_name(&self, name: &str, deps: &Deps) -> String {
        let unique_name = self.incarnons.get(name);
        match unique_name {
            Some(unique_name) => deps.find_game_item(unique_name),
            None => {
                warn!("no such incarnon: {name}");
                name.to_owned()
            }
        }
    }
}

fn load_the_circuit<P: AsRef<Path>>(dir_path: P) -> BizResult<TheCircuit> {
    let file = dir_path.as_ref().join("the_circuit.json");
    if file.exists() {
        let json = std::fs::read_to_string(file)?;
        let the_circuit: TheCircuit = serde_json::from_str(&json)?;
        Ok(the_circuit)
    } else {
        Ok(TheCircuit::default())
    }
}

fn build_nw_challenges(pe_data: &PublicExports) -> BizResult<NWChallenges> {
    let nw_cs = pe_data
        .get("SortieRewards")
        .and_then(|sr| sr.get("ExportNightwave"))
        .and_then(|nw| nw.get("challenges"));
    if let Some(cs) = nw_cs {
        let arr: Vec<NWChallenge> = serde_json::from_value(cs.clone())?;
        let data = arr
            .into_iter()
            .map(|c| (c.unique_name.clone(), c))
            .collect();
        Ok(NWChallenges(data))
    } else {
        Ok(NWChallenges::default())
    }
    // todo!()
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct FlavourEx {
    pub name: String,
    pub description: String,
}

/// flavour not included in public exports
fn load_flavour_ex<P: AsRef<Path>>(dir_path: P) -> BizResult<HashMap<String, FlavourEx>> {
    let file = dir_path.as_ref().join("flavour_ex.json");

    if file.exists() {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);
        let json: HashMap<String, FlavourEx> = serde_json::from_reader(&mut reader)?;
        Ok(json)
    } else {
        Ok(HashMap::new())
    }
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

#[derive(Debug, Deserialize)]
pub struct SortieModifier {
    pub name: String,
    pub description: String,
    #[serde(default = "Default::default")]
    pub vars: HashMap<String, String>,
}

impl SortieModifier {
    pub fn name(&self, deps: &Deps) -> String {
        let name = deps.get_language(&self.name);
        self.embed_args(&name, deps)
    }

    pub fn description(&self, deps: &Deps) -> String {
        let template = deps.get_language(&self.description);
        self.embed_args(&template, deps)
    }

    fn embed_args(&self, template: &str, deps: &Deps) -> String {
        let mut res = template.to_string();
        for (k, v) in self.vars.iter() {
            let v = deps.get_language(v);
            res = template.replace(&format!("|{k}|"), &v);
        }
        res
    }
}

fn load_sortie_modifiers<P: AsRef<Path>>(
    dir_path: P,
) -> BizResult<HashMap<String, SortieModifier>> {
    let file = dir_path.as_ref().join("sortie_modifiers.json");
    if file.exists() {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);
        let json: HashMap<String, SortieModifier> = serde_json::from_reader(&mut reader)?;
        Ok(json)
    } else {
        Ok(HashMap::new())
    }
}

/// mission types from
fn load_mission_types<P: AsRef<Path>>(dir: P) -> BizResult<MissionTypes> {
    let file = dir.as_ref().join("mission_types.json");
    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let mut reader = BufReader::new(&mut file);
        let res: Vec<MissionType> = serde_json::from_reader(&mut reader)?;
        Ok(MissionTypes::new(res))
        // Ok(res)
    } else {
        info!("no mission_types.json found, using default");
        Ok(MissionTypes::default())
    }
}

fn load_factions<P: AsRef<Path>>(dir: P) -> BizResult<HashMap<String, FactionInfo>> {
    let file = dir.as_ref().join("factions.json");
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
    // let mut res = HashMap::new();

    // regions from public export
    let regions_file = dir.as_ref().join("regions.json");
    if fs::exists(&regions_file)? {
        let mut file = File::open(regions_file)?;
        let mut reader = BufReader::new(&mut file);
        let json: ExportRegions = serde_json::from_reader(&mut reader)?;
        let res = json
            .regions
            .into_iter()
            .map(|k| (k.unique_name.clone(), k))
            .collect();
        Ok(res)
    } else {
        info!("no regions.json found!");
        Ok(HashMap::new())
    }
}

//  regions that not included in public export
// see https://raw.githubusercontent.com/calamity-inc/warframe-public-export-plus/refs/heads/senpai/ExportRegions.json
// but we did some changes
fn load_regions_ex<P: AsRef<Path>>(
    dir: P,
    lang_map: &HashMap<String, String>,
) -> BizResult<HashMap<String, Region>> {
    let ex_file = dir.as_ref().join("regions_ex.json");
    if fs::exists(&ex_file)? {
        let mut file = File::open(ex_file)?;
        let mut reader = BufReader::new(&mut file);
        let mut json: HashMap<String, Region> = serde_json::from_reader(&mut reader)?;
        for e in json.values_mut() {
            e.name = lang_map.get(&e.name).unwrap_or(&e.name).to_string();
            e.system_name = lang_map
                .get(&e.system_name)
                .unwrap_or(&e.system_name)
                .to_string();
        }
        Ok(json)
    } else {
        info!("no regions_ex.json found!");
        Ok(HashMap::new())
    }
    // Ok(res)
}

fn load_languages<P: AsRef<Path>>(dir: P) -> BizResult<HashMap<String, String>> {
    let file = dir.as_ref().join("lang.json");

    if fs::exists(&file)? {
        let mut file = File::open(file)?;
        let mut reader = BufReader::new(&mut file);
        let res: HashMap<String, String> = serde_json::from_reader(&mut reader)?;
        Ok(res)
    } else {
        info!("no lang.json found, using default");
        Ok(HashMap::new())
    }
}

fn load_steel_path<P: AsRef<Path>>(dir: P) -> BizResult<SteelPath> {
    let file = dir.as_ref().join("steel_path.json");
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

#[derive(Debug, Default)]
pub struct SortieBosses(HashMap<String, SortieBoss>);

impl Deref for SortieBosses {
    type Target = HashMap<String, SortieBoss>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct NWChallenge {
    #[serde(rename = "uniqueName")]
    unique_name: String,

    name: String,

    description: String,

    required: u32,

    standing: u32,
}

#[derive(Debug, Default)]
pub struct NWChallenges(HashMap<String, NWChallenge>);
impl Deref for NWChallenges {
    type Target = HashMap<String, NWChallenge>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct Deps {
    /// Mission types
    mission_types: MissionTypes,

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

    sortie_modifiers: HashMap<String, SortieModifier>,

    /// nightwave challenges
    /// this field is extracted from `pe_data`
    nightwave_challenges: NWChallenges,

    /// bundles in game
    bundles: HashMap<String, Bundle>,

    others: HashMap<String, OtherItem>,

    flavour_ex: HashMap<String, FlavourEx>,

    /// the circuit rewards map
    the_circuit: TheCircuit,

    conquests: Conquests,

    /// flag for initialization
    /// used to prevent reloading
    is_initialized: bool,
}

impl Deps {
    /// Get a language translation, return the key if no translation is found
    pub fn get_language(&self, key: &str) -> String {
        self.languages.get(key).cloned().unwrap_or_else(|| {
            warn!("no translation for {}", key);
            key.to_owned()
        })
    }

    pub fn get_nightwave_challenge(&self, key: &str) -> Option<&NWChallenge> {
        self.nightwave_challenges.get(key)
    }

    pub fn get_mission_type_by_tag(&self, key: &str) -> String {
        if let Some(mt) = self.mission_types.by_tag(key)
            && let Some(name) = mt.name.as_ref()
        {
            self.get_language(name)
        } else {
            warn!("no mission type for {}", key);
            key.to_owned()
        }
    }

    pub fn get_mission_type_by_id(&self, id: u8) -> String {
        if let Some(mt) = self.mission_types.by_id(id)
            && let Some(name) = mt.name.as_ref()
        {
            self.get_language(name)
        } else {
            "Unknown".to_owned()
        }
    }

    pub fn get_faction(&self, key: &str) -> String {
        if let Some(f) = self.factions.get(key) {
            f.name.clone()
        } else {
            warn!("no faction for {}", key);
            key.to_owned()
        }
    }

    /// Get a region info, return None if no region is found
    pub fn get_region(&self, key: &str) -> Option<&Region> {
        let region = self.regions.get(key);

        if region.is_none() {
            warn!("no region for {}", key);
        }
        region
    }

    /// Get a sortie boss name, return the key if no boss is found
    pub fn get_sortie_boss_name(&self, key: &str) -> String {
        if let Some(boss) = self.sortie_bosses.get(key) {
            boss.name.clone()
        } else {
            warn!("no sortie boss for {}", key);
            key.to_owned()
        }
    }

    pub fn get_sortie_modifier(&self, key: &str) -> Option<&SortieModifier> {
        self.sortie_modifiers.get(key)
    }

    fn find_bundle(&self, unique_name: &str) -> Option<Bundle> {
        self.bundles.get(unique_name).map(|b| Bundle {
            name: b.name.clone().map(|n| self.get_language(&n)),
            description: b.description.clone().map(|d| self.get_language(&d)),
        })
    }

    /// Find a game item name by unique name
    pub fn find_game_item(&self, unique_name: &str) -> String {
        let unique_name = if unique_name.starts_with("/Lotus/StoreItems") {
            unique_name.replace("/Lotus/StoreItems", "/Lotus")
        } else {
            unique_name.to_owned()
        };

        if let Some(f) = self.flavour_ex.get(&unique_name) {
            return self.get_language(&f.name);
        }

        if let Some(o) = self.others.get(&unique_name) {
            return o.name.clone();
        }

        // find in bundle first
        if let Some(b) = self.find_bundle(&unique_name) {
            // return b.name;
            if let Some(n) = b.name {
                return n;
            }
            warn!("bundle name not found for {}", unique_name);
            return unique_name;
        }

        let rei = self.pe_index.get(&unique_name);

        if let Some(rei) = rei
            && let Some(obj) = self.pe_data.get(&rei.category)
            && let Some(arr) = get_opt_array(obj, &rei.prop).unwrap()
            && let Some(item) = arr.get(rei.index)
        {
            let s = get_str(item, "name").unwrap();
            return s.to_owned();
        }
        warn!(unique_name, "item not found!");

        unique_name
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

    #[serde(rename = "missionIndex")]
    mission_index: u8,
    #[serde_as(as = "TryFromInto<u8>")]
    #[serde(rename = "factionIndex", default = "Default::default")]
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

#[derive(Debug, Deserialize, Clone)]
pub struct MissionType {
    id: u8,
    tag: String,
    name: Option<String>,
}

#[derive(Debug, Default)]
pub struct MissionTypes {
    pub mission_types: Vec<MissionType>,
    pub id_index: HashMap<u8, usize>,
    pub tag_index: HashMap<String, usize>,
}
impl MissionTypes {
    pub fn new(mission_types: Vec<MissionType>) -> Self {
        let index = mission_types
            .iter()
            .enumerate()
            .map(|(i, m)| (m.id, i))
            .collect::<HashMap<_, _>>();
        let tag_index = mission_types
            .iter()
            .enumerate()
            .map(|(i, m)| (m.tag.clone(), i))
            .collect::<HashMap<_, _>>();
        MissionTypes {
            mission_types,
            id_index: index,
            tag_index,
        }
    }

    pub fn by_id(&self, id: u8) -> Option<&MissionType> {
        self.id_index.get(&id).map(|i| &self.mission_types[*i])
    }

    pub fn by_tag(&self, tag: &str) -> Option<&MissionType> {
        self.tag_index.get(tag).map(|i| &self.mission_types[*i])
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum NodeType {
    Normal = 0,
    Arena,
    Pvp = 2,
    Hub = 3,
    DarkSector = 4,
    Room = 6,
    Junction = 7,
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
            1 => Ok(NodeType::Arena),
            2 => Ok(NodeType::Pvp),
            3 => Ok(NodeType::Hub),
            4 => Ok(NodeType::DarkSector),
            6 => Ok(NodeType::Room),
            7 => Ok(NodeType::Junction),
            _ => Err(ParseNodeTypeError(value)),
        }
    }
}

#[derive(Debug, Default)]
#[repr(u8)]
pub enum FactionIndex {
    /// 克隆尼 0
    Grineer,
    /// 科普斯 1
    Corpus,
    /// 感染者 2
    Infasted,
    /// 堕落者 3
    Orokin,
    /// 血色面纱 4
    RedVeil,
    /// 灵煞 5
    Sentient,
    /// 合一众 6
    Narmer,
    /// 低语者 7
    TheMurmur,
    /// 炽蛇军 8
    Scaldra,
    /// 科腐者 9
    Techrot,
    /// 禁卫 10 仅在双衍王境中出现
    Duviri,
    #[default]
    Unknown = 255,
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
            3 => Ok(FactionIndex::Orokin),
            4 => Ok(FactionIndex::RedVeil),
            5 => Ok(FactionIndex::Sentient),
            6 => Ok(FactionIndex::Narmer),
            7 => Ok(FactionIndex::TheMurmur),
            8 => Ok(FactionIndex::Scaldra),
            9 => Ok(FactionIndex::Techrot),
            10 => Ok(FactionIndex::Duviri),
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

        let mut file = File::open("./tests/samples/full/ws_wg_org-20251212.json").unwrap();
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
