use std::sync::Arc;

use axum::{body::Body, http::StatusCode, response::IntoResponse, Extension, Router};
use axum_casbin::CasbinAxumLayer;
use chrono::Utc;
use http::Request;
use server_config::Config;
use server_constant::definition::Audience;
use server_core::web::{RequestId, RequestIdLayer};
use server_global::global::{clear_routes, get_collected_routes, get_config};
use server_middleware::jwt_auth_middleware;
use server_router::admin::{
    SysAuthenticationRouter, SysDomainRouter, SysEndpointRouter, SysMenuRouter, SysRoleRouter,
    SysUserRouter,
};
use server_service::{
    admin::{
        SysAuthService, SysDomainService, SysEndpointService, SysMenuService, SysRoleService,
        SysUserService, TEndpointService,
    },
    SysEndpoint,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::{initialize_casbin, project_error, project_info};

pub async fn initialize_admin_router() -> Router {
    clear_routes().await;
    project_info!("Initializing admin router");

    let app_config = get_config::<Config>().await.unwrap();
    let casbin_axum_layer =
        initialize_casbin("server/resources/rbac_model.conf", app_config.database.url.as_str())
            .await
            .unwrap();

    let audience: Audience = Audience::ManagementPlatform;

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
        .merge(
            configure_router(
                SysEndpointRouter::init_endpoint_router().await,
                Arc::new(SysEndpointService),
                Some(casbin_axum_layer.clone()),
                true,
                audience,
            )
            .await,
        )
        .fallback(handler_404);

    process_collected_routes().await;
    project_info!("Admin router initialization completed");

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

async fn process_collected_routes() {
    let routes = get_collected_routes().await;
    let endpoints: Vec<SysEndpoint> = routes
        .into_iter()
        .map(|route| {
            let resource = route.path.split('/').nth(1).unwrap_or("").to_string();
            SysEndpoint {
                id: generate_id(&route.path, &route.method.to_string()),
                path: route.path.clone(),
                method: route.method.to_string(),
                action: "rw".to_string(),
                resource,
                controller: route.service_name,
                summary: Some(route.summary),
                created_at: Utc::now().naive_utc(),
                updated_at: None,
            }
        })
        .collect();

    let endpoint_service = SysEndpointService;
    match endpoint_service.sync_endpoints(endpoints).await {
        Ok(_) => {
            project_info!("Endpoints synced successfully")
        }
        Err(e) => {
            project_error!("Failed to sync endpoints: {:?}", e)
        }
    }
}

fn generate_id(path: &str, method: &str) -> String {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    let mut hasher = DefaultHasher::new();
    format!("{}{}", path, method).hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
