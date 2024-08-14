pub use errors::*;
pub use server_model::admin::{
    entities::{
        prelude::{SysDomain, SysRole, SysUser},
        sys_domain, sys_role, sys_user,
    },
    input::*,
    output::*,
};
pub use sys_auth_service::{handle_login_jwt, start_event_listener, SysAuthService, TAuthService};
pub use sys_domain_service::{SysDomainService, TDomainService};
pub use sys_role_service::{SysRoleService, TRoleService};
pub use sys_user_service::{SysUserService, TUserService};

pub mod errors;
mod sys_auth_service;
mod sys_domain_service;
mod sys_role_service;
mod sys_user_service;
