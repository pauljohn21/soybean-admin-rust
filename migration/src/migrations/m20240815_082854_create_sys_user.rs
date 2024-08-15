use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use super::m20240815_082808_create_enum_status::Status;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建表
        manager
            .create_table(
                Table::create()
                    .table(SysUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SysUser::Id).big_integer().auto_increment().primary_key())
                    .col(ColumnDef::new(SysUser::DomainId).big_integer().not_null().comment("域ID"))
                    .col(ColumnDef::new(SysUser::OrgId).big_integer().comment("组织ID"))
                    .col(
                        ColumnDef::new(SysUser::Username)
                            .string_len(64)
                            .not_null()
                            .comment("用户名"),
                    )
                    .col(
                        ColumnDef::new(SysUser::Password)
                            .string_len(255)
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(SysUser::NickName).string_len(64).not_null().comment("昵称"),
                    )
                    .col(ColumnDef::new(SysUser::Avatar).string_len(255).comment("头像"))
                    .col(ColumnDef::new(SysUser::Email).string_len(64).comment("邮箱"))
                    .col(ColumnDef::new(SysUser::Phone).string_len(64).comment("手机号"))
                    .col(
                        ColumnDef::new(SysUser::Status)
                            .enumeration(Status::Enum, Status::iter())
                            .not_null()
                            .comment("用户状态"),
                    )
                    .col(ColumnDef::new(SysUser::CreatedAt).timestamp())
                    .col(ColumnDef::new(SysUser::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .table(SysUser::Table)
                    .name("idx_sys_user_domain_id")
                    .col(SysUser::DomainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(SysUser::Table)
                    .name("idx_sys_user_username")
                    .col(SysUser::Username)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(SysUser::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum SysUser {
    Table,
    Id,
    DomainId,
    OrgId,
    Username,
    Password,
    NickName,
    Avatar,
    Email,
    Phone,
    Status,
    CreatedAt,
    UpdatedAt,
}
