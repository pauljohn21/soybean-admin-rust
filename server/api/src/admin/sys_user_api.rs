use std::sync::Arc;

use axum::{extract::Query, Extension};
use axum_casbin::{casbin::MgmtApi, CasbinAxumLayer};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{sys_user, SysUserService, TUserService, UserPageRequest};

pub struct SysUserApi;

impl SysUserApi {
    pub async fn get_all_users(
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Result<Res<Vec<sys_user::Model>>, AppError> {
        service.find_all().await.map(Res::new_data)
    }

    pub async fn get_paginated_users(
        Query(params): Query<UserPageRequest>,
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Result<Res<PaginatedData<sys_user::Model>>, AppError> {
        service.find_paginated_users(params).await.map(Res::new_data)
    }

    pub async fn remove_policies(
        Extension(mut cache_enforcer): Extension<CasbinAxumLayer>,
    ) -> Res<bool> {
        let enforcer = cache_enforcer.get_enforcer();
        let mut enforcer_write = enforcer.write().await;
        let rule = vec![
            "1".to_string(),
            "built-in".to_string(),
            "/user/users".to_string(),
            "GET".to_string(),
        ];
        let _ = enforcer_write.remove_policies(vec![rule]).await;
        Res::new_data(true)
    }

    pub async fn add_policies(
        Extension(mut cache_enforcer): Extension<CasbinAxumLayer>,
    ) -> Res<bool> {
        let enforcer = cache_enforcer.get_enforcer();
        let mut enforcer_write = enforcer.write().await;
        let rule = vec![
            "1".to_string(),
            "built-in".to_string(),
            "/user/users".to_string(),
            "GET".to_string(),
        ];
        let _ = enforcer_write.add_policy(rule).await;
        Res::new_data(true)
    }
}
