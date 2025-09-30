use std::sync::{Arc, RwLock};

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use commonx::error::AppError;

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
}
