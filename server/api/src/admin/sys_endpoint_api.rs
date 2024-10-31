use std::sync::Arc;

use axum::{extract::Query, Extension};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    EndpointPageRequest, SysEndpointModel, SysEndpointService, TEndpointService,
};

pub struct SysEndpointApi;

impl SysEndpointApi {
    pub async fn get_paginated_endpoints(
        Query(params): Query<EndpointPageRequest>,
        Extension(service): Extension<Arc<SysEndpointService>>,
    ) -> Result<Res<PaginatedData<SysEndpointModel>>, AppError> {
        service.find_paginated_endpoints(params).await.map(Res::new_data)
    }
}
