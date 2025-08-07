use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WgWorldStateReplacements::Table)
                    .col(string(WgWorldStateReplacements::Path))
                    .col(string(WgWorldStateReplacements::Origin))
                    .col(string_null(WgWorldStateReplacements::Replacement))
                    .col(string_null(WgWorldStateReplacements::Remark))
                    .primary_key(
                        sea_query::Index::create()
                            .col(WgWorldStateReplacements::Path)
                            .col(WgWorldStateReplacements::Origin),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(WgWorldStateReplacements::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum WgWorldStateReplacements {
    Table,
    Path,
    Origin,
    Replacement,
    Remark,
}
