use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MongoConfig {
    /// MongoDB 连接 URI
    /// 支持以下格式：
    /// mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]
    ///
    /// 示例:
    /// - 基本连接：mongodb://localhost:27017/mydb
    /// - 带认证：mongodb://user:pass@localhost:27017/mydb
    /// - 带参数：mongodb://localhost:27017/mydb?maxPoolSize=20&w=majority
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MongoInstancesConfig {
    pub name: String,
    pub mongo: MongoConfig,
}
