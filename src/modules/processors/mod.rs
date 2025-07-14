use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub struct PaymentProcessor {
    client: Client,
    default_url: String,
    fallback_url: String,
}

impl PaymentProcessor {
    pub fn new() -> Self {
        let default_url = std::env::var("PAYMENT_PROCESSOR_DEFAULT_URL")
            .unwrap_or_else(|_| "http://payment-processor-default:8080".to_string());
        let fallback_url = std::env::var("PAYMENT_PROCESSOR_FALLBACK_URL")
            .unwrap_or_else(|_| "http://payment-processor-fallback:8080".to_string());
        
        Self {
            client: Client::new(),
            default_url,
            fallback_url,
        }
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

        // Tentar processador default primeiro
        match self.try_default_processor(&payload).await {
            Ok(response) => Ok(response),
            Err(_) => {
                // Fallback para processador secundário
                self.try_fallback_processor(&payload).await
            }
        }
    }

    async fn try_default_processor(
        &self,
        payload: &PaymentProcessorRequest,
    ) -> Result<PaymentProcessorResponse, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .post(&format!("{}/payments", self.default_url))
            .json(payload)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;

        if response.status().is_success() {
            let processor_response: PaymentProcessorResponse = response.json().await?;
            Ok(processor_response)
        } else {
            Err("Default processor failed".into())
        }
    }

    async fn try_fallback_processor(
        &self,
        payload: &PaymentProcessorRequest,
    ) -> Result<PaymentProcessorResponse, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .post(&format!("{}/payments", self.fallback_url))
            .json(payload)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;

        if response.status().is_success() {
            let processor_response: PaymentProcessorResponse = response.json().await?;
            Ok(processor_response)
        } else {
            Err("Fallback processor also failed".into())
        }
    }
} 