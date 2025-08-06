//! Background jobs

use crate::AppState;
use crate::api::world_state::get_world_state;
use crate::api::world_state::{
    RewriteWarning, WorldStateError, get_nearest_expiry, models::WorldStateData,
    rewrite_world_state,
};
use crate::api::{BizError, BizResult};
use crate::common::brotli_compress;
use crate::entities::prelude::WorldState2;
use crate::entities::world_state2;
use async_trait::async_trait;
use chrono::{DateTime, Local, Utc};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use settings::{JobRetryPolicy, JobSettings};
use std::cmp;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::watch;
use tokio::time::timeout;
use tracing::{error, info, trace};

type BoxJob = Box<dyn Job + Send + Sync>;

pub struct JobRegistry<S> {
    jobs: HashMap<&'static str, (JobConf, BoxJob)>,
    // state: AppState,
    signal: S,
    // broadcast: tokio::sync::broadcast::Sender<()>,
}
impl<S> JobRegistry<S>
where
    S: Future<Output = ()> + Send + 'static,
{
    pub fn new(state: AppState, signal: S) -> Self {
        let mut jobs = HashMap::new();

        // let (broadcast, rx1) = tokio::sync::broadcast::channel::<()>(16);
        let job = Box::new(WgWorldStateJob::new(state)).boxed();
        let conf = job_conf_for(job.name());

        jobs.insert(job.name(), (conf, job));
        info!("registered jobs: {:?}", jobs.keys());
        JobRegistry {
            jobs,
            // state,
            signal,
            // broadcast,
        }
    }

    pub async fn start(self) {
        let Self { jobs, signal, .. } = self;
        let (signal_tx, signal_rx) = tokio::sync::watch::channel(());
        // let
        tokio::spawn(async move {
            // pin!(signal);
            signal.await;
            trace!("received graceful shutdown signal");
            drop(signal_rx);
        });
        for (_, (conf, job)) in jobs {
            // skip disabled jobs
            if !conf.enabled {
                continue;
            }

            info!("starting job: {}", job.name());

            let tx = signal_tx.clone();
            tokio::spawn(async move {
                run_job(conf, job, tx).await;
            });
        }
    }
}

async fn run_job(conf: JobConf, job: BoxJob, tx: watch::Sender<()>) {
    let mut dur = job.bootstrap_run_duration().await;
    match job.job_type() {
        JobType::Periodic => {
            let mut retry_count = 0;

            loop {
                tokio::select! {
                    _ = tokio::time::sleep(dur) =>  (),
                    _ = tx.closed() => {
                        trace!("received signal");
                        break;
                    },
                };
                let r = with_timeout(&conf, &job).await;
                match r {
                    Ok(expect_dur) => {
                        if let Some(d) = expect_dur {
                            info!(
                                "{} job expected duration to next run is: {} secs",
                                job.name(),
                                d.as_secs()
                            );

                            dur = cmp::max(d, Duration::from_secs(conf.interval_secs));
                        } else {
                            info!(
                                "{} job not provide an expected duration to next run use default value: {} secs",
                                job.name(),
                                conf.interval_secs
                            );
                            dur = Duration::from_secs(conf.interval_secs);
                        }
                        retry_count = 0;
                    }
                    Err(e) => {
                        error!("run job failed: {e}");
                        if retry_count >= conf.max_retries {
                            error!("job {} failed, retry count exceeded", job.name());
                            dur = calc_dur(&conf, conf.max_retries);
                        } else {
                            retry_count += 1;
                            dur = calc_dur(&conf, retry_count);
                        }
                        error!("job {} failed, retrying...", job.name());
                    }
                };
                info!(
                    "next run will start at {}, after {} secs",
                    Local::now() + dur,
                    dur.as_secs()
                )
            }
        }
        JobType::OneTime => {
            // we do not have onetime job now!
            unimplemented!("one time job")
        }
    }
}

fn calc_dur(conf: &JobConf, retry: u32) -> Duration {
    match conf.retry_policy {
        JobRetryPolicy::Fixed => Duration::from_secs(conf.interval_secs),
        JobRetryPolicy::Exponential => Duration::from_secs(conf.interval_secs * 2_u64.pow(retry)),
        JobRetryPolicy::Linear => Duration::from_secs(conf.interval_secs * (retry as u64)),
    }
}

#[derive(Debug, Error)]
pub enum JobError {
    #[error("job \"{0}\" timeout")]
    Timeout(String),
    #[error("job \"{0}\" biz error: {1}")]
    Biz(String, BizError),
}

async fn with_timeout(conf: &JobConf, job: &BoxJob) -> Result<Option<Duration>, JobError> {
    let job_name = job.name().to_owned();
    timeout(Duration::from_secs(conf.timeout_secs), job.job_body())
        .await
        .map_err(|_e| JobError::Timeout(job_name.clone()))?
        .map_err(|e| JobError::Biz(job_name, e))
}

#[allow(unused)]
pub enum JobType {
    Periodic,
    OneTime,
}

#[async_trait]
pub trait Job {
    /// job's name
    fn name(&self) -> &'static str;

    /// get bootstrap run duration
    /// default is zero means run bootstrap immediately
    async fn bootstrap_run_duration(&self) -> Duration {
        Duration::ZERO
    }

    /// job running type (periodic or one time)
    fn job_type(&self) -> JobType;

