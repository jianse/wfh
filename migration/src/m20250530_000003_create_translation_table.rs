use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250530_000003_create_translation_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Translation::Table)
                    .col(
                        ColumnDef::new(Translation::UniqueName).text().not_null(), // .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Translation::Prop)
                            .char()
                            .char_len(32)
                            .not_null(), // .primary_key(),
                    )
                    .col(ColumnDef::new(Translation::Content).text().not_null())
                    .primary_key(
                        sea_query::Index::create()
                            .col(Translation::UniqueName)
                            .col(Translation::Prop),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Translation::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Translation {
    Table,
    UniqueName,
    Prop,
    Content,
}
