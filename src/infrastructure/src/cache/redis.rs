use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use bb8::Pool;
use bb8_redis::{RedisConnectionManager, bb8, redis};
use commonx::{error::AppError, web_error};
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};

use commonx::web_info;

use crate::cache::CacheTrait;

#[derive(Debug)]
pub struct RedisCache {
    pool: Pool<RedisConnectionManager>,
    namespace: Arc<RwLock<String>>,
}

impl RedisCache {
    pub async fn new(redis_url: &str, namespace: &String) -> Result<Self, AppError> {
        web_info!("初始化 RedisCache ns:{} url:{}", namespace, redis_url);
        let manager = RedisConnectionManager::new(redis_url).map_err(|e| {
            web_error!(
                "初始化 RedisCache 失败 ns:{} url:{} err:{}",
                namespace,
                redis_url,
                e
            );
            AppError::RedisError(e.to_string())
        })?;
        let pool = Pool::builder()
            .max_size(10)
            .min_idle(Some(2))
            .connection_timeout(Duration::from_secs(20))
            .build(manager)
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        Ok(Self {
            pool,
            namespace: Arc::new(RwLock::new(namespace.to_string())),
        })
    }

    fn get_namespaced_key(&self, key: &str) -> String {
        let namespace = self.namespace.read().unwrap();
        if namespace.is_empty() {
            key.to_string()
        } else {
            format!("{}:{}", namespace, key)
        }
    }

    fn get_namespaced_keys(&self, keys: &Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let namespace = self.namespace.read().unwrap();
        keys.iter().for_each(|k| {
            if namespace.is_empty() {
                result.push(k.to_string());
            } else {
                result.push(format!("{}:{}", namespace, k));
            }
        });
        result
    }
}
#[async_trait::async_trait]
impl CacheTrait for RedisCache {
    async fn set_value_ex<T>(&self, k: &str, value: &T, t: i32) -> Result<bool, AppError>
    where
        T: Serialize + Sync,
    {
        let value_str = serde_json::to_string(value)?;
        let ret = self.set_string_ex(k, &value_str, t).await;
        ret
    }

    async fn get_oneuse_value<T>(&self, k: &str) -> Result<T, AppError>
    where
        T: Serialize + for<'de> Deserialize<'de> + Sync + Send,
    {
        let result = self.get_value::<T>(k).await;
        if result.is_ok() {
            let _ = self.remove(k).await;
        }
        result
    }

    async fn set_string_ex(&self, k: &str, v: &str, t: i32) -> Result<bool, AppError> {
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

    async fn get_value<T>(&self, k: &str) -> Result<T, AppError>
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

    async fn get_string(&self, k: &str) -> Result<String, AppError> {
        let key = self.get_namespaced_key(k);
        let mut conn = self.pool.get().await?;
        let result: Option<String> = conn.get(&key).await?;
        result
            .ok_or_else(|| AppError::CacheNotFoundError(format!("数据不存在: {}", key)))
            .into()
    }

    async fn remove(&self, k: &str) -> Result<usize, AppError> {
        let key = self.get_namespaced_key(k);
        let mut conn = self.pool.get().await?;
        let result: usize = conn.del(&key).await?;
        Ok(result)
    }

    async fn brpop(
        &self,
        keys: &Vec<String>,
        timeout: usize,
    ) -> Result<Option<(String, String)>, AppError> {
        let namespaced_keys = self.get_namespaced_keys(keys);
        let mut conn = self.pool.get().await?;
        let result = conn.brpop(&namespaced_keys, timeout as f64).await?;
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

    async fn set_nx_ex<V>(&self, key: &str, value: V, ttl: usize) -> Result<bool, AppError>
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
    async fn sadd(&self, key: &str, members: &[&str]) -> Result<usize, AppError> {
        let namespaced_key = self.get_namespaced_key(key);
        let mut conn = self.pool.get().await?;
        let result: usize = conn.sadd(&namespaced_key, members).await?;
        Ok(result)
    }

    async fn lpush<V>(&self, key: &str, value: V) -> Result<usize, AppError>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key);
        let mut conn = self.pool.get().await?;
        let result: usize = conn.lpush(&namespaced_key, value.to_string()).await?;
        Ok(result)
    }

    async fn zrangebyscore_limit(
        &self,
        key: &str,
        min_score: f64,
        max_score: f64,
        offset: isize,
        count: isize,
    ) -> Result<Vec<String>, AppError> {
        let namespaced_key = self.get_namespaced_key(key);
        let mut conn = self.pool.get().await?;
        let result: Vec<String> = conn
            .zrangebyscore_limit(&namespaced_key, min_score, max_score, offset, count)
            .await?;
        Ok(result)
    }
    async fn zrem<V>(&self, key: &str, value: V) -> Result<bool, AppError>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key);
        let mut conn = self.pool.get().await?;
        let result: i64 = conn.zrem(&namespaced_key, value.to_string()).await?;
        Ok(result > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_brpop_with_data() {
        // 初始化RedisCache，使用默认的本地Redis连接
        let redis_url = "redis://:ymDT@rediS2pwD@127.0.0.1:6666";
        let namespace = "test".to_string();
        let cache = RedisCache::new(redis_url, &namespace).await.unwrap();

        // 测试前清理相关键
        let test_queue = "test:queue";
        let _ = cache.remove(test_queue).await;

        // 向队列中添加测试数据
        let test_value = "test_value";
        cache.lpush(test_queue, test_value).await.unwrap();

        // 测试brpop方法
        let keys = vec![test_queue.to_string()];
        println!(">>>>>>>>>>>>>>1");
        let result = cache.brpop(&keys, 5).await;
        println!(">>>>>>>>>>>>>>{:?}", result);

        let result = cache.brpop(&keys, 5).await;
        println!(">>>>>>>>>>>>>>{:?}", result);
        // 验证结果
        // assert!(result.is_some());
        // let (queue, value) = result.unwrap();
        // assert_eq!(queue, test_queue);
        // assert_eq!(value, test_value);

        // 测试后清理
        let _ = cache.remove(test_queue).await;
    }
}
