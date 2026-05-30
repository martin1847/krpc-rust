# Documentation Index

This index maps the repository source of truth.

## Decisions

- [ADR-0001: Repository Scope And Boundary](decisions/ADR-0001-repository-scope.md)

## Modules

- [Client](modules/client.md)
- [Server](modules/server.md)
- [Protocol](modules/protocol.md)

## Roadmap

- [Active Roadmap](roadmap/active-roadmap.md)

## Local AI Context

`.ai/` is intentionally local-only and ignored by git.

## Traceability

| Capability | Component | ADR | Roadmap |
| --- | --- | --- | --- |
| Client-side unary KRPC calls | `src/clt.rs` | ADR-0001 | R-2024-001 |
| Server-side route dispatch | `src/svr.rs` | ADR-0001 | R-2024-001 |
| Shared protobuf protocol types | `src/proto.rs`, `proto/internal.proto` | ADR-0001 | R-2024-001 |
| Demo server integration smoke test | `examples/demo-server.rs`, `examples/demo/hello.rs`, `tests/integration_test.rs` | ADR-0001 | R-2024-002 |
