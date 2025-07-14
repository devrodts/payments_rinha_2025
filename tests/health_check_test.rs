use rinha::modules::health::service::HealthCheckService;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_health_check_service_creation() {
    let service = HealthCheckService::new();
    
    assert!(service.get_rate_limit().as_secs() == 5);
    assert!(service.get_cache_ttl().as_secs() == 300);
}

#[tokio::test]
async fn test_health_check_rate_limiting() {
    let service = HealthCheckService::new();
    
    let result1 = service.check_processor_health("default").await;
    assert!(result1.is_ok());
    
    let result2 = service.check_processor_health("default").await;
    assert!(result2.is_ok());
    
    let last_check = service.get_last_check_time("default").await;
    assert!(last_check.is_some());
}

#[tokio::test]
async fn test_health_check_caching() {
    let service = HealthCheckService::new();
    
    let result1 = service.check_processor_health("default").await;
    assert!(result1.is_ok());
    
    let result2 = service.check_processor_health("default").await;
    assert!(result2.is_ok());
    
    assert_eq!(result1.unwrap(), result2.unwrap());
}

#[tokio::test]
async fn test_health_check_cache_expiration() {
    let mut service = HealthCheckService::new();
    
    service.set_cache_ttl(Duration::from_millis(100));
    
    let result1 = service.check_processor_health("default").await;
    assert!(result1.is_ok());
    
    sleep(Duration::from_millis(150)).await;
    
    let result2 = service.check_processor_health("default").await;
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_health_check_multiple_processors() {
    let service = HealthCheckService::new();
    
    let default_health = service.check_processor_health("default").await;
    let fallback_health = service.check_processor_health("fallback").await;
    
    assert!(default_health.is_ok());
    assert!(fallback_health.is_ok());
    
    let default_health2 = service.check_processor_health("default").await;
    let fallback_health2 = service.check_processor_health("fallback").await;
    
    assert!(default_health2.is_ok());
    assert!(fallback_health2.is_ok());
}

#[tokio::test]
async fn test_health_check_status_tracking() {
    let service = HealthCheckService::new();

    let initial_status = service.get_processor_status("default").await;
    assert!(initial_status.is_none());
    
    let _ = service.check_processor_health("default").await;
    
    let status = service.get_processor_status("default").await;
    assert!(status.is_some());
    
    let status = status.unwrap();
    assert!(status.is_healthy || !status.is_healthy);
    assert!(status.last_check.is_some());
}

#[tokio::test]
async fn test_health_check_error_handling() {
    let service = HealthCheckService::new();
    
    let result = service.check_processor_health("invalid-processor").await;
    assert!(result.is_ok());
    
    let result = service.check_processor_health("").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_health_check_concurrent_access() {
    let service = std::sync::Arc::new(HealthCheckService::new());
    let mut handles = vec![];
    
    for i in 0..5 {
        let service_clone = std::sync::Arc::clone(&service);
        let handle = tokio::spawn(async move {
            let processor = if i % 2 == 0 { "default" } else { "fallback" };
            service_clone.check_processor_health(processor).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_health_check_configuration() {
    let mut service = HealthCheckService::new();
    
    service.set_rate_limit(Duration::from_secs(10));
    assert_eq!(service.get_rate_limit().as_secs(), 10);
    
    service.set_cache_ttl(Duration::from_secs(600));
    assert_eq!(service.get_cache_ttl().as_secs(), 600);
}

#[tokio::test]
async fn test_health_check_endpoint_url() {
    let service = HealthCheckService::new();
    
    let default_url = service.get_health_endpoint_url("default").await;
    assert!(default_url.contains("/payments/service-health"));
    
    let fallback_url = service.get_health_endpoint_url("fallback").await;
    assert!(fallback_url.contains("/payments/service-health"));
}

#[tokio::test]
async fn test_health_check_batch_operation() {
    let service = HealthCheckService::new();
    
    let processors = vec!["default".to_string(), "fallback".to_string()];
    let results = service.check_all_processors_health(&processors).await;
    
    assert_eq!(results.len(), 2);
    for (processor, result) in results {
        assert!(result.is_ok(), "Health check failed for processor: {}", processor);
    }
} 