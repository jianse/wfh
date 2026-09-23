use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;

use async_trait::async_trait;

use super::Job;
use super::JobType;
use crate::AppState;
use crate::api::BizError;
use crate::api::BizResult;

/// How often the language table is refreshed from the CDN.
const SYNC_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

/// Sub-directory (inside the deps cache dir) used to cache the game resources.
const RESOURCE_CACHE_DIR: &str = "client";

/// Periodically rebuilds `data/lang.json` from the game resources on the CDN.
pub struct LangSyncJob {
    state: AppState,
}

#[async_trait]
impl Job for LangSyncJob {
    fn name(&self) -> &'static str {
        "lang_sync"
    }

    fn job_type(&self) -> JobType {
        JobType::Periodic
    }

    async fn bootstrap_run_duration(&self) -> Duration {
        // run immediately when we have never synced, otherwise wait until the
        // most recent sync in `cache/client` becomes `SYNC_INTERVAL` old
        let cache_dir = self.state.deps.cache_dir().join(RESOURCE_CACHE_DIR);
        match latest_modified(&cache_dir) {
            Some(mtime) => {
                let elapsed = SystemTime::now()
                    .duration_since(mtime)
                    .unwrap_or(Duration::ZERO);
                SYNC_INTERVAL.saturating_sub(elapsed)
            }
            None => Duration::ZERO,
        }
    }

    async fn job_body(&self) -> BizResult<Option<Duration>> {
        let cache_dir = self.state.deps.cache_dir().join(RESOURCE_CACHE_DIR);

        let langs = wf_cache::fetch_languages(&self.state.http_client, &cache_dir)
            .await
            .map_err(|e| BizError::custom(format!("fetch languages failed: {e}")))?;

        if langs.is_empty() {
            return Err(BizError::custom("parsed an empty language table"));
        }

        self.state.deps.replace_languages(langs).await?;

        Ok(Some(SYNC_INTERVAL))
    }
}

impl LangSyncJob {
    pub(super) fn new(state: AppState) -> Self {
        Self { state }
    }

    pub(super) fn boxed(self) -> Box<dyn Job + Send + Sync> {
        Box::new(self)
    }
}

/// Most recent modification time in `dir`, or `None` when the directory does
/// not exist or is empty. Used as the "last sync time" of the cached resources.
fn latest_modified(dir: &Path) -> Option<SystemTime> {
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.metadata().ok())
        .filter_map(|meta| meta.modified().ok())
        .max()
}
