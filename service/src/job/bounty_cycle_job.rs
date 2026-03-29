use std::time::Duration;

use async_trait::async_trait;

use super::Job;
use super::JobType;
use crate::AppState;
use crate::api::BizResult;
use crate::api::raw_world_state::download_bounty_cycle;
use crate::api::raw_world_state::get_bounty_cycle_expiry;

pub struct BountyCycleJob {
    state: AppState,
}

#[async_trait]
impl Job for BountyCycleJob {
    fn name(&self) -> &'static str {
        "bounty_cycle"
    }

    fn job_type(&self) -> JobType {
        JobType::Periodic
    }

    async fn bootstrap_run_duration(&self) -> Duration {
        let bootstrap_time = get_bounty_cycle_expiry()
            .ok()
            .flatten()
            .unwrap_or(Duration::ZERO);
        #[cfg(debug_assertions)]
        {
            dbg!(bootstrap_time);
        }
        bootstrap_time
    }

    async fn job_body(&self) -> BizResult<Option<Duration>> {
        let r = download_bounty_cycle(self.state.clone()).await?;
        #[cfg(debug_assertions)]
        {
            dbg!(r);
        }
        Ok(r)
    }
}

impl BountyCycleJob {
    pub(super) fn new(state: AppState) -> Self {
        BountyCycleJob { state }
    }

    pub(super) fn boxed(self) -> Box<dyn Job + Send + Sync> {
        Box::new(self)
    }
}
