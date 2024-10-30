use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct AuthOutput {
    pub token: String,
    // 为了复用soybean-admin-nestjs前端,暂时弃用
    // pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoOutput {
    pub user_id: String,
    pub user_name: String,
    pub roles: Vec<String>,
}
