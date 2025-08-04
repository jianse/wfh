use sea_orm_migration::prelude::*;

mod m20250524_000001_create_user_table;
mod m20250524_000002_create_admin_user;
mod m20250530_000003_create_translation_table;
mod m20250630_000004_add_user_role_column;
mod m20250630_110142_alter_user_0_role_to_admin;
mod m20250802_102116_add_world_state_table;
mod m20250803_043406_alter_world_state_table;
mod m20250804_101453_world_state_add_nearest_expiry;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250524_000001_create_user_table::Migration),
            Box::new(m20250524_000002_create_admin_user::Migration),
            Box::new(m20250530_000003_create_translation_table::Migration),
            Box::new(m20250630_000004_add_user_role_column::Migration),
            Box::new(m20250630_110142_alter_user_0_role_to_admin::Migration),
            Box::new(m20250802_102116_add_world_state_table::Migration),
            Box::new(m20250803_043406_alter_world_state_table::Migration),
            Box::new(m20250804_101453_world_state_add_nearest_expiry::Migration),
        ]
    }
}
