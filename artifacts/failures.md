# Registro de Falhas - Rinha de Backend 2025

Este arquivo registra todas as falhas encontradas durante o desenvolvimento TDD, mantendo histórico imutável para rastreabilidade.

## Formato de Registro
```
[FAIL][Task-ID][ISO_TIMESTAMP]
| Arquivo       | caminho/para/arquivo:linha |
| Erro          | Descrição do erro |
| Código-Teste  | expect(response.status).toBe(200) |
| Contexto      | Informações adicionais |
```

---

## Falhas Registradas

### [INFO][T1.1][2025-01-27T21:00:00Z]
| Arquivo       | tests/setup_test.rs:39 |
| Erro          | Teste passou, mas vamos verificar estrutura completa |
| Código-Teste  | assert!(output.status.success()) |
| Contexto      | Projeto já está bem estruturado, mas precisamos verificar se atende todos os critérios |

### [FAIL][T2.3][2025-01-27T22:30:00Z]
| Arquivo       | tests/processor_selector_test.rs:66 |
| Erro          | Teste de fallback automático falhou - processadores não distinguíveis |
| Código-Teste  | assert!(result.is_ok()) |
| Contexto      | Mock server usando mesma URL para default e fallback processadores |

### [FAIL][T2.3][2025-01-27T22:35:00Z]
| Arquivo       | tests/processor_selector_test.rs:114 |
| Erro          | Teste de acesso concorrente falhou - HTTP calls reais |
| Código-Teste  | assert!(result.is_ok()) |
| Contexto      | Processador tentando fazer chamadas HTTP reais em ambiente de teste |

### [FAIL][T13.1][2025-01-27T23:00:00Z]
| Arquivo       | tests/setup_test.rs:88 |
| Erro          | Found 3 warnings in compilation - dead code warnings |
| Código-Teste  | assert_eq!(warnings.len(), 0) |
| Contexto      | Módulos processors/selector.rs, health/service.rs, cache/redis.rs têm funções não utilizadas |

---

## Legenda
- **[FAIL]**: Falha em teste
- **[ERROR]**: Erro de compilação
- **[WARNING]**: Aviso que pode indicar problema
- **[PERF]**: Problema de performance
- **[SEC]**: Problema de segurança
- **[INFO]**: Informação importante 