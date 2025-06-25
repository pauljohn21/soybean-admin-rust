use serde::Deserialize;

/// Redis 配置
///
/// 支持的环境变量：
/// - APP_REDIS_MODE: Redis 模式 (single/cluster)
/// - APP_REDIS_URL: Redis 连接 URL (单机模式)
/// - APP_REDIS_URLS: Redis 集群节点地址列表 (逗号分隔)
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis 模式
    /// 环境变量: APP_REDIS_MODE
    pub mode: RedisMode,

    /// Redis 连接 URL
    /// 环境变量: APP_REDIS_URL
    ///
    /// 支持以下格式：
    /// 1. 标准 TCP 连接:
    ///    redis://[<username>][:<password>@]<hostname>[:port][/[<db>][?protocol=<protocol>]]
    ///    示例：
    ///    - 基本连接：redis://127.0.0.1:6379/0
    ///    - 带密码：redis://:password@127.0.0.1:6379/0
    ///    - 带用户名和密码：redis://username:password@127.0.0.1:6379/0
    ///
    /// 2. Unix Socket 连接 (如果系统支持):
    ///    redis+unix:///<path>[?db=<db>[&pass=<password>][&user=<username>][&protocol=<protocol>]]
    ///    或
    ///    unix:///<path>[?db=<db>][&pass=<password>][&user=<username>][&protocol=<protocol>]]
    pub url: Option<String>,

    /// Redis 集群节点地址列表
    /// 环境变量: APP_REDIS_URLS (逗号分隔的URL列表)
    /// 每个地址都支持与 url 相同的格式
    ///
    /// 注意：
    /// - 集群模式下，db 参数将被忽略，因为 Redis 集群不支持多数据库
    /// - 所有节点应使用相同的认证信息（用户名/密码）
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum RedisMode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "cluster")]
    Cluster,
}

/// Redis 实例配置
///
/// 支持的环境变量（数组形式）：
/// - APP_REDIS_INSTANCES_0_NAME: 第一个实例名称
/// - APP_REDIS_INSTANCES_0_REDIS_MODE: 第一个实例模式
/// - APP_REDIS_INSTANCES_0_REDIS_URL: 第一个实例URL
/// - APP_REDIS_INSTANCES_1_NAME: 第二个实例名称
/// - APP_REDIS_INSTANCES_1_REDIS_MODE: 第二个实例模式
/// - APP_REDIS_INSTANCES_1_REDIS_URL: 第二个实例URL
/// 以此类推...
#[derive(Debug, Clone, Deserialize)]
pub struct RedisInstancesConfig {
    /// 实例名称
    pub name: String,

    /// Redis 配置
    pub redis: RedisConfig,
}

impl RedisConfig {
    pub fn is_cluster(&self) -> bool {
        self.mode == RedisMode::Cluster
    }

    pub fn get_url(&self) -> Option<String> {
        match self.mode {
            RedisMode::Single => self.url.clone(),
            RedisMode::Cluster => None,
        }
    }

    pub fn get_urls(&self) -> Option<Vec<String>> {
        match self.mode {
            RedisMode::Single => None,
            RedisMode::Cluster => self.urls.clone(),
        }
    }
}
