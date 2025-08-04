//! Background jobs

use crate::AppState;
use crate::api::world_state::{
    RewriteWarning, WorldStateData, WorldStateError, WorldStateOut, get_nearest_expiry,
    rewrite_world_state,
};
use crate::entities::prelude::WorldState;
use crate::entities::world_state;
use chrono::{Local, TimeDelta, Utc};
use log::info;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use std::time::{Duration, Instant};
use tracing::{error, trace};

/// min interval between two syncs
/// one minute
const MIN_INTERVAL: TimeDelta = TimeDelta::seconds(60);
/// min interval between two syncs but in [std::time::Duration] type
/// NOTE: this value should be equal to [MIN_INTERVAL]
const MIN_INTERVAL_DURATION: Duration = Duration::from_secs(60);

pub struct WgWorldStateJob<S> {
    state: AppState,
    signal: S,
}

impl<S> WgWorldStateJob<S>
where
    S: Future<Output = ()> + Send + 'static,
{
    pub fn new(state: AppState, signal: S) -> Self {
        WgWorldStateJob { state, signal }
    }

    pub async fn run(self) {
        let Self { state, signal } = self;
        let (signal_tx, signal_rx) = tokio::sync::watch::channel(());
        tokio::spawn(async move {
            // pin!(signal);
            signal.await;
            trace!("received graceful shutdown signal");
            drop(signal_rx);
        });

        // calc start up sleep time via last success time
        // TODO: write nearest expires time into db. so we can get bootstrap sleep time directly
        let mut d = match get_last_run_time(state.clone()).await {
            Some(last) => {
                let now = Utc::now();
                let dur = now - last;
                if dur < MIN_INTERVAL {
                    (MIN_INTERVAL - dur)
                        .to_std()
                        .unwrap_or(MIN_INTERVAL_DURATION)
                } else {
                    Duration::ZERO
                }
            }
            None => Duration::ZERO,
        };

        loop {
            tokio::select! {
                _ = tokio::time::sleep(d) =>  (),
                _ = signal_tx.closed() => {
                    trace!("received signal");
                    break;
                },
            };
            Self::job_body(state.clone(), &mut d).await
        }
        // drop(close_rx);

        trace!("waiting for pending job(s) to finish");
    }

    async fn job_body(state: AppState, d: &mut Duration) -> () {
        let start = Instant::now();
        let res = crate::api::world_state::get_world_state(state.clone()).await;

        match res {
            Ok(ws) => {
                let WorldStateOut { data, raw } = ws;
                *d = match get_nearest_expiry(&data) {
                    Some(next_time) => {
                        info!("next expiry: {}", next_time.with_timezone(&Local));
                        let now = Utc::now();
                        let next_min = now + MIN_INTERVAL;
                        let nt = std::cmp::max(next_time, next_min);
                        nt - Utc::now()
                    }
                    None => MIN_INTERVAL,
                }
                .to_std()
                .unwrap_or(MIN_INTERVAL_DURATION);
                let (data, warn) = rewrite_world_state(data);
                let spent = start.elapsed();
                write_world_state_into_db(state, raw, data, warn, spent).await;
            }
            Err(err) => {
                let spent = start.elapsed();
                *d = MIN_INTERVAL_DURATION;
                log_error(state, err, spent).await
            }
        };
        info!("next run could at {}", Local::now() + *d);
    }
}

/// logging get warframe world state error into db.
async fn log_error(state: AppState, err: WorldStateError, spent: Duration) {
    error!("{}", err.err.to_string());
    let row = world_state::ActiveModel {
        id: NotSet,
        sync_time: Set(Utc::now()),
        text: NotSet,
        is_success: Set(false),
        error_msg: Set(Some(err.err.to_string())),
        time_spent_ms: Set(spent.as_millis() as u32),
        raw: Set(err.raw),
        warnings: NotSet,
    };
    WorldState::insert(row).exec(&state.conn).await.unwrap();
}

/// save warframe world state into db
async fn write_world_state_into_db(
    state: AppState,
    raw: String,
    data: WorldStateData,
    warn: Option<RewriteWarning>,
    spent: Duration,
) {
    let warn_str = serde_json::to_string(&warn).unwrap();
    let t = serde_json::to_string(&data).unwrap();
    let row = world_state::ActiveModel {
        id: NotSet,
        sync_time: Set(Utc::now()),
        text: Set(Some(t)),
        is_success: Set(true),
        error_msg: Set(None),
        time_spent_ms: Set(spent.as_millis() as u32),
        raw: Set(Some(raw)),
        warnings: Set(Some(warn_str)),
    };
    WorldState::insert(row).exec(&state.conn).await.unwrap();
}

/// get the last sync time
async fn get_last_run_time(state: AppState) -> Option<DateTimeUtc> {
    let r = WorldState::find()
        .filter(world_state::Column::IsSuccess.eq(true))
        .order_by_desc(world_state::Column::SyncTime)
        .one(&state.conn)
        .await
        .unwrap();
    r.map(|m| m.sync_time)
}
