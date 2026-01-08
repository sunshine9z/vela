use std::sync::{Arc, RwLock};

use bb8::Pool;
use bb8_redis::{
    RedisConnectionManager,
    redis::{self, AsyncCommands, RedisResult},
};
use commonx::error::AppError;
use serde::{Deserialize, Serialize};

use crate::web_info;

#[derive(Debug)]
pub struct RedisCache {
    pool: Pool<RedisConnectionManager>,
    namespace: Arc<RwLock<String>>,
}

impl RedisCache {
    pub async fn new(redis_url: &str, namespace: &String) -> Result<Self, AppError> {
        let manager = RedisConnectionManager::new(redis_url)
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        let pool = Pool::builder()
            .build(manager)
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        Ok(Self {
            pool,
            namespace: Arc::new(RwLock::new(namespace.to_string())),
        })
    }

    pub async fn set_value_ex<T>(&self, k: &str, value: &T, t: i32) -> Result<bool, AppError>
    where
        T: Serialize + Sync,
    {
        let value_str = serde_json::to_string(value)?;
        let ret = self.set_string_ex(k, &value_str, t).await;
        ret
    }

    pub async fn get_oneuse_value<T>(&self, k: &str) -> Result<T, AppError>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let result = self.get_value(k).await;
        if result.is_ok() {
            let _ = self.remove(k).await;
        }
        result
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, t: i32) -> Result<bool, AppError> {
        let key = self.get_namespaced_key(k);
        let mut conn = self.pool.get().await?;
        let result: RedisResult<()> = conn.set_ex(&key, v, t as u64).await;
        web_info!(
            "设置缓存 ns:{} key:[{}] val:{} (expire: {}s) -> {}",
            self.namespace.read().unwrap(),
            k,
            v,
            t,
            result.is_ok()
        );
        Ok(result.is_ok())
    }

    fn get_namespaced_key(&self, key: &str) -> String {
        let namespace = self.namespace.read().unwrap();
        if namespace.is_empty() {
            key.to_string()
        } else {
            format!("{}:{}", namespace, key)
        }
    }

    pub async fn get_value<T>(&self, k: &str) -> Result<T, AppError>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let value_str = self.get_string(k).await?;
        web_info!(
            "获取缓存:{}:{} -> {}",
            self.namespace.read().unwrap(),
            k,
            value_str
        );
        Ok(serde_json::from_str(&value_str)?)
    }

    pub async fn get_string(&self, k: &str) -> Result<String, AppError> {
        let key = self.get_namespaced_key(k);
        let mut conn = self.pool.get().await?;
        let result: Option<String> = conn.get(&key).await?;
        result
            .ok_or_else(|| AppError::CacheNotFoundError(format!("数据不存在: {}", key)))
            .into()
    }

    pub async fn remove(&self, k: &str) -> Result<usize, AppError> {
        let key = self.get_namespaced_key(k);
        let mut conn = self.pool.get().await?;
        let result: usize = conn.del(&key).await?;
        Ok(result)
    }

    pub async fn brpop(
        &self,
        keys: &Vec<String>,
        timeout: usize,
    ) -> Result<Option<(String, String)>, AppError> {
        let namespaced_keys = self.get_namespaced_keys(keys);
        let mut conn = self.pool.get().await?;
        let result: Option<(String, String)> = conn.brpop(&namespaced_keys, timeout as f64).await?;
        if let Some((key, value)) = result {
            let original_key = keys
                .iter()
                .zip(namespaced_keys.iter())
                .find(|(_, namespaced)| *namespaced == &key)
                .map(|(original, _)| (*original).clone())
                .unwrap_or(key);
            Ok(Some((original_key, value)))
        } else {
            Ok(None)
        }
    }

    pub fn get_namespaced_keys(&self, keys: &Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        keys.iter().for_each(|k| {
            result.push(k.to_string());
        });
        result
    }

    pub async fn set_nx_ex<V>(&self, key: &str, value: V, ttl: usize) -> Result<bool, AppError>
    where
        V: ToString + Sync + Send,
    {
        let namespace_key = self.get_namespaced_key(key);
        let mut conn = self.pool.get().await?;
        let result = redis::cmd("SET")
            .arg(namespace_key)
            .arg(value.to_string())
            .arg("EX")
            .arg(ttl.to_string())
            .arg("NX")
            .query_async::<Option<String>>(&mut *conn)
            .await?;
        Ok(result.is_some())
    }
    pub async fn sadd(&self, key: &str, members: &[&str]) -> Result<usize, AppError> {
        let namespaced_key = self.get_namespaced_key(key);
        let mut conn = self.pool.get().await?;
        let result: usize = conn.sadd(&namespaced_key, members).await?;
        Ok(result)
    }

    pub async fn lpush<V>(&self, key: &str, value: V) -> Result<usize, AppError>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key);
        let mut conn = self.pool.get().await?;
        let result: usize = conn.lpush(&namespaced_key, value.to_string()).await?;
        Ok(result)
    }
}
