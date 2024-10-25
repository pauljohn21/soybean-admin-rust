use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use server_constant::definition::Audience;
use server_core::web::{
    auth::Claims,
    error::AppError,
    jwt::{JwtError, JwtUtils},
};
use server_global::global::{get_dyn_event_receiver, get_dyn_event_sender};
use server_model::admin::{
    entities::{
        prelude::{SysRole, SysUser},
        sys_domain, sys_role, sys_user, sys_user_role,
    },
    input::LoginInput,
    output::{AuthOutput, UserWithDomainAndOrgOutput},
};
use server_utils::SecureUtil;
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::{error, instrument};
use ulid::Ulid;

use crate::{
    admin::{event_handlers::auth_event_handler::AuthEvent, sys_user_error::UserError},
    helper::db_helper,
    project_error, project_info,
};

macro_rules! select_user_with_domain_and_org_info {
    ($query:expr) => {{
        $query
            .select_only()
            .column_as(sys_user::Column::Id, "id")
            .column_as(sys_user::Column::Domain, "domain")
            .column_as(sys_user::Column::Username, "username")
            .column_as(sys_user::Column::Password, "password")
            .column_as(sys_user::Column::NickName, "nick_name")
            .column_as(sys_user::Column::Avatar, "avatar")
            .column_as(sys_domain::Column::Code, "domain_code")
            .column_as(sys_domain::Column::Name, "domain_name")
    }};
}
#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to send event: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<Box<dyn std::any::Any + Send>>),
    #[error("Failed to handle login event: {0}")]
    LoginHandlerError(String),
}

#[async_trait]
pub trait TAuthService {
    async fn pwd_login(&self, input: LoginInput, domain: &str) -> Result<AuthOutput, AppError>;
}

#[derive(Clone)]
pub struct SysAuthService;

#[async_trait]
impl TAuthService for SysAuthService {
    #[instrument(skip(self, input), fields(username = %input.identifier, domain = %domain))]
    async fn pwd_login(&self, input: LoginInput, domain: &str) -> Result<AuthOutput, AppError> {
        let db = db_helper::get_db_connection().await?;

        let user = select_user_with_domain_and_org_info!(SysUser::find())
            .filter(sys_user::Column::Username.eq(&input.identifier))
            .filter(sys_domain::Column::Code.eq(domain))
            .join(JoinType::InnerJoin, sys_user::Relation::SysDomain.def())
            .into_model::<UserWithDomainAndOrgOutput>()
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::from(UserError::UserNotFound))?;

        if !SecureUtil::verify_password(input.password.as_bytes(), &user.password)
            .map_err(|_| AppError::from(UserError::AuthenticationFailed))?
        {
            return Err(AppError::from(UserError::WrongPassword));
        }

        let user_id = user.id.clone();
        let username = user.username.clone();
        let domain_code = user.domain_code.clone();

        let role_codes: Vec<String> = SysRole::find()
            .join(JoinType::InnerJoin, sys_role::Relation::SysUserRole.def())
            .join(JoinType::InnerJoin, sys_user_role::Relation::SysUser.def())
            .filter(sys_user::Column::Id.eq(&user_id))
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?
            .iter()
            .filter_map(|role| Some(role.code.clone()))
            .collect();

        let auth_output = generate_auth_output(
            user_id.clone(),
            username.clone(),
            role_codes,
            domain_code.clone(),
            None,
        )
        .await
        .map_err(AppError::from)?;

        // 发送 AuthEvent
        if let Some(sender) = get_dyn_event_sender().await {
            let auth_event = AuthEvent {
                user_id,
                username,
                domain: domain_code,
                access_token: auth_output.access_token.clone(),
                refresh_token: auth_output.refresh_token.clone(),
            };

            if let Err(e) = send_auth_event(sender, auth_event).await {
                project_error!("Failed to send AuthEvent: {:?}", e);
            }
        }

        Ok(auth_output)
    }
}

#[instrument(skip(sender, auth_event))]
async fn send_auth_event(
    sender: mpsc::UnboundedSender<Box<dyn std::any::Any + Send>>,
    auth_event: AuthEvent,
) -> Result<(), EventError> {
    sender.send(Box::new(auth_event)).map_err(EventError::from)?;
    Ok(())
}

pub async fn generate_auth_output(
    user_id: String,
    username: String,
    role_codes: Vec<String>,
    domain_code: String,
    organization_name: Option<String>,
) -> Result<AuthOutput, JwtError> {
    let claims = Claims::new(
        user_id,
        Audience::ManagementPlatform.as_str().to_string(),
        username,
        role_codes,
        domain_code,
        organization_name,
    );

    let token = JwtUtils::generate_token(&claims).await?;

    Ok(AuthOutput {
        access_token: token,
        refresh_token: Ulid::new().to_string(),
    })
}

#[instrument]
pub async fn handle_login_jwt() {
    if let Some(mut receiver) = get_dyn_event_receiver().await {
        while let Some(event) = receiver.recv().await {
            if let Some(auth_event) = event.downcast_ref::<AuthEvent>() {
                if let Err(e) = handle_auth_event(auth_event).await {
                    project_error!("Failed to handle AuthEvent: {:?}", e);
                }
            }
        }
    }
}

#[instrument(skip(auth_event), fields(user_id = %auth_event.user_id, username = %auth_event.username))]
async fn handle_auth_event(auth_event: &AuthEvent) -> Result<(), EventError> {
    crate::admin::event_handlers::auth_event_handler::handle_successful_login(
        auth_event.user_id.clone(),
        auth_event.username.clone(),
        auth_event.domain.clone(),
        auth_event.access_token.clone(),
        auth_event.refresh_token.clone(),
    )
    .await
    .map_err(|e| EventError::LoginHandlerError(format!("{:?}", e)))
}

#[instrument(skip(rx))]
pub async fn start_event_listener(mut rx: tokio::sync::mpsc::UnboundedReceiver<String>) {
    while let Some(jwt) = rx.recv().await {
        project_info!("JWT created: {}", jwt);
        // TODO: Consider storing the token into the database
    }
}
