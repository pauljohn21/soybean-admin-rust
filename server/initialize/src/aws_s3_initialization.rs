#![allow(dead_code)]
use std::sync::Arc;

use aws_sdk_s3::Client as S3Client;
use server_global::global::{GLOBAL_PRIMARY_S3, GLOBAL_S3_POOL};

/// 初始化 S3 客户端
pub async fn initialize_s3() {
    //TODO: 初始化 S3 客户端
}

/// 设置主要的 S3 客户端
pub async fn set_primary_s3_client(client: S3Client) {
    let mut primary = GLOBAL_PRIMARY_S3.write().await;
    *primary = Some(Arc::new(client));
}

/// 获取主要的 S3 客户端
pub async fn get_primary_s3_client() -> Option<Arc<S3Client>> {
    GLOBAL_PRIMARY_S3.read().await.clone()
}

/// 添加或更新命名的 S3 客户端
pub async fn set_named_s3_client(name: &str, client: S3Client) {
    let mut pool = GLOBAL_S3_POOL.write().await;
    pool.insert(name.to_string(), Arc::new(client));
}

/// 获取命名的 S3 客户端，如果不存在返回 None
pub async fn get_named_s3_client(name: &str) -> Option<Arc<S3Client>> {
    GLOBAL_S3_POOL.read().await.get(name).cloned()
}

/// 移除命名的 S3 客户端，返回被移除的客户端
pub async fn remove_named_s3_client(name: &str) -> Option<Arc<S3Client>> {
    let mut pool = GLOBAL_S3_POOL.write().await;
    pool.remove(name)
}

/// 清空所有命名的 S3 客户端
pub async fn clear_s3_clients() {
    let mut pool = GLOBAL_S3_POOL.write().await;
    pool.clear();
}

/// 获取所有命名的 S3 客户端名称
pub async fn get_s3_client_names() -> Vec<String> {
    GLOBAL_S3_POOL.read().await.keys().cloned().collect()
}

/// 检查指定名称的客户端是否存在
pub async fn has_named_client(name: &str) -> bool {
    GLOBAL_S3_POOL.read().await.contains_key(name)
}
