use std::sync::Arc;

use axum::{body::Body, http::StatusCode, response::IntoResponse, Extension, Router};
use axum_casbin::CasbinAxumLayer;
use server_config::Config;
use server_constant::definition::Audience;
use server_global::global::get_config;
use server_middleware::{jwt_auth_middleware, Request, RequestId, RequestIdLayer};
use server_router::admin::{
    SysAuthenticationRouter, SysDomainRouter, SysMenuRouter, SysRoleRouter, SysUserRouter,
};
use server_service::admin::{
    SysAuthService, SysDomainService, SysMenuService, SysRoleService, SysUserService,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::initialize_casbin;

pub async fn initialize_admin_router() -> Router {
    let app_config = get_config::<Config>().await.unwrap();
    let casbin_axum_layer =
        initialize_casbin("server/resources/rbac_model.conf", app_config.database.url.as_str())
            .await
            .unwrap();

    let audience: Audience = Audience::ManagementPlatform; // Adjust this as needed for different audiences

    let mut app = Router::new();

    app = app
        .merge(
            configure_router(
                SysAuthenticationRouter::init_authentication_router().await,
                Arc::new(SysAuthService),
                None,
                false,
                audience,
            )
            .await,
        )
        .merge(
            configure_router(
                SysMenuRouter::init_menu_router().await,
                Arc::new(SysMenuService),
                None,
                false,
                audience,
            )
            .await,
        )
        .merge(
            configure_router(
                SysMenuRouter::init_protected_menu_router().await,
                Arc::new(SysMenuService),
                Some(casbin_axum_layer.clone()),
                true,
                audience,
            )
            .await,
        )
        .merge(
            configure_router(
                SysUserRouter::init_user_router().await,
                Arc::new(SysUserService),
                Some(casbin_axum_layer.clone()),
                true,
                audience,
            )
            .await,
        )
        .merge(
            configure_router(
                SysDomainRouter::init_domain_router().await,
                Arc::new(SysDomainService),
                Some(casbin_axum_layer.clone()),
                true,
                audience,
            )
            .await,
        )
        .merge(
            configure_router(
                SysRoleRouter::init_role_router().await,
                Arc::new(SysRoleService),
                Some(casbin_axum_layer.clone()),
                true,
                audience,
            )
            .await,
        )
        .fallback(handler_404);

    app
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn configure_router<S>(
    router: Router,
    service: Arc<S>,
    casbin_layer: Option<CasbinAxumLayer>,
    require_auth: bool,
    audience: Audience,
) -> Router
where
    S: Send + Sync + 'static,
{
    let trace_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .map(ToString::to_string)
            .unwrap_or_else(|| "unknown".into());
        info_span!(
            "[soybean-admin-rust] >>>>>> request",
            id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
        )
    });

    let mut router = router.layer(Extension(service)).layer(trace_layer).layer(RequestIdLayer);

    if let Some(casbin) = casbin_layer {
        router = router.layer(casbin.clone());
    }

    if require_auth {
        router.layer(axum::middleware::from_fn(move |req, next| {
            jwt_auth_middleware(req, next, audience.as_str())
        }))
    } else {
        router
    }
}
