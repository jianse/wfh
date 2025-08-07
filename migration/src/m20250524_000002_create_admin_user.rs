use rand::{distr::Alphanumeric, rng, Rng};
use sea_orm_migration::prelude::*;
use tracing::info;

use super::m20250524_000001_create_user_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250524_000002_create_admin_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let rand_string: String = rng()
            .sample_iter(&Alphanumeric)
            .take(18)
            .map(char::from)
            .collect();

        let encrypted = settings::get().auth.pwd_hasher.hash(&rand_string).unwrap();

        let insert = Query::insert()
            .into_table(User::Table)
            .columns([User::Id, User::Username, User::Password])
            .values_panic([0.into(), "admin".into(), encrypted.into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        info!("created user \"admin\" with password {rand_string}");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(User::Table)
            .cond_where(Cond::any().add(Expr::col(User::Id).eq(0)))
            .to_owned();
        manager.exec_stmt(delete).await
    }
}
