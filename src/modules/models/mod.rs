use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use rmp_serde::{from_slice, to_vec_named};

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

impl PaymentRequest {
    /// Deserializa um payload MessagePack para PaymentRequest
    pub fn from_msgpack(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        from_slice(data)
    }
}

impl PaymentResponse {
    /// Serializa PaymentResponse para MessagePack
    pub fn to_msgpack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        to_vec_named(self)
    }
} 