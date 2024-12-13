use axum::{
    routing::{get, post},
    Router,
};
use server_api::admin::SysAuthenticationApi;

pub struct SysAuthenticationRouter;

impl SysAuthenticationRouter {
    pub async fn init_authentication_router() -> Router {
        let router = Router::new().route("/login", post(SysAuthenticationApi::login_handler));
        Router::new().nest("/auth", router)
    }

    pub async fn init_protected_router() -> Router {
        let router = Router::new().route("/getUserInfo", get(SysAuthenticationApi::get_user_info));

        let authorization_router =
            Router::new().route("/getUserRoutes", get(SysAuthenticationApi::get_user_routes));

        Router::new()
            .nest("/auth", router)
            .nest("/authorization", authorization_router)
    }
}
