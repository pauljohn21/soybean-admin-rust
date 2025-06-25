use serde::Deserialize;

/// MongoDB 配置
///
/// 支持的环境变量：
/// - APP_MONGO_URI: MongoDB 连接 URI
#[derive(Debug, Clone, Deserialize)]
pub struct MongoConfig {
    /// MongoDB 连接 URI
    /// 环境变量: APP_MONGO_URI
    /// 支持以下格式：
    /// mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]
    ///
    /// 示例:
    /// - 基本连接：mongodb://localhost:27017/mydb
    /// - 带认证：mongodb://user:pass@localhost:27017/mydb
    /// - 带参数：mongodb://localhost:27017/mydb?maxPoolSize=20&w=majority
    pub uri: String,
}

/// MongoDB 实例配置
///
/// 支持的环境变量（数组形式）：
/// - APP_MONGO_INSTANCES_0_NAME: 第一个实例名称
/// - APP_MONGO_INSTANCES_0_MONGO_URI: 第一个实例URI
/// - APP_MONGO_INSTANCES_1_NAME: 第二个实例名称
/// - APP_MONGO_INSTANCES_1_MONGO_URI: 第二个实例URI
/// 以此类推...
#[derive(Debug, Clone, Deserialize)]
pub struct MongoInstancesConfig {
    /// 实例名称
    pub name: String,

    /// MongoDB 配置
    pub mongo: MongoConfig,
}
