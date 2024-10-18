use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::Serialize;

use crate::admin::entities::sys_user;

#[derive(Debug, FromQueryResult)]
pub struct UserWithDomainAndOrgOutput {
    pub id: i64,
    pub domain_id: i64,
    pub org_id: Option<i64>,
    pub username: String,
    pub password: String,
    pub nick_name: String,
    pub avatar: Option<String>,
    pub domain_code: String,
    pub domain_name: String,
    pub organization_id: Option<i64>,
    pub organization_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserWithoutPassword {
    pub id: i64,
    pub domain_id: i64,
    pub org_id: Option<i64>,
    pub username: String,
    pub nick_name: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<sys_user::Model> for UserWithoutPassword {
    fn from(model: sys_user::Model) -> Self {
        Self {
            id: model.id,
            domain_id: model.domain_id,
            org_id: model.org_id,
            username: model.username,
            nick_name: model.nick_name,
            avatar: model.avatar,
            email: model.email,
            phone: model.phone,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
