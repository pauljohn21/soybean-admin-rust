use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use server_core::web::{error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
use server_service::admin::{
    sys_domain, CreateDomainInput, DomainPageRequest, SysDomainService, TDomainService,
    UpdateDomainInput,
};

pub struct SysDomainApi;

impl SysDomainApi {
    pub async fn get_paginated_domains(
        Query(params): Query<DomainPageRequest>,
        Extension(service): Extension<Arc<SysDomainService>>,
    ) -> Result<Res<PaginatedData<sys_domain::Model>>, AppError> {
        service.find_paginated_domains(params).await.map(Res::new_data)
    }

    pub async fn create_domain(
        Extension(service): Extension<Arc<SysDomainService>>,
        ValidatedForm(input): ValidatedForm<CreateDomainInput>,
    ) -> Result<Res<sys_domain::Model>, AppError> {
        service.create_domain(input).await.map(Res::new_data)
    }

    pub async fn get_domain(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysDomainService>>,
    ) -> Result<Res<sys_domain::Model>, AppError> {
        service.get_domain(&id).await.map(Res::new_data)
    }

    pub async fn update_domain(
        Extension(service): Extension<Arc<SysDomainService>>,
        ValidatedForm(input): ValidatedForm<UpdateDomainInput>,
    ) -> Result<Res<sys_domain::Model>, AppError> {
        service.update_domain(input).await.map(Res::new_data)
    }

    pub async fn delete_domain(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysDomainService>>,
    ) -> Result<Res<()>, AppError> {
        service.delete_domain(&id).await.map(Res::new_data)
    }
}