    /// job body
    ///
    /// `duration` is the duration between two runs
    async fn job_body(&self) -> BizResult<Option<Duration>>;
}

pub struct WgWorldStateJob {
    state: AppState,
}

#[async_trait]
impl Job for WgWorldStateJob {
    fn name(&self) -> &'static str {
        "wg_world_state"
    }

    fn job_type(&self) -> JobType {
        JobType::Periodic
    }

    async fn bootstrap_run_duration(&self) -> Duration {
        let d = get_nearest_expiry_from_db(self.state.clone())
            .await
            .ok()
            .and_then(|s| s);
        match d {
            Some(t) => (t - Utc::now()).to_std().unwrap_or(Duration::ZERO),
            None => Duration::ZERO,
        }
    }

    async fn job_body(&self) -> BizResult<Option<Duration>> {
        let mut expected_duration = None;
        let start = Instant::now();
        let res = get_world_state(self.state.clone()).await;

        match res {
            Ok(ws) => {
                // let WorldStateOut { data, raw } = ws;
                // dbg!(&data);
                let nearest_expiry = get_nearest_expiry(&ws.data);
                // dbg!(&nearest_expiry);
                expected_duration = match nearest_expiry {
                    Some(next_time) => {
                        info!("next expiry: {}", next_time.with_timezone(&Local));
                        // let now = Utc::now();
                        // let next_min = now + MIN_INTERVAL;
                        // let nt = std::cmp::max(next_time, next_min);
                        (next_time - Utc::now()).to_std().ok()
                    }
                    None => None,
                };

                // .unwrap_or(MIN_INTERVAL_DURATION);
                let (data, warn) = rewrite_world_state(self.state.clone(), ws.data).await?;
                let spent = start.elapsed();
                write_world_state_into_db(
                    self.state.clone(),
                    ws.raw,
                    data,
                    warn,
                    spent,
                    nearest_expiry,
                )
                .await?;
            }
            Err(err) => {
                let spent = start.elapsed();
                // *d = MIN_INTERVAL_DURATION;
                log_error(self.state.clone(), err, spent).await?
            }
        };
        Ok(expected_duration)
    }
}

impl WgWorldStateJob {
    fn new(state: AppState) -> Self {
        WgWorldStateJob { state }
    }

    fn boxed(self) -> Box<dyn Job + Send + Sync> {
        Box::new(self)
    }
}

/// logging get warframe world state error into db.
async fn log_error(state: AppState, err: WorldStateError, spent: Duration) -> Result<(), BizError> {
    let raw = match err.raw {
        Some(raw) => {
            let t = brotli_compress(raw).await?;
            Some(t)
        }
        None => None,
    };
    error!("{}", err.err.to_string());
    let row = world_state2::ActiveModel {
        sync_time: Set(Utc::now()),
        is_success: Set(false),
        error_msg: Set(Some(err.err.to_string())),
        time_spent_ms: Set(spent.as_millis() as i32),
        raw: Set(raw),
        ..Default::default()
    };
    WorldState2::insert(row).exec(&state.conn).await?;
    Ok(())
}

/// save warframe world state into db
async fn write_world_state_into_db(
    state: AppState,
    raw: String,
    data: Box<WorldStateData>,
    warn: Option<RewriteWarning>,
    spent: Duration,
    nearest_expiry: Option<DateTime<Utc>>,
) -> BizResult<()> {
    // TODO: remove unwrap
    let warn_str = serde_json::to_string(&warn)?;
    let t = serde_json::to_string(&data)?;
    let t = brotli_compress(t).await?;

    let raw = brotli_compress(raw).await?;

    let row = world_state2::ActiveModel {
        id: NotSet,
        sync_time: Set(Utc::now()),
        text: Set(Some(t)),
        is_success: Set(true),
        error_msg: Set(None),
        time_spent_ms: Set(spent.as_millis() as i32),
        raw: Set(Some(raw)),
        warnings: Set(Some(warn_str)),
        nearest_expiry: Set(nearest_expiry),
    };
    WorldState2::insert(row).exec(&state.conn).await?;
    Ok(())
}

/// get the last sync time
async fn get_nearest_expiry_from_db(state: AppState) -> Result<Option<DateTimeUtc>, DbErr> {
    let r = WorldState2::find()
        .filter(world_state2::Column::IsSuccess.eq(true))
        .order_by_desc(world_state2::Column::SyncTime)
        .one(&state.conn)
        .await?;
    Ok(r.and_then(|m| m.nearest_expiry))
}

struct JobConf {
    pub interval_secs: u64,
    pub retry_policy: JobRetryPolicy,
    pub max_retries: u32,
    pub timeout_secs: u64,
    pub enabled: bool,
}

fn job_conf_for(name: &str) -> JobConf {
    let settings = settings::get();
    let JobSettings {
        interval_secs,
        retry_policy,
        max_retries,
        timeout_secs,
        overrides,
    } = settings.job.clone();

    match overrides.get(name) {
        Some(o) => JobConf {
            interval_secs: o.interval_secs.unwrap_or(interval_secs),
            retry_policy: o.retry_policy.unwrap_or(retry_policy),
            max_retries: o.max_retries.unwrap_or(max_retries),
            timeout_secs: o.interval_secs.unwrap_or(timeout_secs),
            enabled: o.enabled,
        },
        None => JobConf {
            interval_secs,
            retry_policy,
            max_retries,
            timeout_secs,
            enabled: true,
        },
    }
}
