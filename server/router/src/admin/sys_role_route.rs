use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysRoleApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysRoleRouter;

impl SysRoleRouter {
    pub async fn init_role_router() -> Router {
        let base_path = "/role";
        let service_name = "SysRoleApi";

        let routes = vec![
            RouteInfo::new(&format!("{}/list", base_path), Method::GET, service_name),
            RouteInfo::new(base_path, Method::POST, service_name),
            RouteInfo::new(&format!("{}/:id", base_path), Method::GET, service_name),
            RouteInfo::new(base_path, Method::PUT, service_name),
            RouteInfo::new(&format!("{}/:id", base_path), Method::DELETE, service_name),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/list", get(SysRoleApi::get_paginated_roles))
            .route("/", post(SysRoleApi::create_role))
            .route("/:id", get(SysRoleApi::get_role))
            .route("/", put(SysRoleApi::update_role))
            .route("/:id", delete(SysRoleApi::delete_role));

        Router::new().nest(base_path, router)
    }
}
