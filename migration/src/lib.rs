pub use sea_orm_migration::prelude::*;

mod migrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations::m20240815_082808_create_enum_status::Migration),
            Box::new(migrations::m20240815_082854_create_sys_user::Migration),
        ]
    }
}
