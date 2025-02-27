use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct S3Config {
    /// S3 区域
    pub region: String,
    /// S3 访问密钥ID
    pub access_key_id: String,
    /// S3 秘密访问密钥
    pub secret_access_key: String,
    /// S3 端点URL (可选，用于自定义S3兼容服务)
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct S3InstancesConfig {
    pub name: String,
    pub s3: S3Config,
}
