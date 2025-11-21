pub mod error;
mod json_ops;
mod lang;

use crate::error::ExportError;
pub use crate::json_ops::*;
use crate::lang::LangHolder;
use async_compression::tokio::bufread::LzmaDecoder;
use chrono::DateTime;
use error::Result;
use reqwest::Client;
use reqwest::header::LAST_MODIFIED;
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;
use tracing::{debug, info, trace, warn};

pub struct PubExportFetcher<'c> {
    cache_dir: PathBuf,
    client: &'c Client,
}

impl<'a> PubExportFetcher<'a> {
    pub fn new<P: AsRef<Path>>(client: &'a Client, cache_dir: P) -> Self {
        Self {
            cache_dir: cache_dir.as_ref().to_path_buf(),
            client,
        }
    }
    fn load_cache_item(&self, name: &str) -> Result<Option<serde_json::Value>> {
        let path = self.cache_dir.join(name);
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            return Ok(Some(serde_json::from_str(&content)?));
        }

        Ok(None)
    }

    // #[instrument(skip_all)]
    async fn fetch_index(&mut self) -> Result<String> {
        let resp = self
            .client
            .get("https://origin.warframe.com/PublicExport/index_zh.txt.lzma")
            .send()
            .await?;

        let lm_str = resp.headers().get(LAST_MODIFIED).unwrap().to_str()?;
        let t = DateTime::parse_from_rfc2822(lm_str)?;

        let bytes = resp.bytes().await?;
        let mut txt = LzmaDecoder::new(bytes.as_ref());
        let mut buf = Vec::new();
        txt.read_to_end(&mut buf).await?;
        let content = String::from_utf8(buf)?;

        let index_dir = self.cache_dir.join("index");
        std::fs::create_dir_all(&index_dir)?;

        std::fs::write(
            index_dir.join(format!("index_{}.txt", t.timestamp())),
            &content,
        )?;
        info!("index_zh.txt.lzma downloaded");
        Ok(content)
    }

    async fn get_json(&mut self, part: &str) -> Result<serde_json::Value> {
        if let Some(json) = self.load_cache_item(part)? {
            return Ok(json.clone());
        }
        let resp = self
            .client
            .get(format!(
                "https://content.warframe.com/PublicExport/Manifest/{}",
                part
            ))
            .send()
            .await?;

        let json: serde_json::Value = resp.json().await?;
        // self.cache.insert(part.to_string(), json.clone());
        self.write_cache(part, &json)?;
        debug!("{} downloaded", part);
        Ok(json)
    }

    fn write_cache(&self, part: &str, data: &serde_json::Value) -> Result<()> {
        let path = self.cache_dir.join(part);
        std::fs::write(path, serde_json::to_string_pretty(&data)?)?;
        Ok(())
    }

    pub async fn fetch_all(&mut self) -> Result<HashMap<String, serde_json::Value>> {
        let index = self.fetch_index().await?;
        let mut res = HashMap::new();
        let map = to_category_map(&index);
        for (cat, name) in map.iter() {
            let json: serde_json::Value = self.get_json(name).await?;
            res.insert(cat.to_string(), json);
        }
        Ok(res)
    }
}

fn to_category_map(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let cat = name_to_category(line);
        map.insert(cat, line.to_string());
    }
    map
}

fn name_to_category(name: &str) -> String {
    let cat = name.replace("Export", "").replace("_zh", "");

    let i = cat.find(".json");
    match i {
        None => cat,
        Some(idx) => cat[..idx].to_string(),
    }
}
pub fn build_manifest_map(json: &serde_json::Value) -> Result<HashMap<String, String>> {
    let mfl = get_array(json, "Manifest")?;
    let mut map = HashMap::new();
    for item in mfl {
        let unique_name = get_str(item, "uniqueName")?;
        let name = get_str(item, "textureLocation")?;
        map.insert(unique_name.to_string(), name.to_string());
    }
    Ok(map)
}

#[derive(Debug, Serialize)]
pub struct ReIndex {
    pub category: String,
    pub prop: String,
    pub index: usize,
}

