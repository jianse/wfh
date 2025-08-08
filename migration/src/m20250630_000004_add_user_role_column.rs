use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        if !manager
            .has_column(User::Table.to_string(), User::Role.to_string())
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(User::Table)
                        .add_column(
                            ColumnDef::new(User::Role)
                                .enumeration("user_role", ["admin", "normal"])
                                .not_null()
                                .default("normal"),
                        )
                        .to_owned(),
                )
                .await
        } else {
            Ok(())
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager
            .has_column(User::Table.to_string(), User::Role.to_string())
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(User::Table)
                        .drop_column(User::Role)
                        .to_owned(),
                )
                .await
        } else {
            Ok(())
        }
    }
}

#[derive(Iden)]
enum User {
    Table,
    Role,
}
