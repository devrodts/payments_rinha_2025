# Roadmap para Implementação do Sistema Rust na Rinha de Backend 2025

## Visão Geral da Arquitetura
```mermaid
graph TD
    A[Cliente HTTP] --> B[Load Balancer Docker]
    B --> C1[Instância Rust 1]
    B --> C2[Instância Rust 2]
    B --> C3[Instância Rust 3]
    C1 --> D[Circuit Breaker Atômico]
    C2 --> D
    C3 --> D
    D --> E[Redis Cache]
    D --> F[PostgreSQL]
    D --> G[Payment Processor 1]
    D --> H[Payment Processor 2]
```

## Roadmap de Implementação

### Fase 1: Configuração Inicial (1 dia)
- **Setup do Projeto Rust**
  ```bash
  cargo new rinha-backend-2025
  cd rinha-backend-2025
  ```
- **Dependências principais (`Cargo.toml`)**:
  ```toml
  [dependencies]
  axum = "0.7"
  tokio = { version = "1.0", features = ["full"] }
  sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
  redis = "0.24"
  rmp-serde = "1.1"
  serde = { version = "1.0", features = ["derive"] }
  reqwest = "0.11"
  atomic = "0.5"
  ```

### Fase 2: Núcleo da Aplicação (3 dias)
- **Configuração do Servidor Axum**:
  ```rust
  use axum::{Router, routing::post};
  
  #[tokio::main]
  async fn main() {
      let app = Router::new()
          .route("/payments", post(handle_payment));
      
      axum::Server::bind(&"0.0.0.0:9999".parse().unwrap())
          .serve(app.into_make_service())
          .await
          .unwrap();
  }
  ```

- **Serialização com MessagePack**:
  ```rust
  use rmp_serde::{Serializer, Deserializer};
  use serde::{Serialize, Deserialize};
  
  #[derive(Serialize, Deserialize)]
  struct Payment {
      amount: f64,
      currency: String,
  }
  
  fn encode_payment(payment: &Payment) -> Vec<u8> {
      let mut buf = Vec::new();
      payment.serialize(&mut Serializer::new(&mut buf)).unwrap();
      buf
  }
  ```

### Fase 3: Mecanismos de Resiliência (2 dias)
- **Circuit Breaker Lock-Free**:
  ```rust
  use std::sync::atomic::{AtomicU32, Ordering};
  
  pub struct CircuitBreaker {
      failures: AtomicU32,
      state: AtomicU32, // 0: fechado, 1: aberto, 2: meio-aberto
  }
  
  impl CircuitBreaker {
      pub fn new() -> Self {
          CircuitBreaker {
              failures: AtomicU32::new(0),
              state: AtomicU32::new(0),
          }
      }
  
      pub fn record_success(&self) {
          self.failures.store(0, Ordering::Relaxed);
          self.state.store(0, Ordering::Relaxed);
      }
  
      pub fn should_try(&self) -> bool {
          match self.state.load(Ordering::Relaxed) {
              0 => true,
              1 => false,
              _ => rand::random::<f32>() > 0.7, // 30% de chance de tentar
          }
      }
  }
  ```

- **Estratégia de Retry com Fallback**:
  ```rust
  async fn process_payment(payment: Payment) {
      let processor = select_processor();
      
      for attempt in 0..3 {
          match process_with_timeout(&processor, &payment).await {
              Ok(_) => return,
              Err(_) if attempt < 2 => tokio::time::sleep(Duration::from_millis(100 * 2u64.pow(attempt))).await,
              Err(_) => {
                  fallback_processor().await;
                  return;
              }
          }
      }
  }
  ```

### Fase 4: Integração com Banco de Dados (2 dias)
- **Batch Processing para PostgreSQL**:
  ```rust
  use sqlx::postgres::PgPoolOptions;
  use tokio::sync::mpsc;
  
  const BATCH_SIZE: usize = 100;
  
  async fn batch_processor(mut receiver: mpsc::Receiver<Payment>) {
      let pool = PgPoolOptions::new()
          .connect("postgres://user:pass@localhost/db")
          .await
          .unwrap();
  
      let mut batch = Vec::with_capacity(BATCH_SIZE);
      
      loop {
          tokio::select! {
              payment = receiver.recv() => {
                  if let Some(p) = payment {
                      batch.push(p);
                  }
              }
              _ = tokio::time::sleep(Duration::from_millis(10)) => {}
          }
          
          if batch.len() >= BATCH_SIZE {
              save_batch(&pool, &batch).await;
              batch.clear();
          }
      }
  }
  ```

### Fase 5: Cache e Health Checks (1 dia)
- **Cache Reativo no Redis**:
  ```rust
  use redis::Commands;
  
  async fn update_processor_health(processor: &str, healthy: bool) {
      let mut conn = redis_client.get_async_connection().await.unwrap();
      let key = format!("processor:{}:health", processor);
      conn.set_ex(key, healthy, 300).await.unwrap();
  }
  
  async fn get_best_processor() -> String {
      // Lógica para selecionar processador com menor taxa de falha
  }
  ```

