use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysMenuApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysMenuRouter;

impl SysMenuRouter {
    pub async fn init_menu_router() -> Router {
        let router =
            Router::new().route("/getConstantRoutes", get(SysMenuApi::get_constant_routes));
        Router::new().nest("/route", router)
    }

    pub async fn init_protected_menu_router() -> Router {
        let base_path = "/route";
        let service_name = "SysMenuApi";

        let routes = vec![
            RouteInfo::new(base_path, Method::POST, service_name),
            RouteInfo::new(&format!("{}/:id", base_path), Method::GET, service_name),
            RouteInfo::new(base_path, Method::PUT, service_name),
            RouteInfo::new(&format!("{}/:id", base_path), Method::DELETE, service_name),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/", post(SysMenuApi::create_menu))
            .route("/:id", get(SysMenuApi::get_menu))
            .route("/", put(SysMenuApi::update_menu))
            .route("/:id", delete(SysMenuApi::delete_menu));

        Router::new().nest(base_path, router)
    }
}
