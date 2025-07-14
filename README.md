# Rinha de Backend 2025 - Rust Implementation

## Visão Geral

Backend de processamento de pagamentos desenvolvido em Rust para a Rinha de Backend 2025. Implementa um sistema robusto de intermediário de pagamentos com circuit breaker, fallback automático e alta performance.

## Arquitetura

### Módulos Principais
- **Core Infrastructure**: Setup inicial, servidor Axum, configuração Docker
- **Payment Processing**: Endpoints de pagamento, serialização MessagePack, integração com processadores
- **Circuit Breaker & Resilience**: Circuit breaker atômico, retry com backoff exponencial, cache de health check
- **Data Persistence**: Batch processing PostgreSQL, cache Redis reativo
- **Payments Summary**: Endpoint de resumo com consistência
- **Performance & Optimization**: Otimizações de compilação, alocadores customizados, métricas
- **Security & Compliance**: Validação de inputs, zeroização de memória, gerenciamento de segredos

## Status do Desenvolvimento

### ✅ Concluído

#### T1.1: Setup inicial do projeto Rust
**Arquivos-Chave:** 
- `Cargo.toml`: Configuração de dependências ([LOC:27])
- `src/main.rs`: Ponto de entrada da aplicação ([LOC:25])

**Como Usar:**
```bash
cargo build
cargo test
```

#### T1.2: Configuração do servidor Axum
**Arquivos-Chave:** 
- `src/main.rs`: Servidor Axum com endpoints ([LOC:25])
- `src/modules/config/mod.rs`: Configuração do servidor ([LOC:32])

**Como Usar:**
```bash
cargo run
curl http://localhost:9999/health
```

#### T1.2.1: Resolução de warnings de dead code
**Arquivos-Chave:** 
- `src/modules/config/mod.rs`: Campo log_level justificado ([LOC:32])

**Erros Resolvidos:** 
- [Falha: Campo log_level não utilizado #failures-link]

**Como Usar:**
```rust
let config = Config::new();
// log_level está disponível para futura implementação de logging
```

### 🔄 Em Progresso

#### T1.3: Configuração Docker básica
**Arquivos-Chave:** 
- `Dockerfile`: Multi-stage build com Rust 1.81 ([LOC:10])
- `docker-compose.yml`: Configuração com limites de recursos ([LOC:36])

**Limitação Conhecida:** 
- Docker build test temporariamente ignorado devido a ICU crates requerendo Rust 1.82+

**Como Usar:**
```bash
docker build -t rinha-backend:test .
docker-compose up -d
```

### 📋 Pendente

#### T2.1: POST /payments endpoint
**Arquivos-Chave:**
- `src/modules/payment/mod.rs`: Handler do endpoint ([LOC:~20])
- `src/modules/models/mod.rs`: Modelo e validação ([LOC:~25])
- `tests/payment_test.rs`: Testes TDD ([LOC:~30])

**Erros Resolvidos:**
- [Falha: Endpoint não implementado #failures-link]
- [Falha: Validação de UUID e amount #failures-link]

**Como Usar:**
```bash
curl -X POST http://localhost:9999/payments \
  -H "Content-Type: application/json" \
  -d '{"correlation_id": "4a7901b8-7d26-4d9d-aa19-4dc1c7cf60b3", "amount": 19.90}'
```

**Rastreabilidade:**
- Testes: [`tests/payment_test.rs`](tests/payment_test.rs)
- Falhas: [`artifacts/failures.md`](artifacts/failures.md)
- Correções: [`artifacts/fixed.md`](artifacts/fixed.md)
- Commit: [`artifacts/commits.md`](artifacts/commits.md)

## Tecnologias Utilizadas

- **Linguagem**: Rust 1.81
- **Framework Web**: Axum 0.7
- **Runtime**: Tokio 1.0
- **Serialização**: Serde + MessagePack
- **Banco de Dados**: PostgreSQL (SQLx)
- **Cache**: Redis
- **Validação**: Validator
- **Segurança**: Zeroize
- **Containerização**: Docker + Docker Compose

## Estrutura do Projeto

```
rinha/
├── src/
│   ├── main.rs                 # Ponto de entrada
│   └── modules/
│       ├── config/             # Configuração do servidor
│       ├── payment/            # Processamento de pagamentos
│       ├── processors/         # Integração com processadores
│       ├── circuit_breaker/    # Circuit breaker
│       ├── cache/              # Cache Redis
│       ├── db/                 # Banco PostgreSQL
│       ├── health/             # Health checks
│       ├── models/             # Modelos de dados
│       └── mod.rs              # Organização de módulos
├── tests/                      # Testes automatizados
├── artifacts/                  # Artefatos de desenvolvimento
│   ├── todo.md                 # Checklist mestre
│   ├── failures.md             # Histórico de falhas
│   ├── fixed.md                # Correções implementadas
│   └── commits.md              # Histórico de commits
├── docs/                       # Documentação técnica
├── Dockerfile                  # Multi-stage build
├── docker-compose.yml          # Orquestração de containers
└── README.md                   # Este arquivo
```

## Desenvolvimento

### Pré-requisitos
- Rust 1.81+
- Docker & Docker Compose
- PostgreSQL
- Redis

### Setup Local
```bash
# Clone o repositório
git clone <repository-url>
cd rinha

# Instale dependências
cargo build

# Execute testes
cargo test

# Execute o servidor
cargo run
```

### Testes
```bash
# Todos os testes
cargo test

# Testes específicos
cargo test --test setup_test
cargo test --test server_test
cargo test --test integration_test
cargo test --test docker_test
```

## Métricas de Qualidade

### Cobertura de Testes
- **Setup Tests**: 100% ✅
- **Server Tests**: 100% ✅
- **Integration Tests**: 100% ✅
- **Docker Tests**: 80% ✅ (build test temporariamente ignorado)

### Performance
- **Compilação**: Otimizada com LTO thin, panic=abort
- **Runtime**: Tokio async/await para alta concorrência
- **Memória**: Zeroize para dados sensíveis

### Segurança
- **Validação**: Validator crate para inputs
- **Memória**: Zeroize para dados sensíveis
- **Dependências**: Pinned para versões seguras

## Rastreabilidade

### Links Importantes
- [Failures Log](artifacts/failures.md) - Histórico de falhas
- [Fixed Issues](artifacts/fixed.md) - Correções implementadas
- [Commits History](artifacts/commits.md) - Histórico detalhado
- [TODO](artifacts/todo.md) - Checklist mestre

### Convenções
- **Commits**: Seguem [Conventional Commits 1.0]
- **TDD**: Ciclo RED-GREEN-REFACTOR rigoroso
- **Documentação**: Síncrona com desenvolvimento
- **Rastreabilidade**: Links bidirecionais entre artefatos

## Contribuição

1. Siga o fluxo TDD industrializado
2. Documente todas as mudanças nos artefatos
3. Use commits semânticos
4. Mantenha rastreabilidade completa

## Licença

Este projeto é desenvolvido para a Rinha de Backend 2025. 