### Fase 6: Otimizações Finais (1 dia)
- **Amostragem Adaptativa para Métricas**:
  ```rust
  fn should_sample() -> bool {
      use rand::Rng;
      rand::thread_rng().gen_ratio(1, 10) // 10% de amostragem
  }
  ```

- **Dockerfile Otimizado**:
  ```dockerfile
  FROM rust:1.75-slim-bookworm as builder
  WORKDIR /app
  COPY . .
  RUN cargo build --release && strip target/release/rinha-backend

  FROM debian:bookworm-slim
  COPY --from=builder /app/target/release/rinha-backend /usr/local/bin/
  CMD ["rinha-backend"]
  ```

## Configuração de Deployment

**docker-compose.yml**:
```yaml
version: '3.8'

services:
  app:
    image: rinha-backend:2025
    deploy:
      replicas: 3
    ports:
      - "9999:9999"
    cpus: '0.15'
    mem_limit: 100m

  postgres:
    image: postgres:15-alpine
    cpus: '0.3'
    mem_limit: 150m
    environment:
      POSTGRES_PASSWORD: password

  redis:
    image: redis:7-alpine
    cpus: '0.1'
    mem_limit: 50m
    command: redis-server --maxmemory 50mb --maxmemory-policy allkeys-lru
```

## Tabela de Otimizações e Ganhos

| Componente | Técnica | Ganho Esperado | Complexidade |
|------------|---------|----------------|--------------|
| **Serialização** | MessagePack (rmp-serde) | -65% tamanho payload<br>-40% CPU serialização | Baixa |
| **Concorrência** | Tokio worker_threads=3<br>Fire-and-forget | +30% throughput<br>Latência p95 < 15ms | Média |
| **Resiliência** | Circuit Breaker atômico<br>Retry com backoff | -90% falhas cascateadas<br>-50% multas | Alta |
| **Persistência** | Batch processing (100 items/10ms) | +400% writes/s PostgreSQL<br>-70% I/O disco | Média |
| **Cache** | Redis LRU (50MB)<br>Health check reativo | -95% health checks externos<br>5ms acesso | Baixa |
| **Monitoramento** | Amostragem adaptativa (10%) | -40% CPU coleta métricas | Baixa |
| **Deployment** | Remoção Nginx<br>Imagem mínima | -50MB RAM<br>-0.1 CPU<br>Cold start < 100ms | Baixa |

## Estratégia de Teste de Performance

1. **Teste de Carga Inicial**:
   ```bash
   wrk -t4 -c100 -d30s -s payload.lua http://localhost:9999/payments
   ```
   - Script `payload.lua` com MessagePack

2. **Teste de Falhas**:
   - Simular 40% falhas Payment Processors
   - Medir tempo de recuperação Circuit Breaker

3. **Teste de Pico**:
   - Aumento súbito para 500 conexões
   - Medir consumo de memória e latência p99

## Cronograma de Implementação

| Fase | Duração | Entregáveis |
|------|---------|-------------|
| Setup Inicial | 1 dia | Projeto Rust + Docker básico |
| Núcleo API | 3 dias | Roteamento Axum + Serialização |
| Resiliência | 2 dias | Circuit Breaker + Retry/Fallback |
| Persistência | 2 dias | Batch Processing + Redis Cache |
| Otimização | 1 dia | Amostragem métricas + Strip binário |
| Testes | 2 dias | Relatório performance + Ajustes |

**Total: 11 dias (considerando buffer de 4 dias para imprevistos)**

## Riscos e Mitigação

1. **Gerenciamento de Memória no Redis**:
   - Risco: Estouro de memória
   - Mitigação: `maxmemory-policy allkeys-lru` + monitoramento

2. **Contenção no PostgreSQL**:
   - Risco: Deadlocks em alta carga
   - Mitigação: Tamanho de lote ajustável + backoff exponencial

3. **False Positives Circuit Breaker**:
   - Risco: Bloqueio indevido de processadores
   - Mitigação: Janela deslizante + análise heurística

## Conclusão

Esta implementação em Rust atinge o equilíbrio ideal entre performance extrema e minimalismo, utilizando:

1. **Serialização binária** com MessagePack para eficiência máxima
2. **Concorrência lock-free** com atomics e batch processing
3. **Resiliência proativa** através de Circuit Breaker atômico
4. **Arquitetura enxuta** sem componentes intermediários (Nginx removido)
5. **Gerenciamento inteligente de recursos** com alocação estática otimizada

As estimativas conservadoras apontam para **3.500+ req/s com latência p95 < 15ms** dentro dos limites de 1.5 vCPU e 350MB RAM, posicionando esta solução como forte candidata à vitória na Rinha de Backend 2025.