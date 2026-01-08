pub mod memory;
pub mod redis;

use std::sync::Arc;

use commonx::error::AppError;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{cache::memory::MemoryCache, cache::redis::RedisCache, config::APP_CONFIG, web_info};

static MODULE_NAME: &str = "[cache]";

static DEFAULT_NAMESPACE: &str = "vela";

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
    Memory(MemoryCache),
}

impl Cache {
    pub async fn init() -> Result<Self, AppError> {
        let config = &APP_CONFIG.cache;
        web_info!("{MODULE_NAME}: 初始化缓存, {:?}", config);
        let default_namespace = DEFAULT_NAMESPACE.to_string();
        let namespace = config
            .namespace
            .as_ref()
            .unwrap_or_else(|| &default_namespace);

        match config.cache_type.as_str() {
            "redis" => {
                if config.url.is_none() {
                    return Err(AppError::CacheInitError(format!(
                        "redis缓存类型, 但未配置url"
                    )));
                }
                let redis = RedisCache::new(&config.url.as_ref().unwrap(), namespace).await?;
                web_info!("{MODULE_NAME}: 初始化redis缓存, namespace: {namespace} ... [ok]");
                Ok(Self::Redis(redis))
            }
            "memory" => Ok(Self::Memory(MemoryCache::new(namespace))),
            _ => Err(AppError::CacheInitError(format!(
                "未知的缓存类型: {}",
                config.cache_type
            ))),
        }
    }

    pub async fn set_value_ex<T>(&self, k: &str, value: &T, ttl: i32) -> Result<bool, AppError>
    where
        T: Serialize + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.set_value_ex(k, value, ttl).await,
            Cache::Memory(cache) => cache.set_value_ex(k, value, ttl).await,
        }
    }

    pub async fn get_oneuse_value<T>(&self, k: &str) -> Result<T, AppError>
    where
        T: Serialize + for<'de> Deserialize<'de> + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.get_oneuse_value(k).await,
            Cache::Memory(cache) => cache.get_oneuse_value(k).await,
        }
    }

    pub async fn brpop(
        &self,
        keys: &Vec<String>,
        timeout: usize,
    ) -> Result<Option<(String, String)>, AppError> {
        match self {
            Cache::Redis(cache) => cache.brpop(keys, timeout).await,
            Cache::Memory(cache) => cache.brpop(keys, timeout).await,
        }
    }

    pub async fn set_nx_ex<V>(&self, key: &str, value: V, ttl: usize) -> Result<bool, AppError>
    where
        V: ToString + Send + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.set_nx_ex::<V>(key, value, ttl).await,
            Cache::Memory(cache) => cache.set_nx_ex::<V>(key, value, ttl).await,
        }
    }

    pub async fn sadd(&self, key: &str, values: &[&str]) -> Result<usize, AppError> {
        match self {
            Cache::Redis(cache) => cache.sadd(key, values).await,
            Cache::Memory(cache) => cache.sadd(key, values).await,
        }
    }
    pub async fn lpush<V>(&self, key: &str, value: V) -> Result<usize, AppError>
    where
        V: ToString + Send + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.lpush(key, value).await,
            Cache::Memory(cache) => cache.lpush(key, value).await,
        }
    }
}
