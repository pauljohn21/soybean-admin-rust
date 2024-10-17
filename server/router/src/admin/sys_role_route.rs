use axum::{
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysRoleApi;

pub struct SysRoleRouter;

impl SysRoleRouter {
    pub async fn init_role_router() -> Router {
        let router = Router::new()
            .route("/", get(SysRoleApi::get_paginated_roles))
            .route("/", post(SysRoleApi::create_role))
            .route("/:id", get(SysRoleApi::get_role))
            .route("/", put(SysRoleApi::update_role))
            .route("/:id", delete(SysRoleApi::delete_role));
        Router::new().nest("/role", router)
    }
}
