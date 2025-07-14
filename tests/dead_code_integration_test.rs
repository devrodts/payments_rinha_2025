use rinha::modules::ApplicationServices;
use rinha::modules::processors::selector::ProcessorSelector;
use rinha::modules::health::service::HealthCheckService;
use rinha::modules::cache::redis::RedisCache;

#[tokio::test]
async fn test_t13_1_application_services_integration() {
    // Test that ApplicationServices can be created and used
    let services = ApplicationServices::new();
    
    // Test that we can access all services
    let processor_info = services.payment_processor.get_processor_info().await;
    assert!(!processor_info.is_empty());
    
    let rates = services.payment_processor.get_processor_rates().await;
    assert!(!rates.is_empty());
    
    let health_rate_limit = services.health_manager.get_rate_limit();
    assert_eq!(health_rate_limit.as_secs(), 5);
    
    let cache_memory_limit = services.cache_manager.get_memory_limit_mb();
    assert_eq!(cache_memory_limit, 50);
}

#[tokio::test]
async fn test_t13_1_processor_selector_integration() {
    // Test that ProcessorSelector can be created and used
    let selector = ProcessorSelector::new();
    
    // Test basic functionality
    let processors = selector.get_processors().await;
    assert!(!processors.is_empty());
    
    // Test that we can get processor rates
    let rates = selector.get_processor_rates().await;
    assert!(!rates.is_empty());
    
    // Test that we can get default processor
    let default_processor = selector.get_default_processor().await;
    assert!(default_processor.is_some());
}

#[tokio::test]
async fn test_t13_1_health_service_integration() {
    // Test that HealthCheckService can be created and used
    let health_service = HealthCheckService::new();
    
    // Test configuration methods
    let rate_limit = health_service.get_rate_limit();
    assert_eq!(rate_limit.as_secs(), 5);
    
    let cache_ttl = health_service.get_cache_ttl();
    assert_eq!(cache_ttl.as_secs(), 300);
    
    // Test that we can get health endpoint URLs
    let default_url = health_service.get_health_endpoint_url("default").await;
    assert!(default_url.contains("service-health"));
    
    let fallback_url = health_service.get_health_endpoint_url("fallback").await;
    assert!(fallback_url.contains("service-health"));
}

#[tokio::test]
async fn test_t13_1_redis_cache_integration() {
    // Test that RedisCache can be created and used
    let cache = RedisCache::new();
    
    // Test basic functionality
    let memory_limit = cache.get_memory_limit_mb();
    assert_eq!(memory_limit, 50);
    
    // Test that we can get entry count
    let entry_count = cache.get_entry_count().await;
    assert_eq!(entry_count, 0);
    
    // Test that we can get memory usage
    let memory_usage = cache.get_memory_usage_mb().await;
    assert_eq!(memory_usage, 0);
}

#[tokio::test]
async fn test_t13_1_all_modules_integration() {
    // Test that all modules can work together
    let services = ApplicationServices::new();
    
    // Test that we can get processor information
    let processors = services.payment_processor.get_processor_info().await;
    let processor_names: Vec<String> = processors.keys().cloned().collect();
    
    // Test that health service can work with processor names
    for processor_name in &processor_names {
        let url = services.health_manager.get_health_endpoint_url(processor_name).await;
        assert!(!url.is_empty());
    }
    
    // Test that cache can store and retrieve data
    let test_data = "test_value";
    let result = services.cache_manager.set("test_key", &test_data, std::time::Duration::from_secs(60)).await;
    assert!(result.is_ok());
    
    let retrieved: Option<String> = services.cache_manager.get("test_key").await.unwrap();
    assert_eq!(retrieved, Some(test_data.to_string()));
} 