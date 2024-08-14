use axum::{routing::get, Router};
use server_api::admin::SysRoleApi;

pub struct SysRoleRouter;

impl SysRoleRouter {
    pub async fn init_role_router() -> Router {
        let router = Router::new().route("/", get(SysRoleApi::get_paginated_roles));
        Router::new().nest("/role", router)
    }
}
