use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use http::Method;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use tokio::sync::{mpsc, Mutex, OnceCell, RwLock};

//*****************************************************************************
// 全局配置
//*****************************************************************************

pub static GLOBAL_CONFIG: Lazy<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn init_config<T: 'static + Any + Send + Sync>(config: T) {
    let mut context = GLOBAL_CONFIG.write().await;
    context.insert(TypeId::of::<T>(), Arc::new(config));
}

pub async fn get_config<T: 'static + Any + Send + Sync>() -> Option<Arc<T>> {
    let context = GLOBAL_CONFIG.read().await;
    context
        .get(&TypeId::of::<T>())
        .and_then(|config| config.clone().downcast::<T>().ok())
}

//*****************************************************************************
// 数据库连接
//*****************************************************************************

pub static GLOBAL_PRIMARY_DB: Lazy<RwLock<Option<Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(None));
pub static GLOBAL_DB_POOL: Lazy<RwLock<HashMap<String, Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

//*****************************************************************************
// JWT 密钥和验证
//*****************************************************************************

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: OnceCell<Arc<Mutex<Keys>>> = OnceCell::const_new();
pub static VALIDATION: OnceCell<Arc<Mutex<Validation>>> = OnceCell::const_new();

//*****************************************************************************
// 事件通道
//*****************************************************************************

pub struct EventChannels {
    pub string_sender: mpsc::UnboundedSender<String>,
    pub string_receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<String>>>>,
    pub dyn_sender: mpsc::UnboundedSender<Box<dyn Any + Send>>,
    pub dyn_receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<Box<dyn Any + Send>>>>>,
}

pub static EVENT_CHANNELS: Lazy<Mutex<Option<EventChannels>>> = Lazy::new(|| Mutex::new(None));

pub async fn initialize_event_channels() {
    let (string_tx, string_rx) = mpsc::unbounded_channel::<String>();
    let (dyn_tx, dyn_rx) = mpsc::unbounded_channel::<Box<dyn Any + Send>>();

    let channels = EventChannels {
        string_sender: string_tx,
        string_receiver: Arc::new(Mutex::new(Some(string_rx))),
        dyn_sender: dyn_tx,
        dyn_receiver: Arc::new(Mutex::new(Some(dyn_rx))),
    };

    *EVENT_CHANNELS.lock().await = Some(channels);
}

pub async fn get_string_event_sender() -> Option<mpsc::UnboundedSender<String>> {
    EVENT_CHANNELS
        .lock()
        .await
        .as_ref()
        .map(|channels| channels.string_sender.clone())
}

pub async fn get_string_event_receiver() -> Option<mpsc::UnboundedReceiver<String>> {
    if let Some(channels) = EVENT_CHANNELS.lock().await.as_ref() {
        channels.string_receiver.lock().await.take()
    } else {
        None
    }
}

pub async fn get_dyn_event_sender() -> Option<mpsc::UnboundedSender<Box<dyn Any + Send>>> {
    EVENT_CHANNELS.lock().await.as_ref().map(|channels| channels.dyn_sender.clone())
}

pub async fn get_dyn_event_receiver() -> Option<mpsc::UnboundedReceiver<Box<dyn Any + Send>>> {
    if let Some(channels) = EVENT_CHANNELS.lock().await.as_ref() {
        channels.dyn_receiver.lock().await.take()
    } else {
        None
    }
}

//*****************************************************************************
// 路由信息收集
//*****************************************************************************

#[derive(Clone, Debug)]
pub struct RouteInfo {
    pub path: String,
    pub method: Method,
    pub service_name: String,
    pub summary: String,
}

impl RouteInfo {
    pub fn new(path: &str, method: Method, service_name: &str, summary: &str) -> Self {
        RouteInfo {
            path: path.to_string(),
            method,
            service_name: service_name.to_string(),
            summary: summary.to_string(),
        }
    }
}

pub static ROUTE_COLLECTOR: Lazy<Mutex<Vec<RouteInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub async fn add_route(route: RouteInfo) {
    ROUTE_COLLECTOR.lock().await.push(route);
}

pub async fn get_collected_routes() -> Vec<RouteInfo> {
    ROUTE_COLLECTOR.lock().await.clone()
}

pub async fn clear_routes() {
    ROUTE_COLLECTOR.lock().await.clear();
}
