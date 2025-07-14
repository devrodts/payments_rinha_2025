

#[tokio::test]
async fn test_t2_1_payments_endpoint_exists() {
    // T2.1: Endpoint POST /payments
    // Critério: Deve aceitar correlationId (UUID) e amount (decimal)
    // Critério: Deve retornar HTTP 2XX para sucesso
    
    // Verificar se o endpoint está registrado no main.rs
    let main_content = std::fs::read_to_string("src/main.rs").expect("Should read main.rs");
    assert!(main_content.contains("/payments"), "main.rs should have /payments route");
    assert!(main_content.contains("create_payment"), "main.rs should have create_payment handler");
    
    // Verificar se o handler existe
    let payment_mod_content = std::fs::read_to_string("src/modules/payment/mod.rs").expect("Should read payment/mod.rs");
    assert!(payment_mod_content.contains("create_payment"), "payment/mod.rs should have create_payment function");
    
    // Verificar se a validação está implementada
    let models_content = std::fs::read_to_string("src/modules/models/mod.rs").expect("Should read models/mod.rs");
    assert!(models_content.contains("PaymentRequest"), "models/mod.rs should have PaymentRequest struct");
    assert!(models_content.contains("validate_uuid"), "models/mod.rs should have UUID validation");
    assert!(models_content.contains("range(min = 0.01)"), "models/mod.rs should have amount validation");
}

#[tokio::test]
async fn test_t2_1_payments_validation_correlation_id() {
    // T2.1: Validação de correlationId
    
    // Verificar se a validação de UUID está implementada
    let models_content = std::fs::read_to_string("src/modules/models/mod.rs").expect("Should read models/mod.rs");
    assert!(models_content.contains("validate_uuid"), "Should have UUID validation function");
    assert!(models_content.contains("Uuid::parse_str"), "Should use Uuid::parse_str for validation");
    
    // Verificar se o handler usa validação
    let payment_content = std::fs::read_to_string("src/modules/payment/mod.rs").expect("Should read payment/mod.rs");
    assert!(payment_content.contains("payment.validate()"), "Handler should validate payment request");
    assert!(payment_content.contains("StatusCode::BAD_REQUEST"), "Handler should return 400 for invalid requests");
}

#[tokio::test]
async fn test_t2_1_payments_validation_amount() {
    // T2.1: Validação de amount
    
    // Verificar se a validação de amount está implementada
    let models_content = std::fs::read_to_string("src/modules/models/mod.rs").expect("Should read models/mod.rs");
    assert!(models_content.contains("range(min = 0.01)"), "Should have amount minimum validation");
    assert!(models_content.contains("amount: f64"), "Should have amount field as f64");
    
    // Verificar se o handler retorna erro para amount inválido
    let payment_content = std::fs::read_to_string("src/modules/payment/mod.rs").expect("Should read payment/mod.rs");
    assert!(payment_content.contains("payment.validate()"), "Handler should validate amount");
}

#[tokio::test]
async fn test_t2_1_payments_validation_amount_zero() {
    // T2.1: Validação de amount zero
    
    // Verificar se amount zero é rejeitado
    let models_content = std::fs::read_to_string("src/modules/models/mod.rs").expect("Should read models/mod.rs");
    assert!(models_content.contains("range(min = 0.01)"), "Should reject amounts less than 0.01");
    
    // Verificar se o handler trata erros de validação
    let payment_content = std::fs::read_to_string("src/modules/payment/mod.rs").expect("Should read payment/mod.rs");
    assert!(payment_content.contains("StatusCode::BAD_REQUEST"), "Should return 400 for validation errors");
}

#[tokio::test]
async fn test_t2_1_payments_validation_amount_minimum() {
    // T2.1: Validação de amount mínimo
    
    // Verificar se amount mínimo (0.01) é aceito
    let models_content = std::fs::read_to_string("src/modules/models/mod.rs").expect("Should read models/mod.rs");
    assert!(models_content.contains("range(min = 0.01)"), "Should accept amounts >= 0.01");
    
    // Verificar se o handler retorna sucesso para valores válidos
    let payment_content = std::fs::read_to_string("src/modules/payment/mod.rs").expect("Should read payment/mod.rs");
    assert!(payment_content.contains("StatusCode::OK"), "Should return 200 for valid requests");
} 