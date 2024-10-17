use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct RoleInput {
    pub pid: i64,
    #[validate(length(
        min = 1,
        max = 50,
        message = "Code must be between 1 and 50 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must be between 1 and 50 characters"
    ))]
    pub name: String,
    #[validate(length(max = 200, message = "Remark must not exceed 200 characters"))]
    pub remark: Option<String>,
}

pub type CreateRoleInput = RoleInput;

#[derive(Deserialize, Validate)]
pub struct UpdateRoleInput {
    pub id: i64,
    #[serde(flatten)]
    pub role: RoleInput,
}
