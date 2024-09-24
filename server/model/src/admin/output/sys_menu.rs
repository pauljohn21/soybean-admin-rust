use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RouteMeta {
    pub title: String,
    pub i18n_key: Option<String>,
    pub keep_alive: Option<bool>,
    pub constant: bool,
    pub icon: Option<String>,
    pub order: i32,
    pub href: Option<String>,
    pub hide_in_menu: Option<bool>,
    pub active_menu: Option<String>,
    pub multi_tab: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MenuRoute {
    pub name: String,
    pub path: String,
    pub component: String,
    pub meta: RouteMeta,
    pub children: Option<Vec<MenuRoute>>,
}
