use rinha::modules::processors::selector::ProcessorSelector;
use rinha::modules::processors::selector::ProcessorInfo;
use std::collections::HashMap;
use std::sync::Arc;
use httpmock::MockServer;
use httpmock::Method::POST;
use reqwest::Client;

#[tokio::test]
async fn test_processor_selector_creation() {
    let selector = ProcessorSelector::new();
    let processors = selector.get_processors().await;
    assert_eq!(processors.len(), 2);
    let default_processor = selector.get_default_processor().await;
    assert!(default_processor.is_some());
}

#[tokio::test]
async fn test_processor_rate_tracking() {
    let selector = ProcessorSelector::new();
    for i in 0..10 {
        let correlation_id = format!("test-{}", i);
        let _ = selector.process_payment(&correlation_id, 100.0).await;
    }
    let rates = selector.get_processor_rates().await;
    assert!(rates.len() > 0);
    for (_, rate) in rates {
        assert!(rate >= 0.0);
    }
}

#[tokio::test]
async fn test_lowest_rate_selection() {
    let selector = ProcessorSelector::new();
    selector.update_processor_rate("default", 0.05).await;
    selector.update_processor_rate("fallback", 0.03).await;
    let default_processor = selector.get_default_processor().await;
    assert_eq!(default_processor.unwrap().name, "fallback");
}

#[tokio::test]
async fn test_automatic_fallback_on_failure() {
    let server = MockServer::start_async().await;
    // Default processor will fail
    let _default_mock = server.mock_async(|when, then| {
        when.method(POST).path("/default/payments");
        then.status(500);
    }).await;
    // Fallback processor will succeed
    let _fallback_mock = server.mock_async(|when, then| {
        when.method(POST).path("/fallback/payments");
        then.status(200).json_body_obj(&serde_json::json!({"message": "ok"}));
    }).await;
    let mut config = HashMap::new();
    config.insert("default".to_string(), server.url("/default"));
    config.insert("fallback".to_string(), server.url("/fallback"));
    let client = Client::new();
    let selector = ProcessorSelector::with_config_and_client(config, client);
    selector.update_processor_rate("default", 0.01).await;
    selector.update_processor_rate("fallback", 0.02).await;
    selector.mark_processor_healthy("default").await;
    selector.mark_processor_healthy("fallback").await;
    let correlation_id = "test-fallback";
    let result = selector.process_payment(correlation_id, 100.0).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_processor_health_tracking() {
    let selector = ProcessorSelector::new();
    assert!(selector.is_processor_healthy("default").await);
    assert!(selector.is_processor_healthy("fallback").await);
    selector.mark_processor_failed("default").await;
    assert!(!selector.is_processor_healthy("default").await);
    assert!(selector.is_processor_healthy("fallback").await);
}

#[tokio::test]
async fn test_processor_recovery() {
    let selector = ProcessorSelector::new();
    selector.mark_processor_failed("default").await;
    assert!(!selector.is_processor_healthy("default").await);
    selector.mark_processor_healthy("default").await;
    assert!(selector.is_processor_healthy("default").await);
}

#[tokio::test]
async fn test_concurrent_processor_access() {
    let server = MockServer::start_async().await;
    let _mock = server.mock_async(|when, then| {
        when.method(POST).path("/payments");
        then.status(200).json_body_obj(&serde_json::json!({"message": "ok"}));
    }).await;
    let mut config = HashMap::new();
    config.insert("default".to_string(), server.url(""));
    config.insert("fallback".to_string(), server.url(""));
    let client = Client::new();
    let selector = Arc::new(ProcessorSelector::with_config_and_client(config, client));
    let mut handles = vec![];
    for i in 0..10 {
        let selector_clone = Arc::clone(&selector);
        let handle = tokio::spawn(async move {
            let correlation_id = format!("concurrent-{}", i);
            selector_clone.process_payment(&correlation_id, 100.0).await
        });
        handles.push(handle);
    }
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_processor_info_structure() {
    let info = ProcessorInfo {
        name: "test".to_string(),
        url: "http://test:8080".to_string(),
        rate: 0.05,
        is_healthy: true,
        failure_count: 0,
    };
    assert_eq!(info.name, "test");
    assert_eq!(info.url, "http://test:8080");
    assert_eq!(info.rate, 0.05);
    assert!(info.is_healthy);
    assert_eq!(info.failure_count, 0);
}

#[tokio::test]
async fn test_processor_selector_configuration() {
    let mut config = HashMap::new();
    config.insert("default".to_string(), "http://payment-processor-default:8080".to_string());
    config.insert("fallback".to_string(), "http://payment-processor-fallback:8080".to_string());
    let selector = ProcessorSelector::with_config(config);
    let processors = selector.get_processors().await;
    assert_eq!(processors.len(), 2);
    assert!(processors.contains_key("default"));
    assert!(processors.contains_key("fallback"));
} 