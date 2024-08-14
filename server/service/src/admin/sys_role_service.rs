use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{prelude::SysRole, sys_role},
    input::RolePageRequest,
};

use crate::helper::db_helper;

#[async_trait]
pub trait TRoleService {
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<sys_role::Model>, AppError>;
}

#[derive(Clone)]
pub struct SysRoleService;

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
}
