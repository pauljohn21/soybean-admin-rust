use std::sync::Arc;

use axum::{body::Body, Extension, Router};
use server_config::Config;
use server_constant::definition::Audience;
use server_global::global::get_config;
use server_middleware::{jwt_auth_middleware, Request, RequestId, RequestIdLayer};
use server_router::admin::{SysAuthenticationRouter, SysDomainRouter, SysUserRouter};
use server_service::admin::{SysAuthService, SysDomainService, SysUserService};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::initialize_casbin;

pub async fn initialize_admin_router() -> Router {
    let app_config = get_config::<Config>().await.unwrap();
    let casbin_axum_layer =
        initialize_casbin("server/resources/rbac_model.conf", app_config.database.url.as_str())
            .await
            .unwrap();

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

    // casbin_axum_layer
    //     .write()
    //     .await
    //     .get_role_manager()
    //     .write()
    //     .matching_fn(Some(key_match2), None);

    let app = Router::new();
    app.merge(
        SysAuthenticationRouter::init_authentication_router()
            .await
            .layer(Extension(Arc::new(SysAuthService)))
            .layer(trace_layer.clone())
            .layer(RequestIdLayer),
    )
    .merge(
        SysUserRouter::init_user_router()
            .await
            .layer(Extension(Arc::new(SysUserService)))
            .layer(Extension(casbin_axum_layer.clone()))
            .layer(trace_layer.clone())
            .layer(RequestIdLayer)
            .layer(casbin_axum_layer.clone())
            .layer(axum::middleware::from_fn(move |req, next| {
                jwt_auth_middleware(req, next, Audience::ManagementPlatform.as_str())
            })),
    )
    .merge(
        SysDomainRouter::init_domain_router()
            .await
            .layer(Extension(Arc::new(SysDomainService)))
            .layer(Extension(casbin_axum_layer.clone()))
            .layer(trace_layer.clone())
            .layer(RequestIdLayer)
            .layer(casbin_axum_layer.clone())
            .layer(axum::middleware::from_fn(move |req, next| {
                jwt_auth_middleware(req, next, Audience::ManagementPlatform.as_str())
            })),
    )
}
