use std::{net::SocketAddr, sync::Arc};

use axum::{extract::ConnectInfo, http::HeaderMap, Extension};
use axum_extra::{headers::UserAgent, TypedHeader};
use server_core::web::{
    auth::User, error::AppError, res::Res, util::ClientIp, validator::ValidatedForm, RequestId,
};
use server_service::{
    admin::{
        dto::sys_auth_dto::LoginContext, AuthOutput, LoginInput, SysAuthService, TAuthService,
        UserInfoOutput,
    },
    Audience,
};

pub struct SysAuthenticationApi;

impl SysAuthenticationApi {
    pub async fn login_handler(
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        headers: HeaderMap,
        TypedHeader(user_agent): TypedHeader<UserAgent>,
        Extension(request_id): Extension<RequestId>,
        Extension(service): Extension<Arc<SysAuthService>>,
        ValidatedForm(input): ValidatedForm<LoginInput>,
    ) -> Result<Res<AuthOutput>, AppError> {
        let client_ip = {
            let header_ip = ClientIp::get_real_ip(&headers);
            if header_ip == "unknown" {
                addr.ip().to_string()
            } else {
                header_ip
            }
        };

        let address = xdb::searcher::search_by_ip(client_ip.as_str())
            .unwrap_or_else(|_| "Unknown Location".to_string());

        let login_context = LoginContext {
            client_ip,
            client_port: Some(addr.port() as i32),
            address,
            user_agent: user_agent.as_str().to_string(),
            request_id: request_id.to_string(),
            audience: Audience::ManagementPlatform,
            login_type: "PC".to_string(),
            domain: "built-in".to_string(),
        };

        service.pwd_login(input, login_context).await.map(Res::new_data)
    }

    pub async fn get_user_info(
        Extension(user): Extension<User>,
    ) -> Result<Res<UserInfoOutput>, AppError> {
        let user_info = UserInfoOutput {
            user_id: user.user_id(),
            user_name: user.username(),
            roles: user.subject(),
        };

        Ok(Res::new_data(user_info))
    }
}
