use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PaymentRequest {
    #[validate(custom = "validate_uuid")]
    pub correlation_id: String,
    
    #[validate(range(min = 0.01))]
    pub amount: f64,
}

fn validate_uuid(correlation_id: &str) -> Result<(), validator::ValidationError> {
    if Uuid::parse_str(correlation_id).is_ok() {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_uuid"))
    }
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub message: String,
} 