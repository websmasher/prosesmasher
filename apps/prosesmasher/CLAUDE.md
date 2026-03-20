# prosesmasher

Agent-managed Rust CLI project with hexagonal architecture.

## Architecture

```
apps/prosesmasher/crates/
├── domain/types/           — core types, business rules
├── ports/
│   ├── inbound/            — inbound port interfaces (empty)
│   └── outbound/traits/    — outbound port trait definitions
├── app/core/               — use cases, orchestration
└── adapters/
    ├── inbound/cli/        — CLI binary (composition root)
    └── outbound/fs/        — filesystem adapter
```

## Guardrails

Run `guardrail3 rs validate .` to check compliance.
