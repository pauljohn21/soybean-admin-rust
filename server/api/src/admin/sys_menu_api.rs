use std::sync::Arc;

use axum::Extension;
use server_core::web::{error::AppError, res::Res};
use server_service::admin::{MenuRoute, SysMenuService, TMenuService};

pub struct SysMenuApi;

impl SysMenuApi {
    pub async fn get_constant_routes(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Res<Vec<MenuRoute>>, AppError> {
        service.get_constant_routes().await.map(Res::new_data)
    }
}
