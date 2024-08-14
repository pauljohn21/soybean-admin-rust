use std::sync::Arc;

use axum::{extract::Query, Extension};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{sys_role, RolePageRequest, SysRoleService, TRoleService};

pub struct SysRoleApi;

impl SysRoleApi {
    pub async fn get_paginated_roles(
        Query(params): Query<RolePageRequest>,
        Extension(service): Extension<Arc<SysRoleService>>,
    ) -> Result<Res<PaginatedData<sys_role::Model>>, AppError> {
        service.find_paginated_roles(params).await.map(Res::new_data)
    }
}
