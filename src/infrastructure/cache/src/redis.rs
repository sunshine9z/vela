use std::sync::{Arc, RwLock};

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use commonx::error::AppError;

#[derive(Debug)]
pub struct RedisCache {
    pool: Pool<RedisConnectionManager>,
    namespace: Arc<RwLock<String>>,
}

impl RedisCache {}
