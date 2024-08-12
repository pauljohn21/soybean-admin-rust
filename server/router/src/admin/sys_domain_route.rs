use axum::{routing::get, Router};
use server_api::admin::SysDomainApi;

pub struct SysDomainRouter;

impl SysDomainRouter {
    pub async fn init_domain_router() -> Router {
        let router = Router::new().route("/", get(SysDomainApi::get_paginated_domains));
        Router::new().nest("/domain", router)
    }
}
