pub mod redis;
use redis::RedisCache;

pub struct CacheManager {
    redis_cache: RedisCache,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            redis_cache: RedisCache::new(),
        }
    }

    pub fn with_memory_limit(memory_limit_mb: u64) -> Self {
        Self {
            redis_cache: RedisCache::with_memory_limit(memory_limit_mb),
        }
    }

    pub async fn set<T: serde::Serialize>(&self, key: &str, value: &T, ttl: std::time::Duration) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.redis_cache.set(key, value, ttl).await
    }

    pub async fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, Box<dyn std::error::Error + Send + Sync>> {
        self.redis_cache.get(key).await
    }

    pub async fn remove(&self, key: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.redis_cache.remove(key).await
    }

    pub async fn clear(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.redis_cache.clear().await
    }

    pub async fn get_memory_usage_mb(&self) -> u64 {
        self.redis_cache.get_memory_usage_mb().await
    }

    pub async fn get_entry_count(&self) -> usize {
        self.redis_cache.get_entry_count().await
    }

    pub fn get_memory_limit_mb(&self) -> u64 {
        self.redis_cache.get_memory_limit_mb()
    }
} 