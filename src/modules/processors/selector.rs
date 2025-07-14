use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorInfo {
    pub name: String,
    pub url: String,
    pub rate: f64,
    pub is_healthy: bool,
    pub failure_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProcessorRequest {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
    #[serde(rename = "requestedAt")]
    pub requested_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProcessorResponse {
    pub message: String,
}

#[allow(dead_code)]
pub struct ProcessorSelector {
    processors: Arc<RwLock<HashMap<String, ProcessorInfo>>>,
    client: Client,
}

impl ProcessorSelector {
    pub fn new() -> Self {
        let mut config = HashMap::new();
        let default_url = std::env::var("PAYMENT_PROCESSOR_DEFAULT_URL")
            .unwrap_or_else(|_| "http://payment-processor-default:8080".to_string());
        let fallback_url = std::env::var("PAYMENT_PROCESSOR_FALLBACK_URL")
            .unwrap_or_else(|_| "http://payment-processor-fallback:8080".to_string());
        
        config.insert("default".to_string(), default_url);
        config.insert("fallback".to_string(), fallback_url);
        
        Self::with_config_and_client(config, Client::new())
    }

    pub fn with_config(config: HashMap<String, String>) -> Self {
        Self::with_config_and_client(config, Client::new())
    }

    pub fn with_config_and_client(config: HashMap<String, String>, client: Client) -> Self {
        let mut processors = HashMap::new();
        
        for (name, url) in config {
            processors.insert(name.clone(), ProcessorInfo {
                name,
                url,
                rate: 0.05, // Default rate
                is_healthy: true,
                failure_count: 0,
            });
        }
        
        Self {
            processors: Arc::new(RwLock::new(processors)),
            client,
        }
    }

    pub async fn get_processors(&self) -> HashMap<String, ProcessorInfo> {
        self.processors.read().await.clone()
    }

    pub async fn get_default_processor(&self) -> Option<ProcessorInfo> {
        let processors = self.processors.read().await;
        
        // Find the processor with the lowest rate that is healthy
        processors
            .values()
            .filter(|p| p.is_healthy)
            .min_by(|a, b| a.rate.partial_cmp(&b.rate).unwrap_or(std::cmp::Ordering::Equal))
            .cloned()
    }

    pub async fn get_processor_rates(&self) -> HashMap<String, f64> {
        let processors = self.processors.read().await;
        processors
            .iter()
            .map(|(name, info)| (name.clone(), info.rate))
            .collect()
    }

    pub async fn update_processor_rate(&self, name: &str, rate: f64) {
        let mut processors = self.processors.write().await;
        if let Some(processor) = processors.get_mut(name) {
            processor.rate = rate;
        }
    }

    pub async fn mark_processor_failed(&self, name: &str) {
        let mut processors = self.processors.write().await;
        if let Some(processor) = processors.get_mut(name) {
            processor.is_healthy = false;
            processor.failure_count += 1;
        }
    }

    pub async fn mark_processor_healthy(&self, name: &str) {
        let mut processors = self.processors.write().await;
        if let Some(processor) = processors.get_mut(name) {
            processor.is_healthy = true;
            processor.failure_count = 0;
        }
    }

    pub async fn is_processor_healthy(&self, name: &str) -> bool {
        let processors = self.processors.read().await;
        processors.get(name).map(|p| p.is_healthy).unwrap_or(false)
    }

    pub async fn process_payment(
        &self,
        correlation_id: &str,
        amount: f64,
    ) -> Result<PaymentProcessorResponse, Box<dyn std::error::Error + Send + Sync>> {
        let requested_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let payload = PaymentProcessorRequest {
            correlation_id: correlation_id.to_string(),
            amount,
            requested_at: format!("{}", requested_at),
        };

        // Get the default processor (lowest rate)
        let default_processor = self.get_default_processor().await;
        
        if let Some(processor) = default_processor {
            // Try the default processor first
            match self.try_processor(&processor, &payload).await {
                Ok(response) => {
                    // Success - mark processor as healthy
                    self.mark_processor_healthy(&processor.name).await;
                    Ok(response)
                }
                Err(_) => {
                    // Mark as failed and try fallback
                    self.mark_processor_failed(&processor.name).await;
                    self.try_fallback_processor(&payload).await
                }
            }
        } else {
            // No healthy processors available, try any available processor
            self.try_any_processor(&payload).await
        }
    }

    async fn try_processor(
        &self,
        processor: &ProcessorInfo,
        payload: &PaymentProcessorRequest,
    ) -> Result<PaymentProcessorResponse, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .post(&format!("{}/payments", processor.url))
            .json(payload)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;

        if response.status().is_success() {
            let processor_response: PaymentProcessorResponse = response.json().await?;
            Ok(processor_response)
        } else {
            Err("Processor failed".into())
        }
    }

    async fn try_fallback_processor(
        &self,
        payload: &PaymentProcessorRequest,
    ) -> Result<PaymentProcessorResponse, Box<dyn std::error::Error + Send + Sync>> {
        let processors_guard = self.processors.read().await;
        let processors: Vec<ProcessorInfo> = processors_guard.values().cloned().collect();
        drop(processors_guard); // Release lock before async call
        
        // Try any healthy processor that's not the default
        for processor in processors.iter() {
            if processor.is_healthy {
                match self.try_processor(processor, payload).await {
                    Ok(response) => {
                        self.mark_processor_healthy(&processor.name).await;
                        return Ok(response);
                    }
                    Err(_) => {
                        self.mark_processor_failed(&processor.name).await;
                        continue;
                    }
                }
            }
        }
        
        Err("All processors failed".into())
    }

    async fn try_any_processor(
        &self,
        payload: &PaymentProcessorRequest,
    ) -> Result<PaymentProcessorResponse, Box<dyn std::error::Error + Send + Sync>> {
        let processors_guard = self.processors.read().await;
        let processors: Vec<ProcessorInfo> = processors_guard.values().cloned().collect();
        drop(processors_guard); // Release lock before async call
        
        // Try any available processor
        for processor in processors.iter() {
            match self.try_processor(processor, payload).await {
                Ok(response) => {
                    self.mark_processor_healthy(&processor.name).await;
                    return Ok(response);
                }
                Err(_) => {
                    self.mark_processor_failed(&processor.name).await;
                    continue;
                }
            }
        }
        
        Err("No processors available".into())
    }
} 