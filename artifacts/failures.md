# Failures Log - Rinha de Backend 2025

## Histórico de Falhas (Imutável)

### [FAIL][T1.3][2025-01-27T16:30:00Z]
| Arquivo       | tests/docker_test.rs:48 |
| Erro          | Docker build should succeed. Exit code: exit status: 101 |
| Código-Teste  | assert!(output.status.success(), "Docker build should succeed") |
| Causa         | ICU crates (icu_collections, icu_normalizer, etc.) require Rust 1.82+ |
| Impacto       | Docker build test fails, blocking T1.3 completion |
| Status        | TEMPORARILY_IGNORED |

### [FAIL][T1.2.1][2025-01-27T16:45:00Z]
| Arquivo       | src/modules/config/mod.rs:6 |
| Erro          | field `log_level` is never read |
| Código-Teste  | pub log_level: String |
| Causa         | Field declared but not used in current implementation |
| Impacto       | Compiler warnings, code quality degradation |
| Status        | FIXED |

### [FAIL][T1.3][2025-01-27T17:00:00Z]
| Arquivo       | Cargo.toml |
| Erro          | failed to resolve patches for crates.io-index |
| Código-Teste  | [patch.crates-io] icu_collections = "=1.4.0" |
| Causa         | Patch points to same source, Cargo limitation |
| Impacto       | Dependency resolution fails |
| Status        | FIXED |

### [FAIL][T1.3][2025-01-27T17:15:00Z]
| Arquivo       | Cargo.toml |
| Erro          | one of the features ['runtime-actix-native-tls', ...] must be enabled |
| Código-Teste  | sqlx = { version = "=0.6.3", features = ["postgres", "runtime-tokio"] } |
| Causa         | SQLx requires explicit runtime feature selection |
| Impacto       | Compilation fails |
| Status        | FIXED |

### [FAIL][T2.1][2025-01-27T18:00:00Z]
| Arquivo       | tests/payment_test.rs:10 |
| Erro          | Endpoint /payments deve ser implementado |
| Código-Teste  | assert!(false, "Endpoint /payments deve ser implementado") |
| Causa         | POST /payments endpoint not implemented |
| Impacto       | T2.1 task cannot be completed |
| Status        | PENDING |

### [FAIL][T2.1][2025-01-27T18:00:00Z]
| Arquivo       | tests/payment_test.rs:17 |
| Erro          | Validação de correlationId deve ser implementada |
| Código-Teste  | assert!(false, "Validação de correlationId deve ser implementada") |
| Causa         | UUID validation not implemented |
| Impacto       | Input validation missing |
| Status        | PENDING |

### [FAIL][T2.1][2025-01-27T18:00:00Z]
| Arquivo       | tests/payment_test.rs:24 |
| Erro          | Validação de amount deve ser implementada |
| Código-Teste  | assert!(false, "Validação de amount deve ser implementada") |
| Causa         | Amount validation not implemented |
| Impacto       | Input validation missing |
| Status        | PENDING |

### [FAIL][T2.3][2025-01-27T18:50:00Z]
| Arquivo       | tests/integration_payment_processor.rs:76 |
| Erro          | Default processor deve ser chamado |
| Código-Teste  | assert!(state.default_called, "Default processor deve ser chamado") |
| Causa         | Endpoint /payments não integra com Payment Processors |
| Impacto       | Fallback automático não implementado |
| Status        | PENDING |

---

## Padrão de Registro
```
[FAIL][TASK_ID][ISO_TIMESTAMP]
| Arquivo       | file:line |
| Erro          | Error description |
| Código-Teste  | Relevant code snippet |
| Causa         | Root cause analysis |
| Impacto       | Business/technical impact |
| Status        | PENDING|FIXED|IGNORED|TEMPORARILY_IGNORED |
``` 