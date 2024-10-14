use axum::{
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysDomainApi;

pub struct SysDomainRouter;

impl SysDomainRouter {
    pub async fn init_domain_router() -> Router {
        let router = Router::new()
            .route("/list", get(SysDomainApi::get_paginated_domains))
            .route("/", post(SysDomainApi::create_domain))
            .route("/:id", get(SysDomainApi::get_domain))
            .route("/", put(SysDomainApi::update_domain))
            .route("/:id", delete(SysDomainApi::delete_domain));
        Router::new().nest("/domain", router)
    }
}
