use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{prelude::SysUser, sys_user},
    input::UserPageRequest,
};

use crate::helper::db_helper;

#[async_trait]
pub trait TUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, AppError>;
    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<sys_user::Model>, AppError>;
}

#[derive(Clone)]
pub struct SysUserService;

#[async_trait]
impl TUserService for SysUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find().all(db.as_ref()).await.map_err(AppError::from)
    }

    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<sys_user::Model>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysUser::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(sys_user::Column::Username.contains(keywords));
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
