use axum::{routing::get, Router};
use server_api::admin::SysUserApi;

pub struct SysUserRouter;

impl SysUserRouter {
    pub async fn init_user_router() -> Router {
        let router = Router::new()
            .route("/users", get(SysUserApi::get_all_users))
            .route("/", get(SysUserApi::get_paginated_users))
            .route("/add_policies", get(SysUserApi::add_policies))
            .route("/remove_policies", get(SysUserApi::remove_policies));
        Router::new().nest("/user", router)
    }
}
