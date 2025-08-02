//! Background jobs

use crate::AppState;
use crate::api::BizError;
use crate::api::world_state::{RewriteWarning, WorldStateData, rewrite_world_state};
use crate::entities::prelude::WorldState;
use crate::entities::world_state;
use chrono::{TimeDelta, Utc};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use std::time::{Duration, Instant};
use tracing::trace;

/// min interval between two syncs
const MIN_INTERVAL: TimeDelta = TimeDelta::seconds(60 * 10);

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

        // calc sleep time via last success time
        // and

        let mut d = match get_last_run_time(state.clone()).await {
            Some(last) => {
                let now = Utc::now();
                let dur = now - last;
                if dur < MIN_INTERVAL {
                    (MIN_INTERVAL - dur).to_std().unwrap()
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
            Ok(data) => {
                // TODO: modify world state
                let (data, warn) = rewrite_world_state(data);
                let spent = start.elapsed();
                write_world_state_into_db(state, data, warn, spent).await;
            }
            Err(err) => {
                let spent = start.elapsed();

                log_error(state, err, spent).await
            }
        };
        *d = MIN_INTERVAL.to_std().unwrap();
    }
}

/// logging get warframe world state error into db.
async fn log_error(state: AppState, err: BizError, spent: Duration) {
    let row = world_state::ActiveModel {
        id: NotSet,
        sync_time: Set(Utc::now()),
        text: NotSet,
        is_success: Set(false),
        error_msg: Set(Some(err.to_string())),
        time_spent_ms: Set(spent.as_millis() as u32),
    };
    WorldState::insert(row).exec(&state.conn).await.unwrap();
}

/// save warframe world state into db
async fn write_world_state_into_db(
    state: AppState,
    data: WorldStateData,
    _warn: Option<RewriteWarning>,
    spent: Duration,
) {
    // TODO: save warnings to db
    let t = serde_json::to_string(&data).unwrap();
    let row = world_state::ActiveModel {
        id: NotSet,
        sync_time: Set(Utc::now()),
        text: Set(Some(t)),
        is_success: Set(true),
        error_msg: Set(None),
        time_spent_ms: Set(spent.as_millis() as u32),
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
