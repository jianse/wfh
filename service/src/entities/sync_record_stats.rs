//! `SeaORM` Entity for the single-row sync statistics aggregate.

use sea_orm::entity::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sync_record_stats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub total_count: i64,
    pub success_count: i64,
    pub fail_count: i64,
    pub total_time_ms: i64,
    pub min_time_ms: Option<i32>,
    pub max_time_ms: Option<i32>,
    pub last_sync_time: Option<DateTimeUtc>,
    pub last_success_time: Option<DateTimeUtc>,
    pub last_fail_time: Option<DateTimeUtc>,
    #[sea_orm(column_type = "Text", nullable)]
    pub last_error_msg: Option<String>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
