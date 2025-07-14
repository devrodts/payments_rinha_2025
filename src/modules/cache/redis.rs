use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub value: T,
    pub timestamp: u64,
    pub ttl: u64,
}

#[allow(dead_code)]
pub struct RedisCache {
    memory_limit_mb: u64,
    policy: CachePolicy,
    cache: Arc<RwLock<HashMap<String, CacheEntry<String>>>>,
    memory_usage: Arc<RwLock<u64>>,
}

#[allow(dead_code)]
pub enum CachePolicy {
    AllKeysLRU,
    VolatileLRU,
    AllKeysRandom,
    VolatileRandom,
    VolatileTTL,
}

impl RedisCache {
    pub fn new() -> Self {
        Self {
            memory_limit_mb: 50,
            policy: CachePolicy::AllKeysLRU,
            cache: Arc::new(RwLock::new(HashMap::new())),
            memory_usage: Arc::new(RwLock::new(0)),
        }
    }

    pub fn with_memory_limit(memory_limit_mb: u64) -> Self {
        Self {
            memory_limit_mb,
            policy: CachePolicy::AllKeysLRU,
            cache: Arc::new(RwLock::new(HashMap::new())),
            memory_usage: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let serialized_value = serde_json::to_string(value)?;
        let entry_size = serialized_value.len() as u64;
        
        // Check memory limit
        if !self.can_store_entry(entry_size).await {
            self.evict_entries().await;
        }
        
        let entry = CacheEntry {
            value: serialized_value,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl: ttl.as_secs(),
        };
        
        let mut cache = self.cache.write().await;
        let old_size = cache.get(key).map(|e| e.value.len() as u64).unwrap_or(0);
        cache.insert(key.to_string(), entry);
        
        // Update memory usage
        let mut memory_usage = self.memory_usage.write().await;
        *memory_usage = *memory_usage - old_size + entry_size;
        
        Ok(())
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, Box<dyn std::error::Error + Send + Sync>> {
        let cache = self.cache.read().await;
        
        if let Some(entry) = cache.get(key) {
            // Check if entry has expired
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if current_time - entry.timestamp > entry.ttl {
                // Entry has expired, remove it
                drop(cache);
                let _ = self.remove(key).await;
                return Ok(None);
            }
            
            let value: T = serde_json::from_str(&entry.value)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub async fn remove(&self, key: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut cache = self.cache.write().await;
        let mut memory_usage = self.memory_usage.write().await;
        
        if let Some(entry) = cache.remove(key) {
            *memory_usage -= entry.value.len() as u64;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn clear(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut cache = self.cache.write().await;
        let mut memory_usage = self.memory_usage.write().await;
        
        cache.clear();
        *memory_usage = 0;
        
        Ok(())
    }

    pub async fn get_memory_usage_mb(&self) -> u64 {
        let memory_usage = self.memory_usage.read().await;
        *memory_usage / (1024 * 1024)
    }

    pub async fn get_entry_count(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    pub fn get_memory_limit_mb(&self) -> u64 {
        self.memory_limit_mb
    }

    async fn can_store_entry(&self, entry_size: u64) -> bool {
        let memory_usage = self.memory_usage.read().await;
        let current_usage_mb = *memory_usage / (1024 * 1024);
        current_usage_mb + (entry_size / (1024 * 1024)) <= self.memory_limit_mb
    }

    async fn evict_entries(&self) {
        match self.policy {
            CachePolicy::AllKeysLRU => self.evict_lru_entries().await,
            CachePolicy::VolatileLRU => self.evict_volatile_lru_entries().await,
            CachePolicy::AllKeysRandom => self.evict_random_entries().await,
            CachePolicy::VolatileRandom => self.evict_volatile_random_entries().await,
            CachePolicy::VolatileTTL => self.evict_volatile_ttl_entries().await,
        }
    }

    async fn evict_lru_entries(&self) {
        let mut cache = self.cache.write().await;
        let mut memory_usage = self.memory_usage.write().await;
        
        // Find the oldest entry
        let oldest_key = cache.iter()
            .min_by_key(|(_, entry)| entry.timestamp)
            .map(|(key, _)| key.clone());
        
        if let Some(key) = oldest_key {
            if let Some(entry) = cache.remove(&key) {
                *memory_usage -= entry.value.len() as u64;
            }
        }
    }

    async fn evict_volatile_lru_entries(&self) {
        // For volatile LRU, we would check if entries have TTL set
        // For simplicity, we'll use the same logic as AllKeysLRU
        self.evict_lru_entries().await;
    }

    async fn evict_random_entries(&self) {
        let mut cache = self.cache.write().await;
        let mut memory_usage = self.memory_usage.write().await;
        
        if let Some((key, _entry)) = cache.iter().next() {
            let key = key.clone();
            if let Some(entry) = cache.remove(&key) {
                *memory_usage -= entry.value.len() as u64;
            }
        }
    }

    async fn evict_volatile_random_entries(&self) {
        // For volatile random, we would check if entries have TTL set
        // For simplicity, we'll use the same logic as AllKeysRandom
        self.evict_random_entries().await;
    }

    async fn evict_volatile_ttl_entries(&self) {
        let mut cache = self.cache.write().await;
        let mut memory_usage = self.memory_usage.write().await;
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Find the entry with the shortest TTL remaining
        let shortest_ttl_key = cache.iter()
            .filter(|(_, entry)| current_time - entry.timestamp < entry.ttl)
            .min_by_key(|(_, entry)| entry.ttl - (current_time - entry.timestamp))
            .map(|(key, _)| key.clone());
        
        if let Some(key) = shortest_ttl_key {
            if let Some(entry) = cache.remove(&key) {
                *memory_usage -= entry.value.len() as u64;
            }
        }
    }
} 