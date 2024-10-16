pub use sys_authentication::LoginInput;
pub use sys_domain::{CreateDomainInput, DomainPageRequest, UpdateDomainInput};
pub use sys_menu::{CreateMenuInput, UpdateMenuInput};
pub use sys_role::RolePageRequest;
pub use sys_user::UserPageRequest;

mod sys_authentication;
mod sys_domain;
mod sys_menu;
mod sys_role;
mod sys_user;
