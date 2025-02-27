use server_global::global;
use std::path::Path;
use thiserror::Error;
use tokio::fs;

use crate::{
    model::{Config, OptionalConfigs},
    project_error, project_info, DatabaseConfig, DatabasesInstancesConfig, JwtConfig, MongoConfig,
    MongoInstancesConfig, RedisConfig, RedisInstancesConfig, S3Config, S3InstancesConfig,
    ServerConfig,
};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse YAML config: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Failed to parse TOML config: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Failed to parse JSON config: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Unsupported config file format: {0}")]
    UnsupportedFormat(String),
}

async fn parse_config(file_path: &str, content: String) -> Result<Config, ConfigError> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "yaml" | "yml" => Ok(serde_yaml::from_str(&content)?),
        "toml" => Ok(toml::from_str(&content)?),
        "json" => Ok(serde_json::from_str(&content)?),
        _ => Err(ConfigError::UnsupportedFormat(extension)),
    }
}

pub async fn init_from_file(file_path: &str) -> Result<(), ConfigError> {
    let config_data = fs::read_to_string(file_path).await.map_err(|e| {
        project_error!("Failed to read config file: {}", e);
        ConfigError::ReadError(e)
    })?;

    let config = parse_config(file_path, config_data).await.map_err(|e| {
        project_error!("Failed to parse config file: {}", e);
        e
    })?;

    global::init_config::<Config>(config.clone()).await;
    global::init_config::<DatabaseConfig>(config.database).await;

    global::init_config::<OptionalConfigs<DatabasesInstancesConfig>>(
        config.database_instances.into(),
    )
    .await;

    global::init_config::<ServerConfig>(config.server).await;
    global::init_config::<JwtConfig>(config.jwt).await;

    if let Some(redis_config) = config.redis {
        global::init_config::<RedisConfig>(redis_config).await;
    }
    global::init_config::<OptionalConfigs<RedisInstancesConfig>>(config.redis_instances.into())
        .await;

    if let Some(mongo_config) = config.mongo {
        global::init_config::<MongoConfig>(mongo_config).await;
    }
    global::init_config::<OptionalConfigs<MongoInstancesConfig>>(config.mongo_instances.into())
        .await;

    if let Some(s3_config) = config.s3 {
        global::init_config::<S3Config>(s3_config).await;
    }
    global::init_config::<OptionalConfigs<S3InstancesConfig>>(config.s3_instances.into()).await;

    project_info!("Configuration initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use log::{info, LevelFilter};
    use simplelog::{Config as LogConfig, SimpleLogger};

    use super::*;
    use crate::model::DatabaseConfig;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            SimpleLogger::init(LevelFilter::Info, LogConfig::default()).unwrap();
        });
    }

    #[cfg_attr(test, tokio::test)]
    async fn test_yaml_config() {
        init_logger();
        let result = init_from_file("examples/application.yaml").await;
        assert!(result.is_ok());
        let db_config = global::get_config::<DatabaseConfig>().await.unwrap();
        info!("db_config is {:?}", db_config);
        assert_eq!(db_config.url, "postgres://user:password@localhost/db");
    }

    #[cfg_attr(test, tokio::test)]
    async fn test_toml_config() {
        init_logger();
        let result = init_from_file("examples/application.toml").await;
        assert!(result.is_ok());
    }

    #[cfg_attr(test, tokio::test)]
    async fn test_json_config() {
        init_logger();
        let result = init_from_file("examples/application.json").await;
        assert!(result.is_ok());
    }
}
