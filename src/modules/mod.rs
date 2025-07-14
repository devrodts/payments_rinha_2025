pub mod config;
pub mod models;
pub mod payment;
pub mod processors;
pub mod health;
pub mod cache;

use processors::PaymentProcessor;
use health::HealthManager;
use cache::CacheManager;

pub struct ApplicationServices {
    pub payment_processor: PaymentProcessor,
    pub health_manager: HealthManager,
    pub cache_manager: CacheManager,
}

impl ApplicationServices {
    pub fn new() -> Self {
        Self {
            payment_processor: PaymentProcessor::new(),
            health_manager: HealthManager::new(),
            cache_manager: CacheManager::new(),
        }
    }

    pub fn with_cache_memory_limit(memory_limit_mb: u64) -> Self {
        Self {
            payment_processor: PaymentProcessor::new(),
            health_manager: HealthManager::new(),
            cache_manager: CacheManager::with_memory_limit(memory_limit_mb),
        }
    }
} 