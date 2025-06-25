use crate::{
    DatabaseConfig, DatabasesInstancesConfig, MongoConfig, MongoInstancesConfig, RedisConfig,
    RedisInstancesConfig, RedisMode, S3Config, S3InstancesConfig,
};
use std::env;

/// 多实例环境变量处理器
///
/// 专门处理形如 APP_TYPE_INSTANCES_INDEX_FIELD 的环境变量
/// 例如：APP_DATABASE_INSTANCES_0_NAME=test
pub struct MultiInstanceEnvProcessor {
    prefix: String,
}

impl MultiInstanceEnvProcessor {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    /// 从环境变量中解析数据库实例配置
    pub fn parse_database_instances(&self) -> Vec<DatabasesInstancesConfig> {
        let mut instances = Vec::new();
        let mut index = 0;

        loop {
            let name_key = format!("{}_DATABASE_INSTANCES_{}_NAME", self.prefix, index);
            let url_key = format!("{}_DATABASE_INSTANCES_{}_DATABASE_URL", self.prefix, index);

            if let (Ok(name), Ok(url)) = (env::var(&name_key), env::var(&url_key)) {
                let max_connections_key = format!(
                    "{}_DATABASE_INSTANCES_{}_DATABASE_MAX_CONNECTIONS",
                    self.prefix, index
                );
                let min_connections_key = format!(
                    "{}_DATABASE_INSTANCES_{}_DATABASE_MIN_CONNECTIONS",
                    self.prefix, index
                );
                let connect_timeout_key = format!(
                    "{}_DATABASE_INSTANCES_{}_DATABASE_CONNECT_TIMEOUT",
                    self.prefix, index
                );
                let idle_timeout_key = format!(
                    "{}_DATABASE_INSTANCES_{}_DATABASE_IDLE_TIMEOUT",
                    self.prefix, index
                );

                let max_connections = env::var(&max_connections_key)
                    .unwrap_or_else(|_| "10".to_string())
                    .parse::<u32>()
                    .unwrap_or(10);

                let min_connections = env::var(&min_connections_key)
                    .unwrap_or_else(|_| "1".to_string())
                    .parse::<u32>()
                    .unwrap_or(1);

                let connect_timeout = env::var(&connect_timeout_key)
                    .unwrap_or_else(|_| "30".to_string())
                    .parse::<u64>()
                    .unwrap_or(30);

                let idle_timeout = env::var(&idle_timeout_key)
                    .unwrap_or_else(|_| "600".to_string())
                    .parse::<u64>()
                    .unwrap_or(600);

                instances.push(DatabasesInstancesConfig {
                    name,
                    database: DatabaseConfig {
                        url,
                        max_connections,
                        min_connections,
                        connect_timeout,
                        idle_timeout,
                    },
                });

                index += 1;
            } else {
                break;
            }
        }

        instances
    }

    /// 从环境变量中解析 Redis 实例配置
    pub fn parse_redis_instances(&self) -> Vec<RedisInstancesConfig> {
        let mut instances = Vec::new();
        let mut index = 0;

        loop {
            let name_key = format!("{}_REDIS_INSTANCES_{}_NAME", self.prefix, index);
            let mode_key = format!("{}_REDIS_INSTANCES_{}_REDIS_MODE", self.prefix, index);

            if let (Ok(name), Ok(mode_str)) = (env::var(&name_key), env::var(&mode_key)) {
                let mode = match mode_str.to_lowercase().as_str() {
                    "single" => RedisMode::Single,
                    "cluster" => RedisMode::Cluster,
                    _ => RedisMode::Single,
                };

                let url_key = format!("{}_REDIS_INSTANCES_{}_REDIS_URL", self.prefix, index);
                let urls_key = format!("{}_REDIS_INSTANCES_{}_REDIS_URLS", self.prefix, index);

                let url = env::var(&url_key).ok();
                let urls = env::var(&urls_key).ok().map(|s| {
                    s.split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>()
                });

                instances.push(RedisInstancesConfig {
                    name,
                    redis: RedisConfig { mode, url, urls },
                });

                index += 1;
            } else {
                break;
            }
        }

        instances
    }

