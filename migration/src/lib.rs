pub use sea_orm_migration::prelude::*;

mod schemas;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(schemas::m20240815_082808_create_enum_status::Migration),
            Box::new(schemas::m20240815_082854_create_sys_user::Migration),
            Box::new(schemas::m20241023_091143_create_sys_menu::Migration),
            Box::new(schemas::m20241023_091155_create_sys_organization::Migration),
            Box::new(schemas::m20241023_091109_create_sys_access_key::Migration),
            Box::new(schemas::m20241023_091115_create_sys_domain::Migration),
            Box::new(schemas::m20241023_091132_create_sys_endpoint::Migration),
            Box::new(schemas::m20241023_091138_create_sys_login_log::Migration),
            Box::new(schemas::m20241023_091149_create_sys_operation_log::Migration),
            Box::new(schemas::m20241023_090604_create_sys_role::Migration),
            Box::new(schemas::m20241023_091204_create_sys_tokens::Migration),
            Box::new(schemas::m20241023_091210_create_sys_user_role::Migration),
            Box::new(schemas::m20241023_091159_create_sys_role_menu::Migration),
        ]
    }
}
