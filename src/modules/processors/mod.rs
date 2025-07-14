use serde::{Serialize, Deserialize};

pub mod selector;
use selector::ProcessorSelector;

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
    selector: ProcessorSelector,
}

impl PaymentProcessor {
    pub fn new() -> Self {
        Self {
            selector: ProcessorSelector::new(),
        }
    }

    pub async fn process_payment(
        &self,
        correlation_id: &str,
        amount: f64,
    ) -> Result<PaymentProcessorResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Use the ProcessorSelector for intelligent routing
        let selector_response = self.selector.process_payment(correlation_id, amount).await?;
        
        // Convert selector response to our response type
        Ok(PaymentProcessorResponse {
            message: selector_response.message,
        })
    }

    // Expose ProcessorSelector methods for advanced usage
    pub async fn get_processor_info(&self) -> std::collections::HashMap<String, selector::ProcessorInfo> {
        self.selector.get_processors().await
    }

    pub async fn get_processor_rates(&self) -> std::collections::HashMap<String, f64> {
        self.selector.get_processor_rates().await
    }

    pub async fn is_processor_healthy(&self, name: &str) -> bool {
        self.selector.is_processor_healthy(name).await
    }
} 