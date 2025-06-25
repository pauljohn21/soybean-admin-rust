use crate::{project_error, project_info};

/// 初始化配置（仅从文件加载，保持向后兼容）
pub async fn initialize_config(file_path: &str) {
    match server_config::init_from_file(file_path).await {
        Ok(_) => {
            project_info!("Configuration initialized successfully from: {}", file_path)
        },
        Err(e) => {
            project_error!("Failed to initialize config from {}: {:?}", file_path, e);
        },
    }
}

/// 初始化配置（环境变量优先，推荐使用）
///
/// 这是推荐的配置初始化方式，支持环境变量覆盖配置文件中的值
///
/// # 参数
/// - `file_path`: 配置文件路径
/// - `env_prefix`: 环境变量前缀（可选，默认为 "APP"）
///
/// # 示例
/// ```rust
/// // 使用默认前缀 "APP"
/// initialize_config_with_env("application.yaml", None).await;
///
/// // 使用自定义前缀 "MYAPP"
/// initialize_config_with_env("application.yaml", Some("MYAPP")).await;
/// ```
pub async fn initialize_config_with_env(file_path: &str, env_prefix: Option<&str>) {
    let prefix = env_prefix.unwrap_or("APP");
    project_info!("Initializing configuration with environment variable override support");
    project_info!("Config file: {}, Environment prefix: {}", file_path, prefix);

    match server_config::init_from_file_with_env(file_path, env_prefix).await {
        Ok(_) => {
            project_info!(
                "Configuration initialized successfully with environment variable support"
            )
        },
        Err(e) => {
            project_error!(
                "Failed to initialize config with environment variables: {:?}",
                e
            );
        },
    }
}

/// 仅从环境变量初始化配置
///
/// 当不需要配置文件，完全依赖环境变量时使用此函数
///
/// # 参数
/// - `env_prefix`: 环境变量前缀（可选，默认为 "APP"）
///
/// # 示例
/// ```rust
/// // 使用默认前缀 "APP"
/// initialize_config_from_env_only(None).await;
///
/// // 使用自定义前缀 "MYAPP"
/// initialize_config_from_env_only(Some("MYAPP")).await;
/// ```
pub async fn initialize_config_from_env_only(env_prefix: Option<&str>) {
    let prefix = env_prefix.unwrap_or("APP");
    project_info!("Initializing configuration from environment variables only");
    project_info!("Environment prefix: {}", prefix);

    match server_config::init_from_env_only(env_prefix).await {
        Ok(_) => {
            project_info!("Configuration initialized successfully from environment variables only")
        },
        Err(e) => {
            project_error!(
                "Failed to initialize config from environment variables: {:?}",
                e
            );
        },
    }
}

/// 初始化配置（支持多实例环境变量覆盖，推荐使用）
///
/// 这是最强大的配置初始化方式，支持：
/// - 环境变量覆盖配置文件中的单个值
/// - 环境变量覆盖配置文件中的多实例配置
/// - 完整的多实例环境变量支持
///
/// # 参数
/// - `file_path`: 配置文件路径
/// - `env_prefix`: 环境变量前缀（可选，默认为 "APP"）
///
/// # 示例
/// ```rust
/// // 使用默认前缀 "APP"
/// initialize_config_with_multi_instance_env("application.yaml", None).await;
///
/// // 使用自定义前缀 "MYAPP"
/// initialize_config_with_multi_instance_env("application.yaml", Some("MYAPP")).await;
/// ```
pub async fn initialize_config_with_multi_instance_env(file_path: &str, env_prefix: Option<&str>) {
    let prefix = env_prefix.unwrap_or("APP");
    project_info!("Initializing configuration with multi-instance environment variable support");
    project_info!("Config file: {}, Environment prefix: {}", file_path, prefix);

    match server_config::init_from_file_with_multi_instance_env(file_path, env_prefix).await {
        Ok(_) => {
            project_info!("Configuration initialized successfully with multi-instance environment variable support")
        },
        Err(e) => {
            project_error!(
                "Failed to initialize config with multi-instance environment variables: {:?}",
                e
            );
        },
    }
}
