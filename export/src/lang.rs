use crate::error::Result;
use tracing::trace;
use std::collections::HashMap;
use std::path::Path;

pub(crate) struct LangHolder {
    lang_map: HashMap<String, String>,
    map_record: HashMap<String, HashMap<String, String>>,
}

impl LangHolder {
    pub(crate) fn new<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let base_path = dir.as_ref();
        let lang_file = base_path.join("lang.json");
        let buf = std::fs::read(lang_file)?;
        let lang_map = serde_json::from_slice(&buf)?;

        let record_path = base_path.join("wire.json");
        let buf = std::fs::read(record_path)?;
        let map_record = serde_json::from_slice(&buf)?;

        Ok(LangHolder {
            lang_map,
            map_record,
        })
    }

    pub(crate) fn get(&self, unique_name: &str, prop: &str) -> Option<&String> {
        self.map_record
            .get(unique_name)
            .and_then(|map| map.get(prop))
            .and_then(|k| {
                if k == "[?]" {
                    trace!("{unique_name}, {prop} has not map");
                }
                self.lang_map.get(k)
            })
    }
}
