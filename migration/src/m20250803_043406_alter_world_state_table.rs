use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager
            .has_column(WorldState::Table.to_string(), WorldState::Raw.to_string())
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(WorldState::Table)
                        .add_column(string_null(WorldState::Raw))
                        .to_owned(),
                )
                .await?;
        }
        if !manager
            .has_column(
                WorldState::Table.to_string(),
                WorldState::Warnings.to_string(),
            )
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(WorldState::Table)
                        .add_column(string_null(WorldState::Warnings))
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager
            .has_column(WorldState::Table.to_string(), WorldState::Raw.to_string())
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(WorldState::Table)
                        .drop_column(WorldState::Raw)
                        .to_owned(),
                )
                .await?;
        }
        if !manager
            .has_column(
                WorldState::Table.to_string(),
                WorldState::Warnings.to_string(),
            )
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(WorldState::Table)
                        .drop_column(WorldState::Warnings)
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
enum WorldState {
    Table,
    Raw,
    Warnings,
}
