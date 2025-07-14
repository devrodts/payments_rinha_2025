use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
};
use crate::modules::models::{PaymentRequest, PaymentResponse};
use validator::Validate;

pub async fn create_payment(
    Json(payment): Json<PaymentRequest>,
) -> Result<JsonResponse<PaymentResponse>, StatusCode> {
    // Validar input
    if let Err(_) = payment.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Por enquanto, apenas retorna sucesso
    // TODO: Implementar integração com Payment Processors (T2.3)
    Ok(JsonResponse(PaymentResponse {
        message: "payment processed successfully".to_string(),
    }))
} 