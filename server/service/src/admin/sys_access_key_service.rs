use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{
        prelude::SysAccessKey,
        sys_access_key::{
            ActiveModel as SysAccessKeyActiveModel, Column as SysAccessKeyColumn,
            Model as SysAccessKeyModel,
        },
    },
    input::{AccessKeyPageRequest, CreateAccessKeyInput},
};
use ulid::Ulid;

use crate::helper::db_helper;

#[async_trait]
pub trait TAccessKeyService {
    async fn find_paginated_access_keys(
        &self,
        params: AccessKeyPageRequest,
    ) -> Result<PaginatedData<SysAccessKeyModel>, AppError>;
    async fn create_access_key(
        &self,
        input: CreateAccessKeyInput,
    ) -> Result<SysAccessKeyModel, AppError>;
    async fn delete_access_key(&self, id: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysAccessKeyService;

#[async_trait]
impl TAccessKeyService for SysAccessKeyService {
    async fn find_paginated_access_keys(
        &self,
        params: AccessKeyPageRequest,
    ) -> Result<PaginatedData<SysAccessKeyModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysAccessKey::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysAccessKeyColumn::Domain.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await
            .map_err(AppError::from)?;

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

    async fn create_access_key(
        &self,
        input: CreateAccessKeyInput,
    ) -> Result<SysAccessKeyModel, AppError> {
        let db = db_helper::get_db_connection().await?;

        let access_key = SysAccessKeyActiveModel {
            domain: Set(input.domain),
            status: Set(input.status),
            description: Set(input.description),
            access_key_id: Set(format!("AK{}", Ulid::new().to_string())),
            access_key_secret: Set(format!("SK{}", Ulid::new().to_string())),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("system".to_string()),
            ..Default::default()
        };

        let result = access_key
            .insert(db.as_ref())
            .await
            .map_err(AppError::from)?;
        Ok(result)
    }

    async fn delete_access_key(&self, id: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        SysAccessKey::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
