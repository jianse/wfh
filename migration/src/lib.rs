pub use sea_orm_migration::prelude::*;

pub use sea_orm_migration::MigrationStatus;

mod m20250524_000001_create_user_table;
mod m20250524_000002_create_admin_user;
mod m20250530_000003_create_translation_table;
mod m20250630_000004_add_user_role_column;
mod m20250630_110142_alter_user_0_role_to_admin;
mod m20250802_102116_add_world_state_table;
mod m20250803_043406_alter_world_state_table;
mod m20250804_101453_world_state_add_nearest_expiry;
mod m20250804_110641_create_table_wg_world_state_replacements;
mod m20250805_132203_create_world_state_2_table;
mod m20260208_101302_create_user_device_table;
mod m20260317_075747_create_raw_world_state_table;
mod m20260923_000000_create_sync_record_stats;

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
            Box::new(m20250804_110641_create_table_wg_world_state_replacements::Migration),
            Box::new(m20250805_132203_create_world_state_2_table::Migration),
            Box::new(m20260208_101302_create_user_device_table::Migration),
            Box::new(m20260317_075747_create_raw_world_state_table::Migration),
            Box::new(m20260923_000000_create_sync_record_stats::Migration),
        ]
    }
}
