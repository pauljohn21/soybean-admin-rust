use async_trait::async_trait;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
};
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
        sea_orm_active_enums::Status,
        sys_domain,
        sys_menu::{self},
        sys_role, sys_role_menu,
        sys_user::{self},
        sys_user_role,
    },
    input::LoginInput,
    output::{AuthOutput, MenuRoute, RouteMeta, UserRoute, UserWithDomainAndOrgOutput},
};
use server_utils::{SecureUtil, TreeBuilder};
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::{error, instrument};
use ulid::Ulid;

use super::{
    dto::sys_auth_dto::LoginContext, event_handlers::auth_event_handler::AuthEventHandler,
};
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
pub trait TAuthService: Send + Sync {
    async fn pwd_login(
        &self,
        input: LoginInput,
        context: LoginContext,
    ) -> Result<AuthOutput, AppError>;

    async fn get_user_routes(
        &self,
        role_codes: &[String],
        domain: &str,
    ) -> Result<UserRoute, AppError>;
}

#[derive(Clone)]
pub struct SysAuthService;

impl SysAuthService {
    /// 查找第一个有效的路由路径
    fn find_first_valid_route(routes: &[MenuRoute]) -> Option<String> {
        for route in routes {
            if !route.path.is_empty() && route.path != "/" {
                return Some(route.path.clone());
            }
            if let Some(children) = &route.children {
                if let Some(path) = Self::find_first_valid_route(children) {
                    return Some(path);
                }
            }
        }
        None
    }
}

#[async_trait]
impl TAuthService for SysAuthService {
    #[instrument(skip(self, input), fields(username = %input.identifier, domain = %context.domain))]
    async fn pwd_login(
        &self,
        input: LoginInput,
        context: LoginContext,
    ) -> Result<AuthOutput, AppError> {
        // 验证用户并获取角色
        let (user, role_codes) =
            self.verify_user(&input.identifier, &input.password, &context.domain).await?;

        // 生成认证输出
        let auth_output = generate_auth_output(
            user.id.clone(),
            user.username.clone(),
            role_codes,
            user.domain_code.clone(),
            None,
            context.audience,
        )
        .await?;

        // 发送认证事件
        self.send_login_event(&user, &auth_output, &context).await;

        Ok(auth_output)
    }

    #[instrument(skip(self), fields(roles = ?role_codes, domain = %domain))]
    async fn get_user_routes(
        &self,
        role_codes: &[String],
        domain: &str,
    ) -> Result<UserRoute, AppError> {
        if role_codes.is_empty() {
            return Ok(UserRoute {
                routes: vec![],
                home: "/home".to_string(),
            });
        }

        let db = db_helper::get_db_connection().await?;

        let menu_ids = sys_role_menu::Entity::find()
            .select_only()
            .column(sys_role_menu::Column::MenuId)
            .join_rev(JoinType::InnerJoin, sys_role::Entity::has_many(sys_role_menu::Entity).into())
            .filter(sys_role::Column::Code.is_in(role_codes.to_vec()))
            .filter(sys_role_menu::Column::Domain.eq(domain))
            .distinct()
            .into_tuple::<i32>()
            .all(db.as_ref())
            .await?;

        let menus = sys_menu::Entity::find()
            .filter(sys_menu::Column::Id.is_in(menu_ids))
            .filter(sys_menu::Column::Status.eq(Status::Enabled))
            .order_by_asc(sys_menu::Column::Sequence)
            .into_model::<sys_menu::Model>()
            .all(db.as_ref())
            .await?;

        let menu_routes: Vec<MenuRoute> = menus
            .into_iter()
            .map(|menu| MenuRoute {
                name: menu.route_name,
                path: menu.route_path,
                component: menu.component,
                meta: RouteMeta {
                    title: menu.menu_name,
                    i18n_key: menu.i18n_key,
                    keep_alive: menu.keep_alive,
                    constant: menu.constant,
                    icon: menu.icon,
                    order: menu.sequence,
                    href: menu.href,
                    hide_in_menu: menu.hide_in_menu,
                    active_menu: menu.active_menu,
                    multi_tab: menu.multi_tab,
                },
                children: Some(vec![]),
                id: menu.id,
                pid: menu.pid,
            })
            .collect();

        let menu_routes_ref = menu_routes.clone();

        let routes = TreeBuilder::build(
            menu_routes,
            |route| route.name.clone(),
            |route| {
                if route.pid == "0" {
                    None
                } else {
                    menu_routes_ref
                        .iter()
                        .find(|m| m.id.to_string() == route.pid)
                        .map(|m| m.name.clone())
                }
            },
            |route| route.meta.order,
            |route, children| {
                route.children = Some(children);
            },
        );

        let home = Self::find_first_valid_route(&routes).unwrap_or_else(|| "/home".to_string());

        Ok(UserRoute { routes, home })
    }
}

