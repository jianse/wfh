use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorldState::Table)
                    .if_not_exists()
                    .col(pk_auto(WorldState::Id))
                    .col(date_time(WorldState::SyncTime))
                    .col(string_null(WorldState::Text))
                    .col(boolean(WorldState::IsSuccess))
                    .col(string_null(WorldState::ErrorMsg))
                    .col(unsigned(WorldState::TimeSpentMs))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorldState::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WorldState {
    Table,
    Id,
    SyncTime,
    Text,
    IsSuccess,
    ErrorMsg,
    TimeSpentMs,
}
