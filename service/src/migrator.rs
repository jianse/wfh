use sea_orm_migration::prelude::*;

mod m20250524_000001_create_user_table;
mod m20250524_000002_create_admin_user;
mod m20250530_000001_create_translation_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250524_000001_create_user_table::Migration),
            Box::new(m20250524_000002_create_admin_user::Migration),
            Box::new(m20250530_000001_create_translation_table::Migration),
        ]
    }
}
