use std::sync::Arc;

use axum::{body::Body, http::StatusCode, response::IntoResponse, Extension, Router};
use axum_casbin::CasbinAxumLayer;
use chrono::Utc;
use http::Request;
use server_config::Config;
use server_constant::definition::Audience;
use server_core::web::{RequestId, RequestIdLayer};
use server_global::global::{clear_routes, get_collected_routes, get_config};
use server_middleware::{
    api_key_middleware, jwt_auth_middleware, protect_route, ApiKeySource, ApiKeyValidation,
    ComplexApiKeyConfig, ComplexApiKeyValidator, SimpleApiKeyConfig, SimpleApiKeyValidator,
};
use server_router::admin::{
    SysAccessKeyRouter, SysAuthenticationRouter, SysDomainRouter, SysEndpointRouter,
    SysLoginLogRouter, SysMenuRouter, SysOperationLogRouter, SysOrganizationRouter, SysRoleRouter,
    SysSandboxRouter, SysUserRouter,
};
use server_service::{
    admin::{
        SysAccessKeyService, SysAuthService, SysDomainService, SysEndpointService,
        SysLoginLogService, SysMenuService, SysOperationLogService, SysOrganizationService,
        SysRoleService, SysUserService, TEndpointService,
    },
    SysEndpoint,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::{initialize_casbin, project_error, project_info};

async fn apply_layers(
    router: Router,
    service: Arc<dyn Send + Sync + 'static>,
    need_casbin: bool,
    need_auth: bool,
    api_validation: Option<ApiKeyValidation>,
    casbin: Option<CasbinAxumLayer>,
    audience: Audience,
) -> Router {
    let mut router = router
        .layer(Extension(service))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
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
            }),
        )
        .layer(RequestIdLayer);

    if need_casbin {
        if let Some(casbin) = casbin {
            router = router.layer(casbin);
        }
    }

    if let Some(validation) = api_validation {
        router = router.layer(axum::middleware::from_fn(move |req, next| {
            api_key_middleware(validation.clone(), req, next)
        }));
    }

    if need_auth {
        router = router.layer(axum::middleware::from_fn(move |req, next| {
            jwt_auth_middleware(req, next, audience.as_str())
        }));
    }

    router
}

pub async fn initialize_admin_router() -> Router {
    clear_routes().await;
    project_info!("Initializing admin router");

    let app_config = get_config::<Config>().await.unwrap();
    let casbin_layer = initialize_casbin(
        "server/resources/rbac_model.conf",
        app_config.database.url.as_str(),
    )
    .await
    .unwrap();

    // 初始化验证器
    let simple_validation = {
        let validator = SimpleApiKeyValidator::new();
        validator.add_key("test-api-key".to_string());
        ApiKeyValidation::Simple(
            validator,
            SimpleApiKeyConfig {
                source: ApiKeySource::Header,
                key_name: "x-api-key".to_string(),
            },
        )
    };

    let complex_validation = {
        let validator = ComplexApiKeyValidator::new(None);
        validator.add_key_secret("test-access-key".to_string(), "test-secret-key".to_string());
        ApiKeyValidation::Complex(
            validator,
            ComplexApiKeyConfig {
                key_name: "AccessKeyId".to_string(),
                timestamp_name: "t".to_string(),
                nonce_name: "n".to_string(),
                signature_name: "sign".to_string(),
            },
        )
    };

    // 保护路由
    protect_route("/sandbox/simple-api-key");
    protect_route("/sandbox/complex-api-key");

    let audience = Audience::ManagementPlatform;
    let casbin = Some(casbin_layer);
    let mut app = Router::new();

    macro_rules! merge_router {
        ($router:expr, None, $need_casbin:expr, $need_auth:expr, $api_validation:expr) => {
            app = app.merge(
                apply_layers(
                    $router,
                    Arc::new(()),
                    $need_casbin,
                    $need_auth,
                    $api_validation,
                    casbin.clone(),
                    audience,
                )
                .await,
            );
        };
        ($router:expr, $service:expr, $need_casbin:expr, $need_auth:expr, $api_validation:expr) => {
            app = app.merge(
                apply_layers(
                    $router,
                    Arc::new($service),
                    $need_casbin,
                    $need_auth,
                    $api_validation,
                    casbin.clone(),
                    audience,
                )
                .await,
            );
        };
    }

    merge_router!(
        SysAuthenticationRouter::init_authentication_router().await,
        SysAuthService,
        false,
        false,
        None
    );

    merge_router!(
        SysAuthenticationRouter::init_protected_router().await,
        SysAuthService,
        false,
        true,
        None
    );

    merge_router!(
        SysMenuRouter::init_menu_router().await,
        SysMenuService,
        false,
        false,
        None
    );

    merge_router!(
        SysMenuRouter::init_protected_menu_router().await,
        SysMenuService,
        true,
        true,
        None
    );

    merge_router!(
        SysUserRouter::init_user_router().await,
        SysUserService,
        true,
        true,
        None
    );
    merge_router!(
        SysDomainRouter::init_domain_router().await,
        SysDomainService,
        true,
        true,
        None
    );
    merge_router!(
        SysRoleRouter::init_role_router().await,
        SysRoleService,
        true,
        true,
        None
    );
    merge_router!(
        SysEndpointRouter::init_endpoint_router().await,
        SysEndpointService,
        true,
        true,
        None
    );
    merge_router!(
        SysAccessKeyRouter::init_access_key_router().await,
        SysAccessKeyService,
        true,
        true,
        None
    );
    merge_router!(
        SysLoginLogRouter::init_login_log_router().await,
        SysLoginLogService,
        true,
        true,
        None
    );
    merge_router!(
        SysOperationLogRouter::init_operation_log_router().await,
        SysOperationLogService,
        true,
        true,
        None
    );

    merge_router!(
        SysOrganizationRouter::init_organization_router().await,
        SysOrganizationService,
        false,
        false,
        None
    );

    // sandbox
    merge_router!(
        SysSandboxRouter::init_simple_sandbox_router().await,
        None,
        false,
        false,
        Some(simple_validation)
    );
    merge_router!(
        SysSandboxRouter::init_complex_sandbox_router().await,
        None,
        false,
        false,
        Some(complex_validation)
    );

    app = app.fallback(handler_404);

    process_collected_routes().await;
    project_info!("Admin router initialization completed");

    app
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
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
        },
        Err(e) => {
            project_error!("Failed to sync endpoints: {:?}", e)
        },
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
