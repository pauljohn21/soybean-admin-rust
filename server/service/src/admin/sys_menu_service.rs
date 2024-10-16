use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use server_core::web::{auth::User, error::AppError};
use server_model::admin::{
    entities::{prelude::SysMenu, sea_orm_active_enums::Status, sys_menu},
    input::{CreateMenuInput, UpdateMenuInput},
    output::{MenuRoute, RouteMeta},
};

use crate::{admin::sys_menu_error::MenuError, helper::db_helper};

#[async_trait]
pub trait TMenuService {
    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, AppError>;

    async fn create_menu(
        &self,
        input: CreateMenuInput,
        user: User,
    ) -> Result<sys_menu::Model, AppError>;
    async fn get_menu(&self, id: i32) -> Result<sys_menu::Model, AppError>;
    async fn update_menu(
        &self,
        input: UpdateMenuInput,
        user: User,
    ) -> Result<sys_menu::Model, AppError>;
    async fn delete_menu(&self, id: i32, user: User) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysMenuService;

impl SysMenuService {
    async fn check_menu_exists(&self, id: Option<i32>, route_name: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        let route_name_exists = SysMenu::find()
            .filter(sys_menu::Column::RouteName.eq(route_name))
            .filter(sys_menu::Column::Id.ne(id.unwrap_or(-1)))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .is_some();

        if route_name_exists {
            return Err(MenuError::DuplicateRouteName.into());
        }

        Ok(())
    }
}

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

    async fn create_menu(
        &self,
        input: CreateMenuInput,
        user: User,
    ) -> Result<sys_menu::Model, AppError> {
        self.check_menu_exists(None, &input.route_name).await?;

        let db = db_helper::get_db_connection().await?;

        let menu = sys_menu::ActiveModel {
            menu_type: Set(input.menu_type),
            menu_name: Set(input.menu_name),
            icon_type: Set(input.icon_type),
            icon: Set(input.icon),
            route_name: Set(input.route_name),
            route_path: Set(input.route_path),
            component: Set(input.component),
            path_param: Set(input.path_param),
            status: Set(input.status),
            active_menu: Set(input.active_menu),
            hide_in_menu: Set(input.hide_in_menu),
            pid: Set(input.pid),
            sequence: Set(input.sequence),
            i18n_key: Set(input.i18n_key),
            keep_alive: Set(input.keep_alive),
            constant: Set(input.constant),
            href: Set(input.href),
            multi_tab: Set(input.multi_tab),

            created_by: Set(user.user_id()),
            ..Default::default()
        };

        let result = menu.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(result)
    }

    async fn get_menu(&self, id: i32) -> Result<sys_menu::Model, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysMenu::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| MenuError::MenuNotFound.into())
    }

    async fn update_menu(
        &self,
        input: UpdateMenuInput,
        user: User,
    ) -> Result<sys_menu::Model, AppError> {
        let db = db_helper::get_db_connection().await?;
        let existing_menu = self.get_menu(input.id).await?;

        self.check_menu_exists(Some(input.id), &input.menu.route_name).await?;

        let mut menu: sys_menu::ActiveModel = existing_menu.into();
        menu.menu_type = Set(input.menu.menu_type);
        menu.menu_name = Set(input.menu.menu_name);
        menu.icon_type = Set(input.menu.icon_type);
        menu.icon = Set(input.menu.icon);
        menu.route_name = Set(input.menu.route_name);
        menu.route_path = Set(input.menu.route_path);
        menu.component = Set(input.menu.component);
        menu.path_param = Set(input.menu.path_param);
        menu.status = Set(input.menu.status);
        menu.active_menu = Set(input.menu.active_menu);
        menu.hide_in_menu = Set(input.menu.hide_in_menu);
        menu.pid = Set(input.menu.pid);
        menu.sequence = Set(input.menu.sequence);
        menu.i18n_key = Set(input.menu.i18n_key);
        menu.keep_alive = Set(input.menu.keep_alive);
        menu.constant = Set(input.menu.constant);
        menu.href = Set(input.menu.href);
        menu.multi_tab = Set(input.menu.multi_tab);

        menu.updated_at = Set(Some(Utc::now().naive_utc()));
        menu.updated_by = Set(Some(user.user_id()));

        let updated_menu = menu.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(updated_menu)
    }

    async fn delete_menu(&self, id: i32, _user: User) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        SysMenu::delete_by_id(id).exec(db.as_ref()).await.map_err(AppError::from)?;
        Ok(())
    }
}
