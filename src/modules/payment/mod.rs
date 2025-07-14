use axum::{
    extract::Json,
    http::{StatusCode, HeaderMap},
    response::Response,
};
use crate::modules::models::{PaymentRequest, PaymentResponse};
use validator::Validate;

pub async fn create_payment(
    Json(payment): Json<PaymentRequest>,
) -> Result<Response, StatusCode> {
    // Validar input
    if let Err(_) = payment.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Por enquanto, apenas retorna sucesso
    // TODO: Implementar integração com Payment Processors (T2.3)
    let response = PaymentResponse {
        message: "payment processed successfully".to_string(),
    };

    // Retornar em MessagePack para otimização (T2.2)
    let msgpack_data = response.to_msgpack()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/x-msgpack")
        .body(axum::body::Body::from(msgpack_data))
        .unwrap())
} 