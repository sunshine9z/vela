use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use commonx::error::AppError;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::time::interval;

use crate::cache::CacheTrait;

#[derive(Debug, Clone)]
struct MemoryCacheItem {
    value: String,
    expires_at: Option<i64>,
    #[allow(dead_code)]
    cache_key: i64,
}

impl MemoryCacheItem {
    fn new(value: String, ttl_seconds: Option<usize>) -> Self {
        let now = chrono::Local::now().timestamp();
        let expires_at = ttl_seconds.map(|ttl| now + ttl as i64);
        Self {
            value,
            expires_at,
            cache_key: now,
        }
    }

    fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            expires_at <= chrono::Local::now().timestamp()
        } else {
            false
        }
    }

    fn ttl(&self) -> i64 {
        if let Some(expires_at) = self.expires_at {
            if expires_at - chrono::Local::now().timestamp() > 0 {
                expires_at - chrono::Local::now().timestamp()
            } else {
                -1 // 已过期
            }
        } else {
            -1 // 永不过期
        }
    }
}

#[derive(Debug)]
pub struct MemoryCache {
    storage: Arc<DashMap<String, MemoryCacheItem>>,
    namespace: Arc<RwLock<String>>,
    lists: Arc<DashMap<String, Vec<String>>>,
    sets: Arc<DashMap<String, DashMap<String, bool>>>,
    sorted_sets: Arc<DashMap<String, Vec<(String, f64)>>>,
}

impl MemoryCache {
    pub fn new(namespace: &str) -> Self {
        let cache = MemoryCache {
            storage: Arc::new(DashMap::new()),
            namespace: Arc::new(RwLock::new(namespace.to_string())),
            lists: Arc::new(DashMap::new()),
            sets: Arc::new(DashMap::new()),
            sorted_sets: Arc::new(DashMap::new()),
        };
        cache.start_cleanup_task();
        cache
    }

    fn start_cleanup_task(&self) {
        let storage = self.storage.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                storage.retain(|_, item| !item.is_expired());
            }
        });
    }

    async fn get_namespaced_key(&self, key: &str) -> Result<String, AppError> {
        let namespace = self.namespace.read().unwrap_or_else(|e| e.into_inner());
        if namespace.is_empty() {
            Ok(key.to_string())
        } else {
            Ok(format!("{}:{}", namespace, key))
        }
    }
}

#[async_trait::async_trait]
impl CacheTrait for MemoryCache {
    async fn set_value_ex<T>(&self, k: &str, value: &T, ttl: i32) -> Result<bool, AppError>
    where
        T: Serialize + Sync,
    {
        let value_str = serde_json::to_string(value)?;
        self.set_string_ex(k, &value_str, ttl).await
    }

    async fn set_string_ex(&self, k: &str, value: &str, ttl: i32) -> Result<bool, AppError> {
        let item = MemoryCacheItem::new(value.to_string(), Some(ttl as usize));
        self.storage.insert(k.to_string(), item);
        Ok(true)
    }

    async fn get_oneuse_value<T>(&self, k: &str) -> Result<T, AppError>
    where
        T: Serialize + for<'de> Deserialize<'de> + Sync + Send,
    {
        let result = self.get_value(k).await;
        if result.is_ok() {
            let _ = self.remove(k).await;
        }
        result
    }

