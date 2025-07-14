mod modules;

use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use modules::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new();
    
    env_logger::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    log::info!("Starting server on {}", config.server_addr());

    let listener = tokio::net::TcpListener::bind(config.server_addr()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Rinha de Backend 2025 - Rust Implementation"
}

async fn health() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "service": "rinha-backend-2025",
            "version": "0.1.0"
        }))
    )
}
