use commonx::error::AppError;
use configx::{APP_CONFIG, config::Config};

use crate::redis::RedisCache;

pub mod redis;

pub enum Cache {
    Redis(RedisCache),
}

impl Cache {
    // pub async fn new() -> Result<Self, AppError> {
    //     let config = APP_CONFIG.cache.clone();
    //     let namespace = config.namespace.unwrap_or_else(|| "ysweb".to_string());
    // }
}
