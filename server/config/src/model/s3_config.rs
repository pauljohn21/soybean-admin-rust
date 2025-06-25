use serde::Deserialize;

/// S3 配置
///
/// 支持的环境变量：
/// - APP_S3_REGION: S3 区域
/// - APP_S3_ACCESS_KEY_ID: S3 访问密钥ID
/// - APP_S3_SECRET_ACCESS_KEY: S3 秘密访问密钥
/// - APP_S3_ENDPOINT: S3 端点URL (可选)
#[derive(Debug, Clone, Deserialize)]
pub struct S3Config {
    /// S3 区域
    /// 环境变量: APP_S3_REGION
    pub region: String,

    /// S3 访问密钥ID
    /// 环境变量: APP_S3_ACCESS_KEY_ID
    pub access_key_id: String,

    /// S3 秘密访问密钥
    /// 环境变量: APP_S3_SECRET_ACCESS_KEY
    pub secret_access_key: String,

    /// S3 端点URL (可选，用于自定义S3兼容服务)
    /// 环境变量: APP_S3_ENDPOINT
    pub endpoint: Option<String>,
}

/// S3 实例配置
///
/// 支持的环境变量（数组形式）：
/// - APP_S3_INSTANCES_0_NAME: 第一个实例名称
/// - APP_S3_INSTANCES_0_S3_REGION: 第一个实例区域
/// - APP_S3_INSTANCES_0_S3_ACCESS_KEY_ID: 第一个实例访问密钥ID
/// - APP_S3_INSTANCES_0_S3_SECRET_ACCESS_KEY: 第一个实例秘密访问密钥
/// - APP_S3_INSTANCES_0_S3_ENDPOINT: 第一个实例端点URL
/// 以此类推...
#[derive(Debug, Clone, Deserialize)]
pub struct S3InstancesConfig {
    /// 实例名称
    pub name: String,

    /// S3 配置
    pub s3: S3Config,
}
