use axum::{
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysMenuApi;

pub struct SysMenuRouter;

impl SysMenuRouter {
    pub async fn init_menu_router() -> Router {
        let router =
            Router::new().route("/getConstantRoutes", get(SysMenuApi::get_constant_routes));
        Router::new().nest("/route", router)
    }

    pub async fn init_protected_menu_router() -> Router {
        let router = Router::new()
            .route("/", post(SysMenuApi::create_menu))
            .route("/:id", get(SysMenuApi::get_menu))
            .route("/", put(SysMenuApi::update_menu))
            .route("/:id", delete(SysMenuApi::delete_menu));
        Router::new().nest("/route", router)
    }
}
