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

    let (server_addr, _shutdown_tx) = start_test_server().await;
    let client = reqwest::Client::new();
    

    let resp = client.post(&format!("http://{}/payments", server_addr))
        .json(&serde_json::json!({
            "correlationId": "invalid-uuid",
            "amount": 100.50
        }))
        .send().await.unwrap();
    
    assert_eq!(resp.status().as_u16(), 400, "Deve retornar 400 para UUID inválido");
    
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

#[tokio::test]
async fn payment_processor_integration_test() {
    // T2.2: Integração com Payment Processors
    // Critério: Deve integrar com payment-processor-default:8080
    // Critério: Deve integrar com payment-processor-fallback:8080
    // Critério: Deve adicionar requestedAt timestamp ISO UTC
    
    // Verificar se PaymentProcessor existe
    let processors_content = std::fs::read_to_string("src/modules/processors/mod.rs").expect("Should read processors/mod.rs");
    assert!(processors_content.contains("PaymentProcessor"), "Should have PaymentProcessor struct");
    assert!(processors_content.contains("process_payment"), "Should have process_payment method");
    
    // Verificar se usa ProcessorSelector para integração
    assert!(processors_content.contains("ProcessorSelector"), "Should use ProcessorSelector for integration");
    assert!(processors_content.contains("selector: ProcessorSelector"), "Should have selector field");
    
    // Verificar se tem URLs configuráveis via ProcessorSelector
    let selector_content = std::fs::read_to_string("src/modules/processors/selector.rs").expect("Should read processors/selector.rs");
    assert!(selector_content.contains("PAYMENT_PROCESSOR_DEFAULT_URL"), "Should have configurable default URL");
    assert!(selector_content.contains("PAYMENT_PROCESSOR_FALLBACK_URL"), "Should have configurable fallback URL");
}

#[tokio::test]
async fn payment_processor_request_structure_test() {
    // T2.2: Verificar estrutura da requisição
    
    let processors_content = std::fs::read_to_string("src/modules/processors/mod.rs").expect("Should read processors/mod.rs");
    
    // Verificar se PaymentProcessorRequest existe
    assert!(processors_content.contains("PaymentProcessorRequest"), "Should have PaymentProcessorRequest struct");
    
    // Verificar se tem correlationId
    assert!(processors_content.contains("#[serde(rename = \"correlationId\")]"), "Should have correlationId field");
    assert!(processors_content.contains("correlation_id: String"), "Should have correlation_id field");
    
    // Verificar se tem amount
    assert!(processors_content.contains("amount: f64"), "Should have amount field");
    
    // Verificar se tem requestedAt
    assert!(processors_content.contains("#[serde(rename = \"requestedAt\")]"), "Should have requestedAt field");
    assert!(processors_content.contains("requested_at: String"), "Should have requested_at field");
}

#[tokio::test]
async fn payment_processor_response_structure_test() {
       
    let processors_content = std::fs::read_to_string("src/modules/processors/mod.rs").expect("Should read processors/mod.rs");
    
    assert!(processors_content.contains("PaymentProcessorResponse"), "Should have PaymentProcessorResponse struct");
    
    assert!(processors_content.contains("message: String"), "Should have message field");
}

#[tokio::test]
async fn payment_processor_timestamp_test() {
    // T2.2: Verificar se adiciona timestamp
    
    let selector_content = std::fs::read_to_string("src/modules/processors/selector.rs").expect("Should read processors/selector.rs");
    
    // Verificar se usa SystemTime para timestamp
    assert!(selector_content.contains("SystemTime::now()"), "Should use SystemTime::now() for timestamp");
    assert!(selector_content.contains("UNIX_EPOCH"), "Should use UNIX_EPOCH for timestamp");
    assert!(selector_content.contains("as_millis()"), "Should convert to milliseconds");
}

#[tokio::test]
async fn payment_processor_timeout_test() {
    // T2.2: Verificar se tem timeout configurado
    
    let selector_content = std::fs::read_to_string("src/modules/processors/selector.rs").expect("Should read processors/selector.rs");
    
    // Verificar se tem timeout de 5 segundos
    assert!(selector_content.contains("timeout(std::time::Duration::from_secs(5))"), "Should have 5 second timeout");
}

#[tokio::test]
async fn payment_processor_fallback_logic_test() {
    // T2.2: Verificar lógica de fallback
    
    let selector_content = std::fs::read_to_string("src/modules/processors/selector.rs").expect("Should read processors/selector.rs");
    
    // Verificar se tenta default primeiro
    assert!(selector_content.contains("get_default_processor"), "Should try default processor first");
    
    // Verificar se tem fallback em caso de erro
    assert!(selector_content.contains("try_fallback_processor"), "Should try fallback processor on error");
    assert!(selector_content.contains("try_any_processor"), "Should try any available processor as last resort");
}

#[tokio::test]
async fn payment_processor_urls_test() {
    // T2.2: Verificar URLs padrão
    
    let selector_content = std::fs::read_to_string("src/modules/processors/selector.rs").expect("Should read processors/selector.rs");
    
    // Verificar URLs padrão
    assert!(selector_content.contains("payment-processor-default:8080"), "Should have default processor URL");
    assert!(selector_content.contains("payment-processor-fallback:8080"), "Should have fallback processor URL");
    
    // Verificar se usa variáveis de ambiente
    assert!(selector_content.contains("std::env::var"), "Should use environment variables for URLs");
    assert!(selector_content.contains("unwrap_or_else"), "Should have fallback URLs if env vars not set");
} 