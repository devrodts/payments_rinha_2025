pub mod service;
use service::HealthCheckService;

pub struct HealthManager {
    health_service: HealthCheckService,
}

impl HealthManager {
    pub fn new() -> Self {
        Self {
            health_service: HealthCheckService::new(),
        }
    }

    pub async fn check_processor_health(&self, processor_name: &str) -> Result<service::HealthStatus, Box<dyn std::error::Error + Send + Sync>> {
        self.health_service.check_processor_health(processor_name).await
    }

    pub async fn get_processor_status(&self, processor_name: &str) -> Option<service::HealthStatus> {
        self.health_service.get_processor_status(processor_name).await
    }

    pub async fn get_health_endpoint_url(&self, processor_name: &str) -> String {
        self.health_service.get_health_endpoint_url(processor_name).await
    }

    pub async fn check_all_processors_health(&self, processor_names: &[String]) -> std::collections::HashMap<String, Result<service::HealthStatus, Box<dyn std::error::Error + Send + Sync>>> {
        self.health_service.check_all_processors_health(processor_names).await
    }

    // Expose configuration methods
    pub fn get_rate_limit(&self) -> std::time::Duration {
        self.health_service.get_rate_limit()
    }

    pub fn get_cache_ttl(&self) -> std::time::Duration {
        self.health_service.get_cache_ttl()
    }
} 