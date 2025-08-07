use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const IDX1: &str = "idx-world-state-sync-time-desc";
const IDX2: &str = "idx-world-state-is-success";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // add column nearest_expiry
        if !manager
            .has_column(
                WorldState::Table.to_string(),
                WorldState::NearestExpiry.to_string(),
            )
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(WorldState::Table)
                        .add_column(date_time_null(WorldState::NearestExpiry))
                        .to_owned(),
                )
                .await?;
        }

        // add index sync_time
        manager
            .create_index(
                Index::create()
                    .name(IDX1)
                    .table(WorldState::Table)
                    .col((WorldState::SyncTime, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        // add index is_success
        manager
            .create_index(
                Index::create()
                    .name(IDX2)
                    .table(WorldState::Table)
                    .col(WorldState::IsSuccess)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager
            .has_column(
                WorldState::Table.to_string(),
                WorldState::NearestExpiry.to_string(),
            )
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(WorldState::Table)
                        .drop_column(WorldState::NearestExpiry)
                        .to_owned(),
                )
                .await?;
        }

        manager
            .drop_index(Index::drop().name(IDX1).table(WorldState::Table).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name(IDX2).table(WorldState::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum WorldState {
    Table,
    SyncTime,
    IsSuccess,
    NearestExpiry,
}
