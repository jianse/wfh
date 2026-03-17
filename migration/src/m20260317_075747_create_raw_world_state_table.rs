use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(RawWorldState::Table)
                    .if_not_exists()
                    .col(pk_auto(RawWorldState::Id))
                    .col(date_time(RawWorldState::SyncTime))
                    .col(string_null(RawWorldState::Text))
                    .col(boolean(RawWorldState::IsSuccess).default(false))
                    .col(string_null(RawWorldState::ErrorMsg))
                    .col(unsigned(RawWorldState::TimeSpentMs))
                    .col(text_null(RawWorldState::Raw))
                    .col(text_null(RawWorldState::Warnings))
                    .col(date_time_null(RawWorldState::NearestExpiry))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RawWorldState::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RawWorldState {
    Table,
    Id,
    SyncTime,
    Text,
    IsSuccess,
    ErrorMsg,
    TimeSpentMs,
    Raw,
    Warnings,
    NearestExpiry,
}