impl SysAuthService {
    /// 验证用户身份
    async fn verify_user(
        &self,
        identifier: &str,
        password: &str,
        domain: &str,
    ) -> Result<(UserWithDomainAndOrgOutput, Vec<String>), AppError> {
        let db = db_helper::get_db_connection().await?;

        let user = select_user_with_domain_and_org_info!(SysUser::find())
            .filter(sys_user::Column::Username.eq(identifier))
            .filter(sys_domain::Column::Code.eq(domain))
            .join(JoinType::InnerJoin, sys_user::Relation::SysDomain.def())
            .into_model::<UserWithDomainAndOrgOutput>()
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::from(UserError::UserNotFound))?;

        // 验证密码
        if !SecureUtil::verify_password(password.as_bytes(), &user.password)
            .map_err(|_| AppError::from(UserError::AuthenticationFailed))?
        {
            return Err(AppError::from(UserError::WrongPassword));
        }

        // 获取角色
        let role_codes = self.get_user_roles(&user.id, &db).await?;

        Ok((user, role_codes))
    }

    /// 获取用户角色
    async fn get_user_roles(
        &self,
        user_id: &str,
        db: &DatabaseConnection,
    ) -> Result<Vec<String>, AppError> {
        SysRole::find()
            .join(JoinType::InnerJoin, sys_role::Relation::SysUserRole.def())
            .join(JoinType::InnerJoin, sys_user_role::Relation::SysUser.def())
            .filter(sys_user::Column::Id.eq(user_id))
            .all(db)
            .await
            .map(|roles| roles.iter().map(|role| role.code.clone()).collect())
            .map_err(AppError::from)
    }

    async fn send_login_event(
        &self,
        user: &UserWithDomainAndOrgOutput,
        auth_output: &AuthOutput,
        context: &LoginContext,
    ) {
        if let Some(sender) = get_dyn_event_sender().await {
            let auth_event = AuthEvent {
                user_id: user.id.clone(),
                username: user.username.clone(),
                domain: user.domain_code.clone(),
                access_token: auth_output.token.clone(),
                refresh_token: auth_output.refresh_token.clone(),
                client_ip: context.client_ip.clone(),
                client_port: context.client_port,
                address: context.address.clone(),
                user_agent: context.user_agent.clone(),
                request_id: context.request_id.clone(),
                login_type: context.login_type.clone(),
            };

            // 使用 spawn 异步处理事件，避免阻塞登录流程
            tokio::spawn(async move {
                if let Err(e) = send_auth_event(sender, auth_event).await {
                    // 记录错误但不影响主流程
                    project_error!("Failed to send AuthEvent: {:?}", e);
                    // TODO: 可以添加重试机制或将失败事件写入特定队列
                }
            });
        }
    }

    #[allow(dead_code)]
    async fn check_login_security(
        &self,
        _username: &str,
        _client_ip: &str,
    ) -> Result<(), AppError> {
        // TODO: 实现登录安全检查
        // 1. 检查登录失败次数
        // 2. 检查 IP 黑名单
        // 3. 检查账号是否被锁定
        // 4. 检查是否在允许的时间范围内
        Ok(())
    }

    #[allow(dead_code)]
    async fn pwd_login_with_security(
        &self,
        input: LoginInput,
        context: LoginContext,
    ) -> Result<AuthOutput, AppError> {
        self.check_login_security(&input.identifier, &context.client_ip).await?;

        self.pwd_login(input, context).await
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
    audience: Audience,
) -> Result<AuthOutput, JwtError> {
    let claims = Claims::new(
        user_id,
        audience.as_str().to_string(),
        username,
        role_codes,
        domain_code,
        organization_name,
    );

    let token = JwtUtils::generate_token(&claims).await?;

    Ok(AuthOutput {
        token,
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
    AuthEventHandler::handle_login(AuthEvent {
        user_id: auth_event.user_id.clone(),
        username: auth_event.username.clone(),
        domain: auth_event.domain.clone(),
        access_token: auth_event.access_token.clone(),
        refresh_token: auth_event.refresh_token.clone(),
        client_ip: auth_event.client_ip.clone(),
        address: auth_event.address.clone(),
        client_port: auth_event.client_port,
        user_agent: auth_event.user_agent.clone(),
        request_id: auth_event.request_id.clone(),
        login_type: auth_event.login_type.clone(),
    })
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
