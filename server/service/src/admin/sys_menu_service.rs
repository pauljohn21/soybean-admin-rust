use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use server_core::web::error::AppError;
use server_model::admin::{
    entities::{prelude::SysMenu, sea_orm_active_enums::Status, sys_menu},
    output::{MenuRoute, RouteMeta},
};

use crate::helper::db_helper;

#[async_trait]
pub trait TMenuService {
    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, AppError>;
}

#[derive(Clone)]
pub struct SysMenuService;

#[async_trait]
impl TMenuService for SysMenuService {
    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, AppError> {
        let db = db_helper::get_db_connection().await?;

        let menus: Vec<sys_menu::Model> = SysMenu::find()
            .filter(sys_menu::Column::Constant.eq(true))
            .filter(sys_menu::Column::Status.eq(Status::Enabled))
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let result = menus
            .into_iter()
            .map(|menu| MenuRoute {
                name: menu.route_name,
                path: menu.route_path,
                component: menu.component,
                meta: RouteMeta {
                    title: menu.menu_name,
                    i18n_key: menu.i18n_key,
                    keep_alive: menu.keep_alive,
                    constant: menu.constant,
                    icon: menu.icon,
                    order: menu.sequence,
                    href: menu.href,
                    hide_in_menu: menu.hide_in_menu,
                    active_menu: menu.active_menu,
                    multi_tab: menu.multi_tab,
                },
                children: vec![].into(),
            })
            .collect();

        Ok(result)
    }
}
