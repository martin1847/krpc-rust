# Module: Client

## FOR

- Connect to HTTP or HTTPS tonic endpoints.
- Build `InputProto` requests from JSON strings.
- Issue unary calls to KRPC method paths.

## NOT FOR

- CLI argument parsing.
- Request serialization formats beyond the current protobuf envelope.
- Long-lived application retry policy.

## Components

- `src/clt.rs`

## Evolution

### Active

- Prepare for Rust 2024 and MSRV 1.95.0 dependency upgrades — Status: active

### Deferred / Obsolete

- Add higher-level client ergonomics — Status: deferred — keep the upgrade scoped first.
