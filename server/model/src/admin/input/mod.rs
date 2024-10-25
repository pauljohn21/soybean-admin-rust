pub use sys_authentication::LoginInput;
pub use sys_domain::{CreateDomainInput, DomainPageRequest, UpdateDomainInput};
pub use sys_endpoint::EndpointPageRequest;
pub use sys_menu::{CreateMenuInput, UpdateMenuInput};
pub use sys_role::{CreateRoleInput, RolePageRequest, UpdateRoleInput};
pub use sys_user::{CreateUserInput, UpdateUserInput, UserPageRequest};

mod sys_authentication;
mod sys_domain;
mod sys_endpoint;
mod sys_menu;
mod sys_role;
mod sys_user;
