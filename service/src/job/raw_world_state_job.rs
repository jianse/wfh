use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;

use super::Job;
use super::JobType;
use crate::AppState;
use crate::api::BizResult;
use crate::api::raw_world_state::get_nearest_expiry_from_db;
use crate::api::raw_world_state::sync_raw_world_state;

pub struct RawWorldStateJob {
    state: AppState,
}

#[async_trait]
impl Job for RawWorldStateJob {
    fn name(&self) -> &'static str {
        "raw_world_state"
    }

    fn job_type(&self) -> JobType {
        JobType::Periodic
    }

    async fn bootstrap_run_duration(&self) -> Duration {
        let bootstrap_time = get_nearest_expiry_from_db(self.state.clone())
            .await
            .ok()
            .flatten()
            .and_then(|t| (t - Utc::now()).to_std().ok())
            .unwrap_or(Duration::ZERO);
        #[cfg(debug_assertions)]
        {
            dbg!(bootstrap_time);
        }
        bootstrap_time
    }

    async fn job_body(&self) -> BizResult<Option<Duration>> {
        let exp = sync_raw_world_state(self.state.clone()).await?;
        let r = exp.and_then(|t| (t - Utc::now()).to_std().ok());
        #[cfg(debug_assertions)]
        {
            dbg!(r);
        }
        Ok(r)
    }
}

impl RawWorldStateJob {
    pub(super) fn new(state: AppState) -> Self {
        RawWorldStateJob { state }
    }

    pub(super) fn boxed(self) -> Box<dyn Job + Send + Sync> {
        Box::new(self)
    }
}
