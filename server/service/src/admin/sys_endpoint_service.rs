use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{prelude::SysEndpoint, sys_endpoint},
    input::EndpointPageRequest,
};

use crate::helper::db_helper;

#[async_trait]
pub trait TEndpointService {
    async fn sync_endpoints(&self, endpoints: Vec<sys_endpoint::Model>) -> Result<(), AppError>;
    async fn find_paginated_endpoints(
        &self,
        params: EndpointPageRequest,
    ) -> Result<PaginatedData<sys_endpoint::Model>, AppError>;
}

pub struct SysEndpointService;

impl SysEndpointService {
    async fn batch_update_endpoints(
        &self,
        db: &DatabaseConnection,
        endpoints: Vec<sys_endpoint::Model>,
    ) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();
        let active_models: Vec<sys_endpoint::ActiveModel> = endpoints
            .into_iter()
            .map(|endpoint| {
                let mut active_model: sys_endpoint::ActiveModel = endpoint.into_active_model();
                active_model.updated_at = Set(Some(now));
                active_model
            })
            .collect();

        SysEndpoint::insert_many(active_models)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(sys_endpoint::Column::Id)
                    .update_columns([
                        sys_endpoint::Column::Path,
                        sys_endpoint::Column::Method,
                        sys_endpoint::Column::Action,
                        sys_endpoint::Column::Resource,
                        sys_endpoint::Column::Controller,
                        sys_endpoint::Column::Summary,
                        sys_endpoint::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    async fn batch_remove_endpoints(
        &self,
        db: &DatabaseConnection,
        endpoints_to_remove: Vec<String>,
    ) -> Result<DeleteResult, AppError> {
        SysEndpoint::delete_many()
            .filter(sys_endpoint::Column::Id.is_in(endpoints_to_remove))
            .exec(db)
            .await
            .map_err(AppError::from)
    }
}

#[async_trait]
impl TEndpointService for SysEndpointService {
    async fn sync_endpoints(
        &self,
        new_endpoints: Vec<sys_endpoint::Model>,
    ) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        // 获取数据库中现有的所有端点
        let existing_endpoints =
            SysEndpoint::find().all(db.as_ref()).await.map_err(AppError::from)?;

        // 批量更新和插入新的端点
        self.batch_update_endpoints(db.as_ref(), new_endpoints.clone()).await?;

        // 只有在数据库中已经存在端点的情况下才执行删除操作
        if !existing_endpoints.is_empty() {
            // 找出需要删除的端点
            let endpoints_to_remove: Vec<String> = existing_endpoints
                .iter()
                .filter(|existing_endpoint| {
                    !new_endpoints.iter().any(|e| {
                        e.path == existing_endpoint.path && e.method == existing_endpoint.method
                    })
                })
                .map(|endpoint| endpoint.id.clone())
                .collect();

            // 批量删除不再存在的端点
            if !endpoints_to_remove.is_empty() {
                self.batch_remove_endpoints(db.as_ref(), endpoints_to_remove).await?;
            }
        }

        Ok(())
    }

    async fn find_paginated_endpoints(
        &self,
        params: EndpointPageRequest,
    ) -> Result<PaginatedData<sys_endpoint::Model>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysEndpoint::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(sys_endpoint::Column::Path.contains(keywords))
                .add(sys_endpoint::Column::Method.contains(keywords))
                .add(sys_endpoint::Column::Controller.contains(keywords));
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
