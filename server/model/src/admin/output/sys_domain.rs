use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult)]
pub struct DomainOutput {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub remark: Option<String>,
}
