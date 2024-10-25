use server_core::web::error::AppError;
use ulid::Ulid;

use crate::{
    admin::events::{access_token_event::AccessTokenEvent, login_log_event::LoginLogEvent},
    helper::db_helper,
};

pub struct AuthEvent {
    pub user_id: String,
    pub username: String,
    pub domain: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthEventHandler;

impl AuthEventHandler {
    pub async fn handle_login(
        user_id: String,
        username: String,
        domain: String,
        access_token: String,
        refresh_token: String,
    ) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        let request_id = Ulid::new().to_string();

        // 处理登录日志
        let login_log_event = LoginLogEvent {
            user_id: user_id.clone(),
            username: username.clone(),
            domain: domain.clone(),
            ip: "127.0.0.1".to_string(),
            port: None,
            address: "Unknown".to_string(),
            user_agent: "Unknown".to_string(),
            request_id: request_id.clone(),
        };

        login_log_event.handle(&db).await?;

        // 处理访问令牌
        let access_token_event = AccessTokenEvent {
            access_token,
            refresh_token,
            user_id,
            username,
            domain,
            ip: "127.0.0.1".to_string(),
            port: None,
            address: "Unknown".to_string(),
            user_agent: "Unknown".to_string(),
            request_id,
        };

        access_token_event.handle(&db).await?;

        Ok(())
    }
}

pub async fn handle_successful_login(
    user_id: String,
    username: String,
    domain: String,
    access_token: String,
    refresh_token: String,
) -> Result<(), AppError> {
    AuthEventHandler::handle_login(user_id, username, domain, access_token, refresh_token).await
}
