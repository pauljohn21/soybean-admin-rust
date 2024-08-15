use sea_orm::EnumIter;
use sea_orm_migration::{
    prelude::{sea_query::extension::postgres::Type, *},
    sea_orm::{ConnectionTrait, DbBackend},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // let schema = Schema::new(DbBackend::Postgres);

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(Status::Enum)
                            .values([Status::ENABLED, Status::DISABLED, Status::BANNED])
                            .to_owned(),
                    )
                    .await?;

                // manager.create_type(schema.
                // create_enum_from_active_enum::<StatusEnum>()).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager.drop_type(Type::drop().name(Status::Enum).to_owned()).await?;
            }
        }

        Ok(())
    }
}

#[derive(DeriveIden, EnumIter)]
pub enum Status {
    // #[sea_orm(iden = "Status")]
    #[sea_orm(iden = "status")]
    Enum,
    #[sea_orm(iden = "enabled")]
    ENABLED,
    #[sea_orm(iden = "disabled")]
    DISABLED,
    #[sea_orm(iden = "banned")]
    BANNED,
}

// https://www.sea-ql.org/SeaORM/docs/generate-entity/enumeration/#postgres
// #[derive(EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "Status")]
// pub enum StatusEnum {
//     #[sea_orm(string_value = "enabled")]
//     ENABLED,
//     #[sea_orm(string_value = "disabled")]
//     DISABLED,
//     #[sea_orm(string_value = "banned")]
//     BANNED,
// }
