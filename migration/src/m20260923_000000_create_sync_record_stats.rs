use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const IDX_RAW_SYNC_TIME: &str = "idx-raw-world-state-sync-time-desc";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // single row aggregate table (id is always 1) so sync statistics can be
        // served in O(1) instead of scanning the ever growing raw_world_state table.
        manager
            .create_table(
                Table::create()
                    .table(SyncRecordStats::Table)
                    .if_not_exists()
                    .col(integer(SyncRecordStats::Id).primary_key())
                    .col(big_integer(SyncRecordStats::TotalCount))
                    .col(big_integer(SyncRecordStats::SuccessCount))
                    .col(big_integer(SyncRecordStats::FailCount))
                    .col(big_integer(SyncRecordStats::TotalTimeMs))
                    .col(integer_null(SyncRecordStats::MinTimeMs))
                    .col(integer_null(SyncRecordStats::MaxTimeMs))
                    .col(date_time_null(SyncRecordStats::LastSyncTime))
                    .col(date_time_null(SyncRecordStats::LastSuccessTime))
                    .col(date_time_null(SyncRecordStats::LastFailTime))
                    .col(text_null(SyncRecordStats::LastErrorMsg))
                    .col(date_time_null(SyncRecordStats::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // seed the aggregate from all existing sync records. `INSERT OR IGNORE`
        // keeps the migration idempotent. An empty table still yields one row
        // (all aggregates default to 0 / NULL).
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT OR IGNORE INTO "sync_record_stats" (
                    "id", "total_count", "success_count", "fail_count", "total_time_ms",
                    "min_time_ms", "max_time_ms", "last_sync_time", "last_success_time",
                    "last_fail_time", "last_error_msg", "updated_at"
                )
                SELECT
                    1,
                    COUNT(*),
                    COALESCE(SUM(CASE WHEN "is_success" THEN 1 ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN "is_success" THEN 0 ELSE 1 END), 0),
                    COALESCE(SUM("time_spent_ms"), 0),
                    MIN("time_spent_ms"),
                    MAX("time_spent_ms"),
                    MAX("sync_time"),
                    MAX(CASE WHEN "is_success" THEN "sync_time" END),
                    MAX(CASE WHEN "is_success" THEN NULL ELSE "sync_time" END),
                    (SELECT "error_msg" FROM "raw_world_state" WHERE NOT "is_success" ORDER BY "sync_time" DESC LIMIT 1),
                    MAX("sync_time")
                FROM "raw_world_state"
                "#,
            )
            .await?;

        // ordering / "latest record" lookups on raw_world_state
        manager
            .create_index(
                Index::create()
                    .name(IDX_RAW_SYNC_TIME)
                    .table(RawWorldState::Table)
                    .col((RawWorldState::SyncTime, IndexOrder::Desc))
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name(IDX_RAW_SYNC_TIME)
                    .table(RawWorldState::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(SyncRecordStats::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SyncRecordStats {
    Table,
    Id,
    TotalCount,
    SuccessCount,
    FailCount,
    TotalTimeMs,
    MinTimeMs,
    MaxTimeMs,
    LastSyncTime,
    LastSuccessTime,
    LastFailTime,
    LastErrorMsg,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RawWorldState {
    Table,
    SyncTime,
}
