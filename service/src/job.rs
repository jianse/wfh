//! Background jobs

use crate::AppState;
use crate::api::world_state::{
    RewriteWarning, WorldStateData, WorldStateError, WorldStateOut, get_nearest_expiry,
    rewrite_world_state,
};
use crate::entities::prelude::WorldState;
use crate::entities::world_state;
use chrono::{DateTime, Local, TimeDelta, Utc};
use log::info;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
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

        // calc start up sleep time via precomputed expiry
        let mut d = match get_nearest_expiry_from_db(state.clone()).await {
            Ok(d) => match d {
                Some(next_time) => {
                    let dur = next_time - Utc::now();
                    if dur > TimeDelta::zero() {
                        dur.to_std().unwrap_or(MIN_INTERVAL_DURATION)
                    } else {
                        Duration::ZERO
                    }
                }
                None => Duration::ZERO,
            },
            Err(e) => {
                error!("{}", e);
                MIN_INTERVAL_DURATION
            }
        };

        loop {
            tokio::select! {
                _ = tokio::time::sleep(d) =>  (),
                _ = signal_tx.closed() => {
                    trace!("received signal");
                    break;
                },
            };
            match Self::job_body(state.clone(), &mut d).await {
                Ok(_) => {}
                Err(e) => {
                    error!("{}", e);
                    d = MIN_INTERVAL_DURATION;
                }
            }
        }

        trace!("waiting for pending job(s) to finish");
    }

    async fn job_body(state: AppState, d: &mut Duration) -> Result<(), DbErr> {
        let start = Instant::now();
        let res = crate::api::world_state::get_world_state(state.clone()).await;

        match res {
            Ok(ws) => {
                let WorldStateOut { data, raw } = ws;
                let nearest_expiry = get_nearest_expiry(&data);
                *d = match nearest_expiry {
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
                let (data, warn) = rewrite_world_state(state.clone(), data).await?;
                let spent = start.elapsed();
                write_world_state_into_db(state, raw, data, warn, spent, nearest_expiry).await?;
            }
            Err(err) => {
                let spent = start.elapsed();
                *d = MIN_INTERVAL_DURATION;
                log_error(state, err, spent).await?
            }
        };
        info!("next run could at {}", Local::now() + *d);
        Ok(())
    }
}

/// logging get warframe world state error into db.
async fn log_error(state: AppState, err: WorldStateError, spent: Duration) -> Result<(), DbErr> {
    error!("{}", err.err.to_string());
    let row = world_state::ActiveModel {
        sync_time: Set(Utc::now()),
        is_success: Set(false),
        error_msg: Set(Some(err.err.to_string())),
        time_spent_ms: Set(spent.as_millis() as u32),
        raw: Set(err.raw),
        ..Default::default()
    };
    WorldState::insert(row).exec(&state.conn).await?;
    Ok(())
}

/// save warframe world state into db
async fn write_world_state_into_db(
    state: AppState,
    raw: String,
    data: WorldStateData,
    warn: Option<RewriteWarning>,
    spent: Duration,
    nearest_expiry: Option<DateTime<Utc>>,
) -> Result<(), DbErr> {
    // TODO: remove unwrap
    let warn_str = serde_json::to_string(&warn).unwrap();
    let t = serde_json::to_string(&data).unwrap();
    // let t = brotli_compress(t).await.unwrap();
    let row = world_state::ActiveModel {
        id: NotSet,
        sync_time: Set(Utc::now()),
        text: Set(Some(t)),
        is_success: Set(true),
        error_msg: Set(None),
        time_spent_ms: Set(spent.as_millis() as u32),
        raw: Set(Some(raw)),
        warnings: Set(Some(warn_str)),
        nearest_expiry: Set(nearest_expiry),
    };
    WorldState::insert(row).exec(&state.conn).await?;
    Ok(())
}

/// get the last sync time
async fn get_nearest_expiry_from_db(state: AppState) -> Result<Option<DateTimeUtc>, DbErr> {
    let r = WorldState::find()
        .filter(world_state::Column::IsSuccess.eq(true))
        .order_by_desc(world_state::Column::SyncTime)
        .one(&state.conn)
        .await?;
    Ok(r.and_then(|m| m.nearest_expiry))
}