    async fn get_value<T>(&self, k: &str) -> Result<T, AppError>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let value_str = self.get_string(k).await?;
        Ok(serde_json::from_str(&value_str)?)
    }

    async fn get_string(&self, k: &str) -> Result<String, AppError> {
        let key = self.get_namespaced_key(k).await?;
        if let Some(item) = self.storage.get(&key) {
            if item.is_expired() {
                self.storage.remove(&key);
                Err("Key not found or expired".into())
            } else {
                Ok(item.value.clone())
            }
        } else {
            Err("Key not found or expired".into())
        }
    }

    async fn remove(&self, k: &str) -> Result<usize, AppError> {
        let key = self.get_namespaced_key(k).await?;
        let mut removed = 0;
        if self.storage.remove(&key).is_some() {
            removed += 1;
        }
        if self.lists.remove(&key).is_some() {
            removed += 1;
        }
        if self.sets.remove(&key).is_some() {
            removed += 1;
        }
        if self.sorted_sets.remove(&key).is_some() {
            removed += 1;
        }
        Ok(removed)
    }

    async fn brpop(
        &self,
        keys: &Vec<String>,
        _timeout: usize,
    ) -> Result<Option<(String, String)>, AppError> {
        for key in keys {
            let namespaced_key = self.get_namespaced_key(&key).await?;
            if let Some(mut list) = self.lists.get_mut(&namespaced_key) {
                if let Some(value) = list.pop() {
                    return Ok(Some((namespaced_key, value)));
                }
            }
        }
        Ok(None)
    }

    async fn set_nx_ex<V>(
        &self,
        key: &str,
        value: V,
        ttl_in_seconds: usize,
    ) -> Result<bool, AppError>
    where
        V: ToString + Send + Sync,
    {
        let namespace_key = self.get_namespaced_key(&key).await?;

        if self.storage.contains_key(&namespace_key)
            || self.lists.contains_key(&namespace_key)
            || self.sets.contains_key(&namespace_key)
            || self.sorted_sets.contains_key(&namespace_key)
        {
            return Ok(false);
        }
        let item = MemoryCacheItem::new(value.to_string(), Some(ttl_in_seconds));
        self.storage.insert(namespace_key, item);
        Ok(true)
    }

    async fn sadd(&self, key: &str, members: &[&str]) -> Result<usize, AppError> {
        let namespace_key = self.get_namespaced_key(&key).await?;
        let set = self.sets.entry(namespace_key).or_default();
        let mut added = 0;
        for member in members {
            if set.insert(member.to_string(), true).is_none() {
                added += 1;
            }
        }
        Ok(added)
    }

    async fn lpush<V>(&self, key: &str, value: V) -> Result<usize, AppError>
    where
        V: ToString + Send + Sync,
    {
        let namespace_key = self.get_namespaced_key(&key).await?;
        let mut list = self.lists.entry(namespace_key).or_default();
        list.insert(0, value.to_string());
        Ok(list.len())
    }

    async fn zrangebyscore_limit(
        &self,
        key: &str,
        min_score: f64,
        max_score: f64,
        offset: isize,
        count: isize,
    ) -> Result<Vec<String>, AppError> {
        let namespace_key = self.get_namespaced_key(&key).await?;
        if let Some(sorted_set) = self.sorted_sets.get(&namespace_key) {
            // 1. 首先创建一个排序的副本
            let mut sorted_items: Vec<_> = sorted_set.iter().clone().collect();
            sorted_items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // 2. 然后按分数范围过滤
            let filtered_items: Vec<_> = sorted_items
                .into_iter()
                .filter(|item| item.1 >= min_score && item.1 <= max_score)
                .skip(offset as usize)
                .take(count as usize)
                .map(|item| item.0.clone())
                .collect();
            Ok(filtered_items)
        } else {
            Ok(vec![])
        }
    }

    async fn zrem<V>(&self, key: &str, value: V) -> Result<bool, AppError>
    where
        V: ToString + Send + Sync,
    {
        let namespace_key = self.get_namespaced_key(&key).await?;
        if let Some(mut sorted_set) = self.sorted_sets.get_mut(&namespace_key) {
            let value_str = value.to_string();
            if let Some(pos) = sorted_set.iter().position(|(v, _)| v == &value_str) {
                sorted_set.remove(pos);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl Clone for MemoryCache {
    fn clone(&self) -> Self {
        Self {
            storage: Arc::clone(&self.storage),
            namespace: Arc::clone(&self.namespace),
            lists: Arc::clone(&self.lists),
            sets: Arc::clone(&self.sets),
            sorted_sets: Arc::clone(&self.sorted_sets),
        }
    }
}
