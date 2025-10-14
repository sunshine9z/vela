pub mod redis;

use std::sync::Arc;

use commonx::error::AppError;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{cache::redis::RedisCache, config::APP_CONFIG, web_info};

static MODULE_NAME: &str = "[cache]";

static DEFAULT_NAMESPACE: &str = "yela";

static GLOBAL_CACHE: OnceCell<Arc<Cache>> = OnceCell::new();

pub struct CacheManager;

impl CacheManager {
    pub async fn init() -> Result<(), AppError> {
        let cache = Cache::init().await?;
        GLOBAL_CACHE
            .set(Arc::new(cache))
            .map_err(|e| AppError::CacheInitError(format!("初始化全局缓存失败: {:?}", e)))?;
        Ok(())
    }
    pub fn instance() -> Arc<Cache> {
        GLOBAL_CACHE
            .get()
            .cloned()
            .expect(
            "Cache not initialized. This should not happen if CacheManager::init() was called during application startup."
        )
    }
}

#[derive(Debug)]

pub enum Cache {
    Redis(RedisCache),
}

impl Cache {
    pub async fn init() -> Result<Self, AppError> {
        let config = APP_CONFIG.cache.clone();
        web_info!("{MODULE_NAME}: 初始化缓存, {:?}", config);
        let namespace = config
            .namespace
            .unwrap_or_else(|| DEFAULT_NAMESPACE.to_string());

        match config.cache_type.as_str() {
            "redis" => {
                if config.url.is_none() {
                    return Err(AppError::CacheInitError(format!(
                        "redis缓存类型, 但未配置url"
                    )));
                }
                let redis = RedisCache::new(&config.url.unwrap(), namespace.clone()).await?;
                web_info!("{MODULE_NAME}: 初始化redis缓存, namespace: {namespace} ... [ok]");
                Ok(Self::Redis(redis))
            }
            _ => Err(AppError::CacheInitError(format!(
                "未知的缓存类型: {}",
                config.cache_type
            ))),
        }
    }

    pub async fn set_value_ex<T>(&self, k: &str, value: &T, t: i32) -> Result<bool, AppError>
    where
        T: Serialize + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.set_value_ex(k, value, t).await,
        }
    }

    pub async fn get_oneuse_value<T>(&self, k: &str) -> Result<T, AppError>
    where
        T: Serialize + for<'de> Deserialize<'de> + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.get_oneuse_value(k).await,
        }
    }
}
