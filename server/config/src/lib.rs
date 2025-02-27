pub use config_init::init_from_file;
pub use model::{
    Config, DatabaseConfig, DatabasesInstancesConfig, JwtConfig, MongoConfig, MongoInstancesConfig,
    OptionalConfigs, RedisConfig, RedisInstancesConfig, RedisMode, S3Config, S3InstancesConfig,
    ServerConfig,
};
pub use server_global::{project_error, project_info};

mod config_init;
mod model;