fn replace_lang_text(
    json: &mut serde_json::Value,
    lang_holder: &LangHolder,
    unique_name: &str,
    prop: &str,
) -> Result<()> {
    let prop_value = get_opt_str(json, prop)?;
    match prop_value {
        None => {
            trace!("{unique_name} has no property: {prop}");
        }
        Some(prop_value) => {
            if prop_value.is_empty() {
                trace!("{prop_value}({unique_name}) has empty {prop}");
            } else if let Some(text) = lang_holder.get(unique_name, prop) {
                set_value(json, prop, json!(text))?;
            }
        }
    }

    Ok(())
}

fn process_resources(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let resources = get_array_mut(json, "ExportResources")?;
    for (idx, item) in resources.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Resources".to_string(),
                prop: "ExportResources".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // replace description with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} resources processed", resources.len());
    Ok(())
}

pub fn save_all<P: AsRef<Path>>(pe: &HashMap<String, serde_json::Value>, dir: P) -> Result<()> {
    let dir = dir.as_ref();
    std::fs::create_dir_all(dir)?;
    for (cat, json) in pe {
        let path = dir.join(format!("{}.json", cat));
        std::fs::write(path, serde_json::to_string_pretty(&json)?)?;
    }
    info!("{} files saved", pe.len());
    Ok(())
}

fn process_gear(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let gear = get_array_mut(json, "ExportGear")?;
    for (idx, item) in gear.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Gear".to_string(),
                prop: "ExportGear".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // replace description with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} gear processed", gear.len());
    Ok(())
}

fn process_customs(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let customs = get_array_mut(json, "ExportCustoms")?;
    for (idx, item) in customs.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Customs".to_string(),
                prop: "ExportCustoms".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // replace description with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} customs processed", customs.len());
    Ok(())
}

fn process_warframes(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    // ExportAbilities
    let abilities = get_array_mut(json, "ExportAbilities")?;
    for (idx, item) in abilities.iter_mut().enumerate() {
        let unique_name = get_str(item, "abilityUniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Warframes".to_string(),
                prop: "ExportAbilities".to_string(),
                index: idx,
            },
        );

        // replace abilityName with translations
        replace_lang_text(item, lang_holder, &unique_name, "abilityName")?;

        // replace description with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // textureLocation
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} abilities processed", abilities.len());

    // ExportWarframes
    let warframes = get_array_mut(json, "ExportWarframes")?;
    for (idx, item) in warframes.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Warframes".to_string(),
                prop: "ExportWarframes".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // replace description with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // warframe abilities
        let abilities = get_array_mut(item, "abilities")?;
        for ability in abilities.iter_mut() {
            let a_unique_name = get_str(ability, "abilityUniqueName")?.to_string();

            // replace abilityName with translations
            replace_lang_text(ability, lang_holder, &a_unique_name, "abilityName")?;

            // replace description with translations
            replace_lang_text(ability, lang_holder, &a_unique_name, "description")?;

            // textureLocation
            if let Some(texture_location) = mf.get(&unique_name) {
                set_value(ability, "textureLocation", json!(texture_location))?;
            } else {
                info!("{} not found in manifest", unique_name);
            }
        }

        // passiveDescription
        replace_lang_text(item, lang_holder, &unique_name, "passiveDescription")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} warframes processed", warframes.len());
    Ok(())
}

fn process_weapons(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let railjack_weapons = get_array_mut(json, "ExportRailjackWeapons")?;
    for (idx, item) in railjack_weapons.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Weapons".to_string(),
                prop: "ExportRailjackWeapons".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // replace description with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // textureLocation
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} railjack weapons processed", railjack_weapons.len());

    let weapons = get_array_mut(json, "ExportWeapons")?;
    for (idx, item) in weapons.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Weapons".to_string(),
                prop: "ExportWeapons".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // replace description with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} weapons processed", weapons.len());
    Ok(())
}

