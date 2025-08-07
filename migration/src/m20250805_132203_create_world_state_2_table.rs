use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const IDX1: &str = "idx-world-state-2-sync-time-desc";
const IDX2: &str = "idx-world-state-2-is-success";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorldState2::Table)
                    .col(integer(WorldState2::Id).primary_key().auto_increment())
                    .col(date_time(WorldState2::SyncTime))
                    .col(blob_null(WorldState2::Text))
                    .col(boolean(WorldState2::IsSuccess))
                    .col(string_null(WorldState2::ErrorMsg))
                    .col(integer(WorldState2::TimeSpentMs))
                    .col(blob_null(WorldState2::Raw))
                    .col(string_null(WorldState2::Warnings))
                    .col(date_time_null(WorldState2::NearestExpiry))
                    .to_owned(),
            )
            .await?;
        // add index sync_time
        manager
            .create_index(
                Index::create()
                    .name(IDX1)
                    .table(WorldState2::Table)
                    .col((WorldState2::SyncTime, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        // add index is_success
        manager
            .create_index(
                Index::create()
                    .name(IDX2)
                    .table(WorldState2::Table)
                    .col(WorldState2::IsSuccess)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorldState2::Table).to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name(IDX1)
                    .table(WorldState2::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name(IDX2)
                    .table(WorldState2::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum WorldState2 {
    Table,
    Id,
    SyncTime,
    Text,
    IsSuccess,
    ErrorMsg,
    TimeSpentMs,
    NearestExpiry,
    Raw,
    Warnings,
}
