use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{prelude::SysRole, sys_role},
    input::{CreateRoleInput, RolePageRequest, UpdateRoleInput},
};

use super::sys_role_error::RoleError;
use crate::helper::db_helper;

#[async_trait]
pub trait TRoleService {
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<sys_role::Model>, AppError>;

    async fn create_role(&self, input: CreateRoleInput) -> Result<sys_role::Model, AppError>;
    async fn get_role(&self, id: i64) -> Result<sys_role::Model, AppError>;
    async fn update_role(&self, input: UpdateRoleInput) -> Result<sys_role::Model, AppError>;
    async fn delete_role(&self, id: i64) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysRoleService;

impl SysRoleService {
    async fn check_role_exists(&self, id: Option<i64>, code: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find().filter(sys_role::Column::Code.eq(code));

        if let Some(id) = id {
            query = query.filter(sys_role::Column::Id.ne(id));
        }

        let existing_role = query.one(db.as_ref()).await.map_err(AppError::from)?;

        if existing_role.is_some() {
            return Err(RoleError::DuplicateRoleCode.into());
        }

        Ok(())
    }
}

#[async_trait]
impl TRoleService for SysRoleService {
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<sys_role::Model>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(sys_role::Column::Name.contains(keywords));
            query = query.filter(condition);
        }

        let total = query.clone().count(db.as_ref()).await.map_err(AppError::from)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(AppError::from)?;

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_role(&self, input: CreateRoleInput) -> Result<sys_role::Model, AppError> {
        let db = db_helper::get_db_connection().await?;

        self.check_role_exists(None, &input.code).await?;

        let role = sys_role::ActiveModel {
            pid: Set(input.pid),
            code: Set(input.code),
            name: Set(input.name),
            remark: Set(input.remark),
            ..Default::default()
        };

        let result = role.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(result)
    }

    async fn get_role(&self, id: i64) -> Result<sys_role::Model, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysRole::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| RoleError::RoleNotFound.into())
    }

    async fn update_role(&self, input: UpdateRoleInput) -> Result<sys_role::Model, AppError> {
        let db = db_helper::get_db_connection().await?;

        self.check_role_exists(Some(input.id), &input.role.code).await?;

        let role: sys_role::ActiveModel = SysRole::find_by_id(input.id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::from(RoleError::RoleNotFound))?
            .into();

        let role = sys_role::ActiveModel {
            id: Set(input.id),
            pid: Set(input.role.pid),
            code: Set(input.role.code),
            name: Set(input.role.name),
            remark: Set(input.role.remark),

            updated_at: Set(Some(Utc::now().naive_utc())),
            ..role
        };

        let updated_role = role.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(updated_role)
    }

    async fn delete_role(&self, id: i64) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        SysRole::delete_by_id(id).exec(db.as_ref()).await.map_err(AppError::from)?;
        Ok(())
    }
}