    /// 从环境变量中解析 MongoDB 实例配置
    pub fn parse_mongo_instances(&self) -> Vec<MongoInstancesConfig> {
        let mut instances = Vec::new();
        let mut index = 0;

        loop {
            let name_key = format!("{}_MONGO_INSTANCES_{}_NAME", self.prefix, index);
            let uri_key = format!("{}_MONGO_INSTANCES_{}_MONGO_URI", self.prefix, index);

            if let (Ok(name), Ok(uri)) = (env::var(&name_key), env::var(&uri_key)) {
                instances.push(MongoInstancesConfig {
                    name,
                    mongo: MongoConfig { uri },
                });

                index += 1;
            } else {
                break;
            }
        }

        instances
    }

    /// 从环境变量中解析 S3 实例配置
    pub fn parse_s3_instances(&self) -> Vec<S3InstancesConfig> {
        let mut instances = Vec::new();
        let mut index = 0;

        loop {
            let name_key = format!("{}_S3_INSTANCES_{}_NAME", self.prefix, index);
            let region_key = format!("{}_S3_INSTANCES_{}_S3_REGION", self.prefix, index);
            let access_key_id_key =
                format!("{}_S3_INSTANCES_{}_S3_ACCESS_KEY_ID", self.prefix, index);
            let secret_access_key_key = format!(
                "{}_S3_INSTANCES_{}_S3_SECRET_ACCESS_KEY",
                self.prefix, index
            );

            if let (Ok(name), Ok(region), Ok(access_key_id), Ok(secret_access_key)) = (
                env::var(&name_key),
                env::var(&region_key),
                env::var(&access_key_id_key),
                env::var(&secret_access_key_key),
            ) {
                let endpoint_key = format!("{}_S3_INSTANCES_{}_S3_ENDPOINT", self.prefix, index);
                let endpoint = env::var(&endpoint_key).ok();

                instances.push(S3InstancesConfig {
                    name,
                    s3: S3Config {
                        region,
                        access_key_id,
                        secret_access_key,
                        endpoint,
                    },
                });

                index += 1;
            } else {
                break;
            }
        }

        instances
    }

    /// 检查是否有任何多实例环境变量
    pub fn has_any_instances(&self) -> bool {
        let patterns = [
            format!("{}_DATABASE_INSTANCES_0_NAME", self.prefix),
            format!("{}_REDIS_INSTANCES_0_NAME", self.prefix),
            format!("{}_MONGO_INSTANCES_0_NAME", self.prefix),
            format!("{}_S3_INSTANCES_0_NAME", self.prefix),
        ];

        patterns.iter().any(|key| env::var(key).is_ok())
    }

