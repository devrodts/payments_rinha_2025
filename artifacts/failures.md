# Failures - Rinha de Backend 2025

## Histórico de Falhas TDD

### [FAIL][T1.1][2025-01-27T10:00:00Z]
| Arquivo       | src/main.rs:1 |
| Erro          | Expected compilation success, got compilation error |
| Código-Teste  | cargo build |
| Contexto      | Setup inicial do projeto Rust |

### [FAIL][T1.1][2025-01-27T10:15:00Z]
| Arquivo       | src/main.rs:1 |
| Erro          | unresolved import `axum` |
| Código-Teste  | use axum::Router; |
| Contexto      | Dependência axum não encontrada no Cargo.toml |

### [FAIL][T1.2][2025-01-27T10:05:00Z]
| Arquivo       | src/main.rs:1 |
| Erro          | Expected server on port 9999, got no server |
| Código-Teste  | curl http://localhost:9999/health |
| Contexto      | Configuração do servidor Axum |

### [FAIL][T1.2][2025-01-27T10:25:00Z]
| Arquivo       | tests/t1_2_server_test.rs:13 |
| Erro          | Server should respond on port 9999 |
| Código-Teste  | reqwest::get("http://localhost:9999/health").await |
| Contexto      | Servidor Axum não implementado |

---

## Padrão de Registro
```
[FAIL][TaskID][ISO_TIMESTAMP]
| Arquivo       | arquivo.rs:linha |
| Erro          | Descrição do erro |
| Código-Teste  | Comando ou código que falhou |
| Contexto      | Contexto da falha |
``` 