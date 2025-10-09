use std::sync::{Arc, RwLock};

use bb8::Pool;
use bb8_redis::{
    RedisConnectionManager,
    redis::{AsyncCommands, RedisResult},
};
use commonx::error::AppError;
use serde::Serialize;

#[derive(Debug)]
pub struct RedisCache {
    pool: Pool<RedisConnectionManager>,
    namespace: Arc<RwLock<String>>,
}

impl RedisCache {
    pub async fn new(redis_url: &str, namespace: String) -> Result<Self, AppError> {
        let manager = RedisConnectionManager::new(redis_url)
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        let pool = Pool::builder()
            .build(manager)
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        Ok(Self {
            pool,
            namespace: Arc::new(RwLock::new(namespace)),
        })
    }

    pub async fn set_value_ex<T>(&self, k: &str, value: &T, t: i32) -> Result<bool, AppError>
    where
        T: Serialize + Sync,
    {
        let value_str = serde_json::to_string(value)?;
        self.set_string_ex(k, &value_str, t).await
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, t: i32) -> Result<bool, AppError> {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self.pool.get().await?;
        let result: RedisResult<()> = conn.set_ex(&key, v, t as u64).await;
        Ok(result.is_ok())
    }

    async fn get_namespaced_key(&self, key: &str) -> String {
        let namespace = self.namespace.read().unwrap();
        if namespace.is_empty() {
            key.to_string()
        } else {
            format!("{}:{}", namespace, key)
        }
    }
}