fn process_upgrades(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    // railjack mods
    let avionics = get_array_mut(json, "ExportAvionics")?;
    for (idx, item) in avionics.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Upgrades".to_string(),
                prop: "ExportAvionics".to_string(),
                index: idx,
            },
        );
        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} avionics processed", avionics.len());

    let upgrades = get_array_mut(json, "ExportUpgrades")?;
    for (idx, item) in upgrades.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Upgrades".to_string(),
                prop: "ExportUpgrades".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // description
        // mod description is an array
        if let Some(des) = item.get_mut("description") {
            let descriptions = des.as_array_mut().unwrap();

            // let mut dl = Vec::new();

            for (idx, item) in descriptions.iter_mut().enumerate() {
                let description = item.as_str().unwrap();
                if description.is_empty() {
                    trace!(description, unique_name, "has empty description");
                } else if let Some(text) =
                    lang_holder.get(&unique_name, &format!("description[{idx}]"))
                {
                    // dl.push(text.clone());

                    *item = json!(text);
                }
            }
            // set_value(item, "descriptionLang", json!(dl))?;
        }

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} upgrades processed", upgrades.len());
    Ok(())
}

fn process_keys(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let keys = get_array_mut(json, "ExportKeys")?;
    for (idx, item) in keys.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Keys".to_string(),
                prop: "ExportKeys".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // description
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} keys processed", keys.len());
    Ok(())
}

fn process_drones(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let drones = get_array_mut(json, "ExportDrones")?;
    for (idx, item) in drones.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Drones".to_string(),
                prop: "ExportDrones".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // description
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} drones processed", drones.len());
    Ok(())
}
fn process_flavour(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let flavour = get_array_mut(json, "ExportFlavour")?;
    for (idx, item) in flavour.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Flavour".to_string(),
                prop: "ExportFlavour".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // description
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} flavour processed", flavour.len());
    Ok(())
}

fn process_sentinels(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let sentinels = get_array_mut(json, "ExportSentinels")?;
    for (idx, item) in sentinels.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Sentinels".to_string(),
                prop: "ExportSentinels".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // description
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} sentinels processed", sentinels.len());
    Ok(())
}

fn process_sortie_rewards(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let nightwave = get_mut(json, "ExportNightwave")?;

    let challenges = get_array_mut(nightwave, "challenges")?;

    for (idx, item) in challenges.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "SortieRewards".to_string(),
                prop: "ExportNightwave.challenges".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;

        // description
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} nightwave challenges processed", challenges.len());
    Ok(())
}

/// find item in pe by unique name
fn find_item<'a>(
    reverse_map: &HashMap<String, ReIndex>,
    pe: &'a HashMap<String, serde_json::Value>,
    unique_name: &str,
) -> Result<Option<&'a Value>> {
    let idx = reverse_map.get(unique_name).or_else(|| {
        let unique_name = unique_name.replace("/Lotus/StoreItems", "/Lotus");
        reverse_map.get(&unique_name)
    });

    if let Some(idx) = idx
        && let Some(cat) = pe.get(&idx.category)
    {
        let i = get_path(cat, &idx.prop)?;
        let r = i.as_array().and_then(|a| a.get(idx.index));
        return Ok(r);
    }
    Ok(None)
}

fn process_relic_arcane(
    pe: &mut HashMap<String, serde_json::Value>,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let mut json = pe
        .remove("RelicArcane")
        .ok_or_else(|| ExportError::NoSuchExport("RelicArcane".to_owned()))?;

    let relic_arcane = get_array_mut(&mut json, "ExportRelicArcane")?;

    for (idx, item) in relic_arcane.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();

        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "RelicArcane".to_string(),
                prop: "ExportRelicArcane".to_string(),
                index: idx,
            },
        );

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }

        if unique_name.starts_with("/Lotus/Types/Game/Projections") {
            // this is relic

            // description
            replace_lang_text(item, lang_holder, &unique_name, "description")?;

            // relic rewards
            let rewards = get_array_mut(item, "relicRewards")?;
            for item in rewards.iter_mut() {
                let reward_name = get_str(item, "rewardName")?.to_string();

                // find from reverse index
                if let Some(i) = find_item(reverse_map, pe, &reward_name)? {
                    let name = i.get("name");
                    if let Some(name) = name {
                        set_value(item, "name", json!(name))?;
                    } else {
                        debug!("no name for {reward_name}");
                    }
                }
            }
        } else if unique_name.starts_with("/Lotus/Upgrades/CosmeticEnhancers") {
            // this is arcane
            // replace name with translations
            replace_lang_text(item, lang_holder, &unique_name, "name")?;
        } else {
            warn!("unknown relic arcane type: {unique_name}")
        }
    }
    info!("{} relic arcane processed", relic_arcane.len());
    pe.insert("RelicArcane".to_string(), json);
    Ok(())
}

