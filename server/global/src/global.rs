use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use tokio::sync::{mpsc, Mutex, OnceCell, RwLock};

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

pub static GLOBAL_PRIMARY_DB: Lazy<RwLock<Option<Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(None));
pub static GLOBAL_DB_POOL: Lazy<RwLock<HashMap<String, Arc<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// static GLOBAL_REDIS_POOL: Lazy<Arc<RwLock<HashMap<String, RedisClient>>>> =
// Lazy::new(|| {     Arc::new(RwLock::new(HashMap::new()))
// });
//
// static GLOBAL_MONGO_POOL: Lazy<Arc<RwLock<HashMap<String, MongoClient>>>> =
// Lazy::new(|| {     Arc::new(RwLock::new(HashMap::new()))
// });

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

lazy_static! {
    pub static ref EVENT_SENDER: Mutex<Option<mpsc::UnboundedSender<String>>> = Mutex::new(None);
    pub static ref EVENT_RECEIVER: Mutex<Option<mpsc::UnboundedReceiver<String>>> =
        Mutex::new(None);
}

pub async fn initialize_global_event_channel() {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    *EVENT_SENDER.lock().await = Some(tx);
    *EVENT_RECEIVER.lock().await = Some(rx);
}

pub async fn get_event_sender() -> Option<mpsc::UnboundedSender<String>> {
    let lock = EVENT_SENDER.lock().await;
    lock.clone()
}

pub async fn get_event_receiver() -> Option<mpsc::UnboundedReceiver<String>> {
    EVENT_RECEIVER.lock().await.take()
}

lazy_static! {
    pub static ref DYN_EVENT_SENDER: Arc<Mutex<Option<mpsc::UnboundedSender<Box<dyn Any + Send>>>>> =
        Arc::new(Mutex::new(None));
    pub static ref DYN_EVENT_RECEIVER: Arc<Mutex<Option<mpsc::UnboundedReceiver<Box<dyn Any + Send>>>>> =
        Arc::new(Mutex::new(None));
}

pub async fn initialize_dyn_global_event_channel() {
    let (tx, rx) = mpsc::unbounded_channel::<Box<dyn Any + Send>>();
    *crate::global::DYN_EVENT_SENDER.lock().await = Some(tx);
    *crate::global::DYN_EVENT_RECEIVER.lock().await = Some(rx);
}

pub async fn get_dyn_event_sender() -> Option<mpsc::UnboundedSender<Box<dyn Any + Send>>> {
    let lock = crate::global::DYN_EVENT_SENDER.lock().await;
    lock.clone()
}

pub async fn get_dyn_event_receiver() -> Option<mpsc::UnboundedReceiver<Box<dyn Any + Send>>> {
    crate::global::DYN_EVENT_RECEIVER.lock().await.take()
}
