# Module: Server

## FOR

- Register unary KRPC handlers.
- Dispatch incoming tonic requests by path.
- Provide server-side helper macros for small service modules.

## NOT FOR

- Application business logic.
- Runtime service discovery.
- Production process supervision.

## Components

- `src/svr.rs`
- `examples/demo-server.rs`
- `examples/demo/hello.rs`

## Evolution

### Active

- Keep the existing route-map and macro surface compiling on Rust 2024 — Status: active
- Keep demo server integration smoke test runnable from examples — Status: active

### Deferred / Obsolete

- Replace macro-based registration with generated service metadata — Status: deferred — not required for the current upgrade.
