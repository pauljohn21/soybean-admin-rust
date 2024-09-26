use axum::{routing::get, Router};
use server_api::admin::SysMenuApi;

pub struct SysMenuRouter;

impl SysMenuRouter {
    pub async fn init_menu_router() -> Router {
        let router =
            Router::new().route("/getConstantRoutes", get(SysMenuApi::get_constant_routes));
        Router::new().nest("/route", router)
    }
}