fn process_regions(
    json: &mut serde_json::Value,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let regions = get_array_mut(json, "ExportRegions")?;
    for (idx, item) in regions.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Regions".to_string(),
                prop: "ExportRegions".to_string(),
                index: idx,
            },
        );

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "name")?;
    }
    info!("{} regions processed", regions.len());
    Ok(())
}

fn process_fusion_bundles(
    json: &mut serde_json::Value,
    mf: &HashMap<String, String>,
    lang_holder: &LangHolder,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let fusion_bundles = get_array_mut(json, "ExportFusionBundles")?;
    for (idx, item) in fusion_bundles.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "FusionBundles".to_string(),
                prop: "ExportFusionBundles".to_string(),
                index: idx,
            },
        );

        // add name field
        if let Some(name) = lang_holder.get_raw("/Lotus/Language/Items/FusionBundle") {
            set_value(item, "name", json!(name))?;
        }

        // replace name with translations
        replace_lang_text(item, lang_holder, &unique_name, "description")?;

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }
    info!("{} fusion bundle processed", fusion_bundles.len());
    Ok(())
}

fn process_recipes(
    pe: &mut HashMap<String, serde_json::Value>,
    mf: &HashMap<String, String>,
    reverse_map: &mut HashMap<String, ReIndex>,
) -> Result<()> {
    let mut json = pe.remove("Recipes").unwrap();

    let recipes = get_array_mut(&mut json, "ExportRecipes")?;
    for (idx, item) in recipes.iter_mut().enumerate() {
        let unique_name = get_str(item, "uniqueName")?.to_string();
        // add to reverse index map
        reverse_map.insert(
            unique_name.to_string(),
            ReIndex {
                category: "Recipes".to_string(),
                prop: "ExportRecipes".to_string(),
                index: idx,
            },
        );

        // build name and description from its products
        let product_unique_name = get_str(item, "resultType")?;
        if let Some(idx) = reverse_map.get(product_unique_name) {
            set_value(item, "resultCategory", json!(idx.category))?;

            // unwrap on this is good
            let j = pe.get(&idx.category).unwrap();
            let arr = get_array(j, &idx.prop)?;
            let it = arr.get(idx.index).unwrap();
            // set_value(item, "resultItem", json!(it))?;

            let name = it
                .get("name")
                .and_then(|e| e.as_str().map(|e| format!("{e} 设计图")));
            set_value(item, "name", json!(name))?;

            let description = it.get("description");
            set_value(item, "description", json!(description))?;
        } else {
            debug!("no reverse index for {}", product_unique_name)
        }

        // process ingredients
        let ingredients = get_array_mut(item, "ingredients")?;
        for ing in ingredients.iter_mut() {
            let item_type = get_str(ing, "ItemType")?.to_string();
            if let Some(idx) = reverse_map.get(&item_type) {
                set_value(ing, "category", json!(idx.category))?;

                // unwrap on this is good
                let j = pe.get(&idx.category).unwrap();
                let arr = get_array(j, &idx.prop)?;
                let it = arr.get(idx.index).unwrap();

                let name = it.get("name");
                set_value(ing, "ItemName", json!(name))?;

                let description = it.get("description");
                set_value(ing, "ItemDescription", json!(description))?;
            }
        }

        // add textureLocation field
        if let Some(texture_location) = mf.get(&unique_name) {
            set_value(item, "textureLocation", json!(texture_location))?;
        } else {
            info!("{} not found in manifest", unique_name);
        }
    }

    info!("{} recipes processed", recipes.len());
    pe.insert("Recipes".to_string(), json);
    Ok(())
}

pub type ReverseMap = HashMap<String, ReIndex>;
pub type PublicExports = HashMap<String, serde_json::Value>;

