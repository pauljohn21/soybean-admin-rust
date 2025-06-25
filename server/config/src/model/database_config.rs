use serde::Deserialize;

/// 数据库配置
///
/// 支持的环境变量：
/// - APP_DATABASE_URL: 数据库连接URL
/// - APP_DATABASE_MAX_CONNECTIONS: 最大连接数
/// - APP_DATABASE_MIN_CONNECTIONS: 最小连接数
/// - APP_DATABASE_CONNECT_TIMEOUT: 连接超时时间（秒）
/// - APP_DATABASE_IDLE_TIMEOUT: 空闲超时时间（秒）
#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    /// 数据库连接URL
    /// 环境变量: APP_DATABASE_URL
    pub url: String,

    /// 最大连接数
    /// 环境变量: APP_DATABASE_MAX_CONNECTIONS
    pub max_connections: u32,

    /// 最小连接数
    /// 环境变量: APP_DATABASE_MIN_CONNECTIONS
    pub min_connections: u32,

    /// 连接超时时间（秒）
    /// 环境变量: APP_DATABASE_CONNECT_TIMEOUT
    pub connect_timeout: u64,

    /// 空闲超时时间（秒）
    /// 环境变量: APP_DATABASE_IDLE_TIMEOUT
    pub idle_timeout: u64,
}

/// 数据库实例配置
///
/// 支持的环境变量（数组形式）：
/// - APP_DATABASE_INSTANCES_0_NAME: 第一个实例名称
/// - APP_DATABASE_INSTANCES_0_DATABASE_URL: 第一个实例数据库URL
/// - APP_DATABASE_INSTANCES_1_NAME: 第二个实例名称
/// - APP_DATABASE_INSTANCES_1_DATABASE_URL: 第二个实例数据库URL
/// 以此类推...
#[derive(Deserialize, Debug, Clone)]
pub struct DatabasesInstancesConfig {
    /// 实例名称
    pub name: String,

    /// 数据库配置
    pub database: DatabaseConfig,
}
