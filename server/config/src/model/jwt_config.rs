use serde::Deserialize;

/// JWT 配置
///
/// 支持的环境变量：
/// - APP_JWT_JWT_SECRET: JWT 密钥
/// - APP_JWT_ISSUER: JWT 签发者
/// - APP_JWT_EXPIRE: JWT 过期时间（秒）
#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    /// JWT 密钥
    /// 环境变量: APP_JWT_JWT_SECRET
    pub jwt_secret: String,

    /// JWT 签发者
    /// 环境变量: APP_JWT_ISSUER
    pub issuer: String,

    /// JWT 过期时间（秒）
    /// 环境变量: APP_JWT_EXPIRE
    pub expire: i64,
}