pub struct PE<'c, 'l> {
    cache_dir: PathBuf,
    client: &'c Client,
    wire_file: PathBuf,
    lang_map: &'l HashMap<String, String>,
}
impl<'c, 'l> PE<'c, 'l> {
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(
        client: &'c Client,
        cache_dir: P1,
        wire_file: P2,
        lang_map: &'l HashMap<String, String>,
    ) -> Self {
        PE {
            client,
            cache_dir: cache_dir.as_ref().to_path_buf(),
            wire_file: wire_file.as_ref().to_path_buf(),
            lang_map,
        }
    }

    pub async fn fetch_transform_all(&self) -> Result<(PublicExports, ReverseMap)> {
        let mut fetcher = PubExportFetcher::new(self.client, &self.cache_dir);
        let mut pe = fetcher.fetch_all().await?;

        // Manifest contains all the images
        let mf = pe.get("Manifest").unwrap();
        let mf_kv = build_manifest_map(mf)?;

        let mut reverse_map = HashMap::new();

        let lang_holder = LangHolder::new(&self.wire_file, self.lang_map)?;

        let customs = pe.get_mut("Customs").unwrap();
        process_customs(customs, &mf_kv, &lang_holder, &mut reverse_map)?;

        let drones = pe.get_mut("Drones").unwrap();
        process_drones(drones, &mf_kv, &lang_holder, &mut reverse_map)?;

        let flavour = pe.get_mut("Flavour").unwrap();
        process_flavour(flavour, &mf_kv, &lang_holder, &mut reverse_map)?;

        let fusion_bundles = pe.get_mut("FusionBundles").unwrap();
        process_fusion_bundles(fusion_bundles, &mf_kv, &lang_holder, &mut reverse_map)?;

        let gear = pe.get_mut("Gear").unwrap();
        process_gear(gear, &mf_kv, &lang_holder, &mut reverse_map)?;

        let keys = pe.get_mut("Keys").unwrap();
        process_keys(keys, &mf_kv, &lang_holder, &mut reverse_map)?;

        let regions = pe.get_mut("Regions").unwrap();
        process_regions(regions, &lang_holder, &mut reverse_map)?;

        let resources = pe.get_mut("Resources").unwrap();
        process_resources(resources, &mf_kv, &lang_holder, &mut reverse_map)?;

        let sentinels = pe.get_mut("Sentinels").unwrap();
        process_sentinels(sentinels, &mf_kv, &lang_holder, &mut reverse_map)?;

        // process sortieRewards
        let sortie_rewards = pe.get_mut("SortieRewards").unwrap();
        process_sortie_rewards(sortie_rewards, &mf_kv, &lang_holder, &mut reverse_map)?;

        let upgrades = pe.get_mut("Upgrades").unwrap();
        process_upgrades(upgrades, &mf_kv, &lang_holder, &mut reverse_map)?;

        let warframes = pe.get_mut("Warframes").unwrap();
        process_warframes(warframes, &mf_kv, &lang_holder, &mut reverse_map)?;

        let weapons = pe.get_mut("Weapons").unwrap();
        process_weapons(weapons, &mf_kv, &lang_holder, &mut reverse_map)?;

        process_recipes(&mut pe, &mf_kv, &mut reverse_map)?;

        // process relic arcane
        process_relic_arcane(&mut pe, &mf_kv, &lang_holder, &mut reverse_map)?;

        Ok((pe, reverse_map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber;

    fn init_logger() {
        let _t = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
    }

    fn load_lang_map() -> eyre::Result<HashMap<String, String>> {
        let lang_map = std::fs::read("../data/lang.json")?;
        let lang_map: HashMap<String, String> = serde_json::from_slice(&lang_map)?;
        Ok(lang_map)
    }

    #[tokio::test]
    async fn test_fetch() -> eyre::Result<()> {
        init_logger();
        let client = reqwest::Client::new();
        let lang_map = load_lang_map()?;
        let pe = PE::new(&client, "../cache", "../data/wire.json", &lang_map);
        let (data, idx) = pe.fetch_transform_all().await?;

        save_all(&data, "../out")?;

        std::fs::write("../out/rindex.json", serde_json::to_string_pretty(&idx)?)?;
        Ok(())
    }

    #[test]
    fn test_load_lang() -> eyre::Result<()> {
        init_logger();
        // let _lang_map = load_lang_map()?;
        Ok(())
    }
}