    /// 打印所有找到的多实例配置（用于调试）
    pub fn debug_print_instances(&self) {
        let db_instances = self.parse_database_instances();
        let redis_instances = self.parse_redis_instances();
        let mongo_instances = self.parse_mongo_instances();
        let s3_instances = self.parse_s3_instances();

        if !db_instances.is_empty() {
            println!(
                "Found {} database instances from environment variables:",
                db_instances.len()
            );
            for (i, instance) in db_instances.iter().enumerate() {
                println!("  [{}] {} -> {}", i, instance.name, instance.database.url);
            }
        }

        if !redis_instances.is_empty() {
            println!(
                "Found {} Redis instances from environment variables:",
                redis_instances.len()
            );
            for (i, instance) in redis_instances.iter().enumerate() {
                println!("  [{}] {} -> {:?}", i, instance.name, instance.redis.mode);
            }
        }

        if !mongo_instances.is_empty() {
            println!(
                "Found {} MongoDB instances from environment variables:",
                mongo_instances.len()
            );
            for (i, instance) in mongo_instances.iter().enumerate() {
                println!("  [{}] {} -> {}", i, instance.name, instance.mongo.uri);
            }
        }

        if !s3_instances.is_empty() {
            println!(
                "Found {} S3 instances from environment variables:",
                s3_instances.len()
            );
            for (i, instance) in s3_instances.iter().enumerate() {
                println!("  [{}] {} -> {}", i, instance.name, instance.s3.region);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_parse_database_instances() {
        // 设置测试环境变量
        env::set_var("TEST_DATABASE_INSTANCES_0_NAME", "test_db");
        env::set_var(
            "TEST_DATABASE_INSTANCES_0_DATABASE_URL",
            "postgres://test@localhost:5432/test",
        );
        env::set_var("TEST_DATABASE_INSTANCES_0_DATABASE_MAX_CONNECTIONS", "5");

        env::set_var("TEST_DATABASE_INSTANCES_1_NAME", "analytics_db");
        env::set_var(
            "TEST_DATABASE_INSTANCES_1_DATABASE_URL",
            "postgres://analytics@localhost:5432/analytics",
        );
        env::set_var("TEST_DATABASE_INSTANCES_1_DATABASE_MAX_CONNECTIONS", "10");

        let processor = MultiInstanceEnvProcessor::new("TEST");
        let instances = processor.parse_database_instances();

        assert_eq!(instances.len(), 2);
        assert_eq!(instances[0].name, "test_db");
        assert_eq!(
            instances[0].database.url,
            "postgres://test@localhost:5432/test"
        );
        assert_eq!(instances[0].database.max_connections, 5);

        assert_eq!(instances[1].name, "analytics_db");
        assert_eq!(
            instances[1].database.url,
            "postgres://analytics@localhost:5432/analytics"
        );
        assert_eq!(instances[1].database.max_connections, 10);

        // 清理环境变量
        env::remove_var("TEST_DATABASE_INSTANCES_0_NAME");
        env::remove_var("TEST_DATABASE_INSTANCES_0_DATABASE_URL");
        env::remove_var("TEST_DATABASE_INSTANCES_0_DATABASE_MAX_CONNECTIONS");
        env::remove_var("TEST_DATABASE_INSTANCES_1_NAME");
        env::remove_var("TEST_DATABASE_INSTANCES_1_DATABASE_URL");
        env::remove_var("TEST_DATABASE_INSTANCES_1_DATABASE_MAX_CONNECTIONS");
    }

    #[test]
    fn test_parse_redis_instances() {
        // 设置测试环境变量
        env::set_var("TEST_REDIS_INSTANCES_0_NAME", "cache");
        env::set_var("TEST_REDIS_INSTANCES_0_REDIS_MODE", "single");
        env::set_var(
            "TEST_REDIS_INSTANCES_0_REDIS_URL",
            "redis://localhost:6379/0",
        );

        env::set_var("TEST_REDIS_INSTANCES_1_NAME", "cluster_cache");
        env::set_var("TEST_REDIS_INSTANCES_1_REDIS_MODE", "cluster");
        env::set_var(
            "TEST_REDIS_INSTANCES_1_REDIS_URLS",
            "redis://host1:7001,redis://host2:7002",
        );

        let processor = MultiInstanceEnvProcessor::new("TEST");
        let instances = processor.parse_redis_instances();

        assert_eq!(instances.len(), 2);
        assert_eq!(instances[0].name, "cache");
        assert_eq!(instances[0].redis.mode, RedisMode::Single);
        assert_eq!(
            instances[0].redis.url,
            Some("redis://localhost:6379/0".to_string())
        );

        assert_eq!(instances[1].name, "cluster_cache");
        assert_eq!(instances[1].redis.mode, RedisMode::Cluster);
        assert_eq!(
            instances[1].redis.urls,
            Some(vec![
                "redis://host1:7001".to_string(),
                "redis://host2:7002".to_string()
            ])
        );

        // 清理环境变量
        env::remove_var("TEST_REDIS_INSTANCES_0_NAME");
        env::remove_var("TEST_REDIS_INSTANCES_0_REDIS_MODE");
        env::remove_var("TEST_REDIS_INSTANCES_0_REDIS_URL");
        env::remove_var("TEST_REDIS_INSTANCES_1_NAME");
        env::remove_var("TEST_REDIS_INSTANCES_1_REDIS_MODE");
        env::remove_var("TEST_REDIS_INSTANCES_1_REDIS_URLS");
    }
}
