# Active Roadmap

## R-2024-001: Rust 2024 And MSRV 1.95.0 Upgrade

Status: active

Capability: Maintain the KRPC library on the Rust 2024 baseline.

Components:

- `Cargo.toml`
- `Cargo.lock`
- `src/clt.rs`
- `src/svr.rs`
- `src/proto.rs`

ADR: ADR-0001

Acceptance Criteria:

- `Cargo.toml` declares edition 2024.
- `Cargo.toml` declares Rust MSRV 1.95.0.
- Dependency versions are updated within the selected tonic/prost compatibility line.
- `cargo check --all-targets --all-features` passes with required compile-time environment.
- Any unsupported target-specific validation is recorded explicitly.

## R-2024-002: Demo Server Integration Test

Status: active

Capability: Validate client and server interoperability against a local demo server.

Components:

- `examples/demo-server.rs`
- `examples/demo/hello.rs`
- `tests/integration_test.rs`

ADR: ADR-0001

Acceptance Criteria:

- Start the demo server with `KRPC_APP_NAME=demo-server KRPC_BIND=0.0.0.0:50051 cargo run --example demo-server --features svr`.
- Run integration tests with `KRPC_APP_NAME=test-server cargo test --test integration_test --all-features`.
- `cargo package --list` does not include the demo server example files.
