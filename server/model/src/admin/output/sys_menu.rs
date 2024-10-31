use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct MenuRoute {
    pub name: String,
    pub path: String,
    pub component: String,
    pub meta: RouteMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuRoute>>,
    pub id: i32,
    pub pid: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct RouteMeta {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "i18nKey")]
    pub i18n_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    pub constant: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hideInMenu")]
    pub hide_in_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeMenu")]
    pub active_menu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multiTab")]
    pub multi_tab: Option<bool>,
}
