use axum::{http::Method, routing::get, Router};
use server_api::admin::SysEndpointApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysEndpointRouter;

impl SysEndpointRouter {
    pub async fn init_endpoint_router() -> Router {
        let base_path = "/endpoint";
        let service_name = "SysEndpointApi";

        let routes = vec![RouteInfo::new(
            &format!("{}/list", base_path),
            Method::GET,
            service_name,
            "获取接口列表",
        )];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new().route("/list", get(SysEndpointApi::get_paginated_endpoints));

        Router::new().nest(base_path, router)
    }
}
