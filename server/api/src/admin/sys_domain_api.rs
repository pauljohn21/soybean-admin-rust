use std::sync::Arc;

use axum::{extract::Query, Extension};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{sys_domain, DomainPageRequest, SysDomainService, TDomainService};

pub struct SysDomainApi;

impl SysDomainApi {
    pub async fn get_paginated_domains(
        Query(params): Query<DomainPageRequest>,
        Extension(service): Extension<Arc<SysDomainService>>,
    ) -> Result<Res<PaginatedData<sys_domain::Model>>, AppError> {
        service.find_paginated_domains(params).await.map(Res::new_data)
    }
}
