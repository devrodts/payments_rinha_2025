# Rinha de Backend 2025 - Rust Implementation

## T1.1: Setup Inicial do Projeto Rust
**Arquivos-Chave:** 
- `Cargo.toml`: Configuração de dependências e otimizações de compilação ([LOC:25])
- `src/main.rs`: Ponto de entrada da aplicação ([LOC:3])
- `tests/t1_1_setup_test.rs`: Testes de validação do setup ([LOC:25])

## T1.2: Configuração do Servidor Axum
**Arquivos-Chave:** 
- `src/main.rs`: Servidor Axum com endpoints ([LOC:35])
- `src/modules/config/mod.rs`: Módulo de configuração ([LOC:25])
- `tests/t1_2_server_test.rs`: Testes do servidor ([LOC:30])
- `tests/integration_test.rs`: Testes de integração ([LOC:35])

**Erros Resolvidos:** 
- [Falha: Servidor não responde na porta 9999](artifacts/failures.md#failt122025-01-27t102500z)

**Como Usar:**
```bash
# Iniciar servidor
cargo run

# Testar endpoints
curl http://localhost:9999/
curl http://localhost:9999/health

# Executar testes
cargo test
```

**Endpoints Implementados:**
- `GET /` - Página inicial
- `GET /health` - Health check (retorna JSON com status)

**Erros Resolvidos:** 
- [Falha: Dependências não encontradas](artifacts/failures.md#failt112025-01-27t101500z)

**Como Usar:**
```bash
# Compilar o projeto
cargo build

# Executar testes
cargo test

# Executar em modo release (otimizado)
cargo build --release
```

**Dependências Principais:**
- `axum = "0.7"` - Framework web assíncrono
- `tokio = "1.0"` - Runtime assíncrono
- `serde = "1.0"` - Serialização/deserialização
- `rmp-serde = "1.1"` - Serialização MessagePack
- `sqlx = "0.7"` - ORM assíncrono para PostgreSQL
- `redis = "0.24"` - Cliente Redis
- `validator = "0.16"` - Validação de dados
- `zeroize = "1.7"` - Zeroização segura de memória

**Otimizações de Compilação:**
- `panic = "abort"` - Reduz tamanho do binário
- `lto = "thin"` - Link Time Optimization
- `codegen-units = 1` - Otimizações mais agressivas
- `strip = true` - Remove símbolos de debug

---

## Status do Projeto
- [x] T1.1: Setup inicial do projeto Rust
- [ ] T1.2: Configuração do servidor Axum
- [ ] T1.3: Configuração Docker básica
- [ ] T2.1: POST /payments endpoint
- [ ] T2.2: Serialização MessagePack
- [ ] T2.3: Integração com Payment Processors
- [ ] T3.1: Circuit Breaker atômico lock-free
- [ ] T3.2: Estratégia de retry com backoff exponencial
- [ ] T3.3: Health check cache
- [ ] T4.1: Batch processing PostgreSQL
- [ ] T4.2: Cache Redis reativo
- [ ] T5.1: GET /payments-summary endpoint
- [ ] T6.1: Otimizações de compilação
- [ ] T6.2: Alocadores de memória customizados
- [ ] T6.3: Métricas com amostragem adaptativa
- [ ] T7.1: Validação de inputs
- [ ] T7.2: Zeroização de memória
- [ ] T7.3: Gerenciamento de segredos
- [ ] T8.1: Testes unitários TDD
- [ ] T8.2: Testes de performance
- [ ] T8.3: Testes de resiliência
- [ ] T9.1: Dockerfile otimizado
- [ ] T9.2: docker-compose.yml completo
- [ ] T9.3: Métricas Prometheus

## Arquitetura
```
rinha/
├── src/
│   ├── main.rs                    # Ponto de entrada
│   └── modules/                   # Módulos organizados
│       ├── auth/                  # Autenticação
│       ├── payment/               # Processamento de pagamentos
│       ├── health/                # Health checks
│       ├── circuit_breaker/       # Circuit breaker lock-free
│       ├── db/                    # Persistência PostgreSQL
│       ├── cache/                 # Cache Redis
│       ├── processors/            # Integração com processadores
│       ├── models/                # Modelos de dados
│       └── config/                # Configuração
├── tests/                         # Testes TDD
├── artifacts/                     # Documentação de desenvolvimento
│   ├── todo.md                    # Checklist mestre
│   ├── failures.md                # Erros brutos
│   ├── fixed.md                   # Correções implementadas
│   └── commits.md                 # Histórico de commits
└── README.md                      # Documentação principal
```

## Critérios de Aceitação
- [x] Projeto compila sem erros
- [x] Todas as dependências necessárias configuradas
- [x] Otimizações de compilação aplicadas
- [ ] Servidor responde na porta 9999
- [ ] Limites de recursos respeitados (1.5 vCPU, 350MB RAM)
- [ ] Performance >3500 req/s com p95 < 15ms
- [ ] Resiliência a 40% de falhas nos processadores
- [ ] Consistência com Payment Processors
- [ ] Segurança PCI-DSS compliant
- [ ] Zero hardcoded secrets
- [ ] Cobertura de testes ≥ 80% 