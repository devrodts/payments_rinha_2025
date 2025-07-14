# TODO - Rinha de Backend 2025

## [Módulo] Core Infrastructure
### Critérios Gerais
- [x] T1.1: Setup inicial do projeto Rust
  - [Critério] Projeto deve compilar sem erros
  - [Arquivos] Cargo.toml, src/main.rs
  - [Status] [x] Concluído | [2025-01-27T10:20:00Z]

- [x] T1.2: Configuração do servidor Axum
  - [Critério] Servidor deve responder na porta 9999
  - [Arquivos] src/main.rs, src/modules/config/mod.rs
  - [Status] [x] Concluído | [2025-01-27T10:30:00Z]

- [x] T1.2.1: Resolução de warnings de dead code
  - [Critério] Campo log_level justificado e silenciado com #[allow(dead_code)]
  - [Critério] Documentação inline com TODO para implementação futura
  - [Arquivos] src/modules/config/mod.rs
  - [Status] [x] Concluído | [2025-01-27T17:05:00Z]
  - [Nota] Campo mantido para futura implementação de sistema de logging (T2.x)

- [ ] T1.3: Configuração Docker básica
  - [Critério] docker-compose.yml deve definir limites de recursos
  - [Arquivos] docker-compose.yml, Dockerfile
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Payment Processing
### Critérios Gerais
- [x] T2.1: POST /payments endpoint
  - [Critério] Deve aceitar correlationId (UUID) e amount (decimal)
  - [Critério] Deve retornar 2XX para requisições válidas
  - [Critério] Deve validar inputs obrigatórios
  - [Arquivos] src/modules/payment/mod.rs, src/modules/models/mod.rs, tests/payment_test.rs
  - [Status] [x] Concluído | [2025-01-27T18:10:00Z]
  - [Rastreabilidade] Testes: payment_test.rs | Falhas: failures.md | Correções: fixed.md | Commit: commits.md

- [ ] T2.2: Serialização MessagePack
  - [Critério] Payload deve ser 65% menor que JSON
  - [Critério] CPU serialização deve ser 40% menor
  - [Arquivos] src/modules/models/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T2.3: Integração com Payment Processors
  - [Critério] Deve chamar processador default primeiro
  - [Critério] Deve implementar fallback para processador secundário
  - [Critério] Deve adicionar campo requestedAt (ISO timestamp)
  - [Arquivos] src/modules/processors/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Circuit Breaker & Resilience
### Critérios Gerais
- [ ] T3.1: Circuit Breaker atômico lock-free
  - [Critério] Deve usar AtomicU32 para estado
  - [Critério] Deve ter 3 estados: fechado, aberto, meio-aberto
  - [Critério] Deve abrir após limite de falhas
  - [Arquivos] src/modules/circuit_breaker/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T3.2: Estratégia de retry com backoff exponencial
  - [Critério] Máximo 3 tentativas por pagamento
  - [Critério] Backoff: 100ms, 200ms, 400ms
  - [Critério] Deve fallback para processador secundário
  - [Arquivos] src/modules/payment/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T3.3: Health check cache
  - [Critério] Deve cachear status por 300s
  - [Critério] Deve respeitar limite de 1 chamada/5s
  - [Arquivos] src/modules/health/mod.rs, src/modules/cache/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Data Persistence
### Critérios Gerais
- [ ] T4.1: Batch processing PostgreSQL
  - [Critério] Lote de 100 transações
  - [Critério] Timeout de 10ms para descarga
  - [Critério] Deve usar tokio::select! para não-bloqueante
  - [Arquivos] src/modules/db/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T4.2: Cache Redis reativo
  - [Critério] Limite de 50MB de memória
  - [Critério] Política allkeys-lru
  - [Critério] TTL de 300s para health checks
  - [Arquivos] src/modules/cache/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Payments Summary
### Critérios Gerais
- [ ] T5.1: GET /payments-summary endpoint
  - [Critério] Deve aceitar parâmetros from/to (ISO timestamp)
  - [Critério] Deve retornar totalRequests e totalAmount para default/fallback
  - [Critério] Deve ser consistente com Payment Processors
  - [Arquivos] src/modules/payment/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Performance & Optimization
### Critérios Gerais
- [ ] T6.1: Otimizações de compilação
  - [Critério] panic=abort no release
  - [Critério] LTO thin habilitado
  - [Critério] codegen-units=1
  - [Critério] strip=true
  - [Arquivos] Cargo.toml, Dockerfile
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T6.2: Alocadores de memória customizados
  - [Critério] Deve usar mimalloc ou jemalloc
  - [Critério] Deve reduzir RSS em 50%
  - [Arquivos] Cargo.toml, src/main.rs
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T6.3: Métricas com amostragem adaptativa
  - [Critério] Taxa base de 10% amostragem
  - [Critério] Deve reduzir CPU em 40%
  - [Arquivos] src/modules/metrics/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Security & Compliance
### Critérios Gerais
- [ ] T7.1: Validação de inputs
  - [Critério] Deve usar crate validator
  - [Critério] Deve prevenir OWASP Top 10
  - [Arquivos] src/modules/models/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T7.2: Zeroização de memória
  - [Critério] Deve usar crate zeroize
  - [Critério] Deve zeroizar dados sensíveis
  - [Arquivos] src/modules/models/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T7.3: Gerenciamento de segredos
  - [Critério] Zero hardcoded secrets
  - [Critério] Deve usar HashiCorp Vault pattern
  - [Arquivos] src/modules/config/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Testing & Validation
### Critérios Gerais
- [ ] T8.1: Testes unitários TDD
  - [Critério] 100% cobertura para novos códigos
  - [Critério] Deve seguir ciclo RED-GREEN-REFACTOR
  - [Arquivos] tests/
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T8.2: Testes de performance
  - [Critério] Deve atingir >3500 req/s
  - [Critério] p95 < 15ms
  - [Critério] Deve testar com 40% falhas
  - [Arquivos] tests/performance/
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T8.3: Testes de resiliência
  - [Critério] Deve testar Circuit Breaker
  - [Critério] Deve testar fallback automático
  - [Critério] Deve testar batch processing
  - [Arquivos] tests/resilience/
  - [Status] [ ] Pendente | [x] Concluído

## [Módulo] Deployment & Monitoring
### Critérios Gerais
- [ ] T9.1: Dockerfile otimizado
  - [Critério] Multi-stage build
  - [Critério] Imagem final < 15MB
  - [Critério] Deve usar debian:bookworm-slim
  - [Arquivos] Dockerfile
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T9.2: docker-compose.yml completo
  - [Critério] Limites: 1.5 vCPU, 350MB RAM total
  - [Critério] Rede payment-processor
  - [Critério] 3 réplicas do app
  - [Arquivos] docker-compose.yml
  - [Status] [ ] Pendente | [x] Concluído

- [ ] T9.3: Métricas Prometheus
  - [Critério] Endpoint /metrics
  - [Critério] Métricas de throughput e latência
  - [Critério] Métricas de Circuit Breaker
  - [Arquivos] src/modules/metrics/mod.rs
  - [Status] [ ] Pendente | [x] Concluído

---

## Critérios de Aceitação Gerais
- [ ] Deve respeitar limite de 1.5 vCPU e 350MB RAM
- [ ] Deve atingir >3500 req/s com p95 < 15ms
- [ ] Deve ser resiliente a 40% de falhas nos processadores
- [ ] Deve ser consistente com Payment Processors
- [ ] Deve ser seguro (PCI-DSS compliant)
- [ ] Deve ter zero hardcoded secrets
- [ ] Deve ter cobertura de testes ≥ 80% 