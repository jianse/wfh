use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::update()
            .table(User::Table)
            .value(User::Role, "admin")
            .and_where(Expr::col(User::Id).eq(0))
            .to_owned();

        manager.exec_stmt(stmt).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Role,
}
