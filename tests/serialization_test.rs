use serde::{Serialize, Deserialize};
use rmp_serde::to_vec_named;
use serde_json::to_vec as to_json_vec;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
struct PaymentRequest {
    correlation_id: String,
    amount: f64,
    metadata: PaymentMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
struct PaymentMetadata {
    user_id: String,
    timestamp: String,
    currency: String,
    description: String,
    tags: Vec<String>,
}

fn make_payment() -> PaymentRequest {
    PaymentRequest {
        correlation_id: "4a7901b8-7d26-4d9d-aa19-4dc1c7cf60b3".to_string(),
        amount: 19.90,
        metadata: PaymentMetadata {
            user_id: "user123".to_string(),
            timestamp: "2025-01-27T18:30:00Z".to_string(),
            currency: "BRL".to_string(),
            description: "Payment for services".to_string(),
            tags: vec!["service".to_string(), "monthly".to_string()],
        },
    }
}

#[test]
fn test_msgpack_is_smaller_than_json() {
    let payment = make_payment();
    let json_bytes = to_json_vec(&payment).unwrap();
    let msgpack_bytes = to_vec_named(&payment).unwrap();
    let ratio = msgpack_bytes.len() as f64 / json_bytes.len() as f64;
    assert!(ratio <= 0.80, "MessagePack deve ser <= 80% do tamanho do JSON, ratio atual: {:.2}", ratio);
}

#[test]
fn test_msgpack_is_faster_than_json() {
    let payment = make_payment();
    let start_json = Instant::now();
    for _ in 0..10_000 {
        let _ = to_json_vec(&payment).unwrap();
    }
    let json_duration = start_json.elapsed();
    let start_msgpack = Instant::now();
    for _ in 0..10_000 {
        let _ = to_vec_named(&payment).unwrap();
    }
    let msgpack_duration = start_msgpack.elapsed();
    let speedup = json_duration.as_secs_f64() / msgpack_duration.as_secs_f64();
    assert!(speedup >= 1.4, "MessagePack deve ser >= 40% mais rápido que JSON, speedup atual: {:.2}", speedup);
} 