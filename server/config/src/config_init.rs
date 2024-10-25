use server_global::global;
use thiserror::Error;
use tokio::fs;

use crate::{model::Config, project_error, project_info, DatabaseConfig, JwtConfig, ServerConfig};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] serde_yaml::Error),
}

pub async fn init_from_file(file_path: &str) -> Result<(), ConfigError> {
    let config_data = fs::read_to_string(file_path).await.map_err(|e| {
        project_error!("Failed to read config file: {}", e);
        ConfigError::ReadError(e)
    })?;

    let config: Config = serde_yaml::from_str(&config_data).map_err(|e| {
        project_error!("Failed to parse config file: {}", e);
        ConfigError::ParseError(e)
    })?;

    global::init_config::<Config>(config.clone()).await;
    global::init_config::<DatabaseConfig>(config.database).await;
    global::init_config::<ServerConfig>(config.server).await;
    global::init_config::<JwtConfig>(config.jwt).await;

    project_info!("Configuration initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use simplelog::{Config as LogConfig, SimpleLogger};

    use super::*;
    use crate::model::DatabaseConfig;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            SimpleLogger::init(LevelFilter::Info, LogConfig::default()).unwrap();
        });
    }

    #[tokio::test]
    async fn test_valid_config() {
        init_logger();

        let result = init_from_file("examples/application.yaml").await;
        assert!(result.is_ok());

        let db_config = global::get_config::<DatabaseConfig>().await.unwrap();
        info!("db_config is {:?}", db_config);
        assert_eq!(db_config.url, "postgres://user:password@localhost/db");
    }
}
