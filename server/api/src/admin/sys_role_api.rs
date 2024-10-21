use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use server_core::web::{error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
use server_service::admin::{
    sys_role, CreateRoleInput, RolePageRequest, SysRoleService, TRoleService, UpdateRoleInput,
};

pub struct SysRoleApi;

impl SysRoleApi {
    pub async fn get_paginated_roles(
        Query(params): Query<RolePageRequest>,
        Extension(service): Extension<Arc<SysRoleService>>,
    ) -> Result<Res<PaginatedData<sys_role::Model>>, AppError> {
        service.find_paginated_roles(params).await.map(Res::new_data)
    }

    pub async fn create_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        ValidatedForm(input): ValidatedForm<CreateRoleInput>,
    ) -> Result<Res<sys_role::Model>, AppError> {
        service.create_role(input).await.map(Res::new_data)
    }

    pub async fn get_role(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysRoleService>>,
    ) -> Result<Res<sys_role::Model>, AppError> {
        service.get_role(&id).await.map(Res::new_data)
    }

    pub async fn update_role(
        Extension(service): Extension<Arc<SysRoleService>>,
        ValidatedForm(input): ValidatedForm<UpdateRoleInput>,
    ) -> Result<Res<sys_role::Model>, AppError> {
        service.update_role(input).await.map(Res::new_data)
    }

    pub async fn delete_role(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysRoleService>>,
    ) -> Result<Res<()>, AppError> {
        service.delete_role(&id).await.map(Res::new_data)
    }
}
