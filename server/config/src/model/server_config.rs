use serde::Deserialize;

/// 服务器配置
///
/// 支持的环境变量：
/// - APP_SERVER_HOST: 服务器监听地址
/// - APP_SERVER_PORT: 服务器监听端口
#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    /// 服务器监听地址
    /// 环境变量: APP_SERVER_HOST
    pub host: String,

    /// 服务器监听端口
    /// 环境变量: APP_SERVER_PORT
    pub port: u32,
}
