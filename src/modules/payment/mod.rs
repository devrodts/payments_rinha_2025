use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use crate::modules::models::{PaymentRequest, PaymentResponse};
use crate::modules::processors::PaymentProcessor;
use validator::Validate;

#[debug_handler]
pub async fn create_payment(
    Json(payment): Json<PaymentRequest>,
) -> impl IntoResponse {
    if let Err(_) = payment.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let processor = PaymentProcessor::new();
    match processor.process_payment(&payment.correlation_id, payment.amount).await {
        Ok(processor_response) => {
            let response = PaymentResponse {
                message: processor_response.message,
            };
            (StatusCode::OK, axum::Json(response)).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
} 