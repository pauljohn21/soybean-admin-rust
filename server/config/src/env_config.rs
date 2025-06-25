use config::{Config as ConfigBuilder, ConfigError as ConfigBuilderError, Environment, File};
use serde::de::DeserializeOwned;
use std::path::Path;
use thiserror::Error;

use crate::{project_error, project_info};

#[derive(Error, Debug)]
pub enum EnvConfigError {
    #[error("Config builder error: {0}")]
    ConfigBuilder(#[from] ConfigBuilderError),
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// 环境变量优先的配置加载器
///
/// 加载优先级：环境变量 > 配置文件 > 默认值
///
/// 环境变量命名规范：
/// - 使用 APP_ 前缀
/// - 嵌套配置用下划线分隔，如：APP_DATABASE_URL
/// - 数组配置用索引，如：APP_REDIS_INSTANCES_0_NAME
///
/// # 示例
/// ```rust,no_run
/// use server_config::env_config::EnvConfigLoader;
/// use server_config::Config;
///
/// let config: Config = EnvConfigLoader::new()
///     .with_file("examples/application.yaml")
///     .with_env_prefix("APP")
///     .load()
///     .expect("Failed to load config");
/// ```
pub struct EnvConfigLoader {
    file_path: Option<String>,
    env_prefix: String,
    env_separator: String,
}

impl Default for EnvConfigLoader {
    fn default() -> Self {
        Self {
            file_path: None,
            env_prefix: "APP".to_string(),
            env_separator: "_".to_string(),
        }
    }
}

impl EnvConfigLoader {
    /// 创建新的配置加载器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置配置文件路径
    pub fn with_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.file_path = Some(path.as_ref().to_string_lossy().to_string());
        self
    }

    /// 设置环境变量前缀（默认为 "APP"）
    pub fn with_env_prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.env_prefix = prefix.into();
        self
    }

    /// 设置环境变量分隔符（默认为 "_"）
    pub fn with_env_separator<S: Into<String>>(mut self, separator: S) -> Self {
        self.env_separator = separator.into();
        self
    }

    /// 加载配置
    ///
    /// 按照以下优先级加载配置：
    /// 1. 环境变量（最高优先级）
    /// 2. 配置文件
    /// 3. 默认值（最低优先级）
    pub fn load<T>(&self) -> Result<T, EnvConfigError>
    where
        T: DeserializeOwned,
    {
        let mut builder = ConfigBuilder::builder();

        // 1. 如果指定了配置文件，先加载文件配置
        if let Some(file_path) = &self.file_path {
            project_info!("Loading config from file: {}", file_path);

            let file_format = self.detect_file_format(file_path)?;
            builder = builder.add_source(File::with_name(file_path).format(file_format));
        }

        // 2. 加载环境变量配置（会覆盖文件配置）
        project_info!(
            "Loading config from environment variables with prefix: {}",
            self.env_prefix
        );
        builder = builder.add_source(
            Environment::with_prefix(&self.env_prefix)
                .separator(&self.env_separator)
                .try_parsing(true),
        );

        // 3. 构建最终配置
        let config = builder.build()?;

        // 4. 反序列化为目标类型
        let result: T = config.try_deserialize()?;

        project_info!(
            "Configuration loaded successfully with environment variable override support"
        );
        Ok(result)
    }

    /// 检测文件格式
    fn detect_file_format(&self, file_path: &str) -> Result<config::FileFormat, EnvConfigError> {
        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "yaml" | "yml" => Ok(config::FileFormat::Yaml),
            "toml" => Ok(config::FileFormat::Toml),
            "json" => Ok(config::FileFormat::Json),
            _ => {
                project_error!("Unsupported file format: {}", extension);
                Err(EnvConfigError::UnsupportedFormat(extension))
            },
        }
    }
}

/// 便捷函数：从文件和环境变量加载配置
///
/// # 参数
/// - `file_path`: 配置文件路径
/// - `env_prefix`: 环境变量前缀（可选，默认为 "APP"）
///
/// # 示例
/// ```rust,no_run
/// use server_config::env_config::load_config_with_env;
/// use server_config::Config;
///
/// let config: Config = load_config_with_env("examples/application.yaml", Some("APP"))
///     .expect("Failed to load config");
/// ```
pub fn load_config_with_env<T>(
    file_path: &str,
    env_prefix: Option<&str>,
) -> Result<T, EnvConfigError>
where
    T: DeserializeOwned,
{
    let mut loader = EnvConfigLoader::new().with_file(file_path);

    if let Some(prefix) = env_prefix {
        loader = loader.with_env_prefix(prefix);
    }

    loader.load()
}

/// 便捷函数：仅从环境变量加载配置
///
/// # 参数
/// - `env_prefix`: 环境变量前缀（可选，默认为 "APP"）
///
/// # 示例
/// ```rust,no_run
/// use server_config::env_config::load_config_from_env;
/// use server_config::Config;
/// use std::env;
///
/// // 设置必需的环境变量
/// env::set_var("APP_DATABASE_URL", "postgres://user:pass@localhost/db");
/// env::set_var("APP_DATABASE_MAX_CONNECTIONS", "10");
/// env::set_var("APP_DATABASE_MIN_CONNECTIONS", "1");
/// env::set_var("APP_DATABASE_CONNECT_TIMEOUT", "30");
/// env::set_var("APP_DATABASE_IDLE_TIMEOUT", "600");
/// env::set_var("APP_SERVER_HOST", "0.0.0.0");
/// env::set_var("APP_SERVER_PORT", "8080");
/// env::set_var("APP_JWT_JWT_SECRET", "secret");
/// env::set_var("APP_JWT_ISSUER", "issuer");
/// env::set_var("APP_JWT_EXPIRE", "3600");
///
/// let config: Config = load_config_from_env(Some("APP"))
///     .expect("Failed to load config");
/// ```
pub fn load_config_from_env<T>(env_prefix: Option<&str>) -> Result<T, EnvConfigError>
where
    T: DeserializeOwned,
{
    let mut loader = EnvConfigLoader::new();

    if let Some(prefix) = env_prefix {
        loader = loader.with_env_prefix(prefix);
    }

    loader.load()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_config_loader_creation() {
        let loader = EnvConfigLoader::new();
        assert_eq!(loader.env_prefix, "APP");
        assert_eq!(loader.env_separator, "_");
    }

    #[test]
    fn test_env_config_loader_with_custom_prefix() {
        let loader = EnvConfigLoader::new().with_env_prefix("CUSTOM");
        assert_eq!(loader.env_prefix, "CUSTOM");
    }

    #[test]
    fn test_env_config_loader_with_custom_separator() {
        let loader = EnvConfigLoader::new().with_env_separator("__");
        assert_eq!(loader.env_separator, "__");
    }
}
