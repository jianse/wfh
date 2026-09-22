use crate::error::Result;
use std::collections::HashMap;
use std::path::Path;
use indexmap::IndexMap;
use tracing::trace;

pub(crate) struct LangHolder<'a> {
    lang_map: &'a IndexMap<String, String>,
    map_record: HashMap<String, HashMap<String, String>>,
}

impl<'a> LangHolder<'a> {
    pub(crate) fn new<P: AsRef<Path>>(
        record_path: P,
        lang_map: &'a IndexMap<String, String>,
    ) -> Result<Self> {
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

    pub(crate) fn get_raw(&self, text_key: &str) -> Option<&String> {
        self.lang_map.get(text_key)
    }
}
