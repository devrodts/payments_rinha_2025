use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub last_check: Option<u64>,
    pub response_time_ms: Option<u64>,
    pub error_message: Option<String>,
}

#[allow(dead_code)]
pub struct HealthCheckService {
    client: Client,
    rate_limit: Duration,
    cache_ttl: Duration,
    last_checks: Arc<RwLock<HashMap<String, Instant>>>,
    health_cache: Arc<RwLock<HashMap<String, (HealthStatus, Instant)>>>,
    processor_urls: Arc<RwLock<HashMap<String, String>>>,
}

impl HealthCheckService {
    pub fn new() -> Self {
        let mut processor_urls = HashMap::new();
        let default_url = std::env::var("PAYMENT_PROCESSOR_DEFAULT_URL")
            .unwrap_or_else(|_| "http://payment-processor-default:8080".to_string());
        let fallback_url = std::env::var("PAYMENT_PROCESSOR_FALLBACK_URL")
            .unwrap_or_else(|_| "http://payment-processor-fallback:8080".to_string());
        
        processor_urls.insert("default".to_string(), default_url);
        processor_urls.insert("fallback".to_string(), fallback_url);
        
        Self {
            client: Client::new(),
            rate_limit: Duration::from_secs(5),
            cache_ttl: Duration::from_secs(300),
            last_checks: Arc::new(RwLock::new(HashMap::new())),
            health_cache: Arc::new(RwLock::new(HashMap::new())),
            processor_urls: Arc::new(RwLock::new(processor_urls)),
        }
    }

    pub fn get_rate_limit(&self) -> Duration {
        self.rate_limit
    }

    pub fn get_cache_ttl(&self) -> Duration {
        self.cache_ttl
    }

    pub fn set_rate_limit(&mut self, rate_limit: Duration) {
        self.rate_limit = rate_limit;
    }

    pub fn set_cache_ttl(&mut self, cache_ttl: Duration) {
        self.cache_ttl = cache_ttl;
    }

    pub async fn check_processor_health(&self, processor_name: &str) -> Result<HealthStatus, Box<dyn std::error::Error + Send + Sync>> {
        // Check if we have a cached result that's still valid
        if let Some(cached_status) = self.get_cached_health_status(processor_name).await {
            return Ok(cached_status);
        }

        // Check rate limiting
        if !self.can_perform_health_check(processor_name).await {
            // Return last known status or default unhealthy status
            return self.get_last_known_status(processor_name).await;
        }

        // Perform actual health check
        let health_status = self.perform_health_check(processor_name).await?;
        
        // Cache the result
        self.cache_health_status(processor_name, &health_status).await;
        
        // Update last check time
        self.update_last_check_time(processor_name).await;
        
        Ok(health_status)
    }

    pub async fn get_processor_status(&self, processor_name: &str) -> Option<HealthStatus> {
        self.get_cached_health_status(processor_name).await
    }

    pub async fn get_last_check_time(&self, processor_name: &str) -> Option<u64> {
        let last_checks = self.last_checks.read().await;
        last_checks.get(processor_name).map(|_instant| {
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        })
    }

    pub async fn get_health_endpoint_url(&self, processor_name: &str) -> String {
        let processor_urls = self.processor_urls.read().await;
        let base_url = processor_urls.get(processor_name)
            .unwrap_or(&format!("http://{}-processor:8080", processor_name))
            .clone();
        format!("{}/payments/service-health", base_url)
    }

    pub async fn check_all_processors_health(&self, processor_names: &[String]) -> HashMap<String, Result<HealthStatus, Box<dyn std::error::Error + Send + Sync>>> {
        let mut results = HashMap::new();
        
        for processor_name in processor_names {
            let result = self.check_processor_health(processor_name).await;
            results.insert(processor_name.clone(), result);
        }
        
        results
    }

    async fn can_perform_health_check(&self, processor_name: &str) -> bool {
        let last_checks = self.last_checks.read().await;
        if let Some(last_check) = last_checks.get(processor_name) {
            return last_check.elapsed() >= self.rate_limit;
        }
        true
    }

    async fn get_cached_health_status(&self, processor_name: &str) -> Option<HealthStatus> {
        let health_cache = self.health_cache.read().await;
        if let Some((status, timestamp)) = health_cache.get(processor_name) {
            if timestamp.elapsed() < self.cache_ttl {
                return Some(status.clone());
            }
        }
        None
    }

    async fn get_last_known_status(&self, processor_name: &str) -> Result<HealthStatus, Box<dyn std::error::Error + Send + Sync>> {
        let health_cache = self.health_cache.read().await;
        if let Some((status, _)) = health_cache.get(processor_name) {
            return Ok(status.clone());
        }
        
        // Return default unhealthy status if no cached data
        Ok(HealthStatus {
            is_healthy: false,
            last_check: None,
            response_time_ms: None,
            error_message: Some("No health data available".to_string()),
        })
    }

    async fn perform_health_check(&self, processor_name: &str) -> Result<HealthStatus, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.get_health_endpoint_url(processor_name).await;
        let start_time = Instant::now();
        
        match self.client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                let is_healthy = response.status().is_success();
                
                Ok(HealthStatus {
                    is_healthy,
                    last_check: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
                    response_time_ms: Some(response_time),
                    error_message: if is_healthy { None } else { Some(format!("HTTP {}", response.status())) },
                })
            }
            Err(e) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                Ok(HealthStatus {
                    is_healthy: false,
                    last_check: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
                    response_time_ms: Some(response_time),
                    error_message: Some(e.to_string()),
                })
            }
        }
    }

    async fn cache_health_status(&self, processor_name: &str, status: &HealthStatus) {
        let mut health_cache = self.health_cache.write().await;
        health_cache.insert(processor_name.to_string(), (status.clone(), Instant::now()));
    }

    async fn update_last_check_time(&self, processor_name: &str) {
        let mut last_checks = self.last_checks.write().await;
        last_checks.insert(processor_name.to_string(), Instant::now());
    }
} 