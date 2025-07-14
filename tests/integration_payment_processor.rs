use axum::{Router, routing::post};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use tokio::sync::oneshot;
use axum::serve;
use reqwest;
use rinha::modules::payment::create_payment;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaymentPayload {
    #[serde(rename = "correlationId")]
    correlation_id: String,
    amount: f64,
    #[serde(rename = "requestedAt")]
    requested_at: String,
}

async fn start_test_server() -> (SocketAddr, oneshot::Sender<()>) {
    let app = Router::new()
        .route("/payments", post(create_payment));
    
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let actual_addr = listener.local_addr().unwrap();
    
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        serve(listener, app).with_graceful_shutdown(async {
            rx.await.ok();
        }).await.unwrap();
    });
    
    // Aguarda servidor subir
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    (actual_addr, tx)
}

#[tokio::test]
async fn test_payment_validation() {
    // Teste de validação de inputs
    let (server_addr, _shutdown_tx) = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Teste com UUID inválido
    let resp = client.post(&format!("http://{}/payments", server_addr))
        .json(&serde_json::json!({
            "correlationId": "invalid-uuid",
            "amount": 100.50
        }))
        .send().await.unwrap();
    
    assert_eq!(resp.status().as_u16(), 400, "Deve retornar 400 para UUID inválido");
    
    // Teste com amount inválido
    let resp = client.post(&format!("http://{}/payments", server_addr))
        .json(&serde_json::json!({
            "correlationId": "4a7901b8-7d26-4d9d-aa19-4dc1c7cf60b3",
            "amount": 0.0
        }))
        .send().await.unwrap();
    
    assert_eq!(resp.status().as_u16(), 400, "Deve retornar 400 para amount inválido");
}

#[tokio::test]
#[ignore]
async fn test_real_integration_with_payment_processors() {
    // Este teste só roda se os containers/endpoints reais estiverem disponíveis
    // Exemplo de chamada real:
    // let client = reqwest::Client::new();
    // let resp = client.post("http://localhost:9999/payments")
    //     .json(&PaymentPayload { ... })
    //     .send().await.unwrap();
    // assert_eq!(resp.status(), StatusCode::OK);
    // // Validar fallback, payload, etc.
    assert!(true, "Teste real será implementado quando containers estiverem disponíveis");
} 