# Module: Protocol

## FOR

- Define the KRPC protobuf request and response envelope used by this crate.
- Keep generated Rust message types available without requiring protoc during normal builds.

## NOT FOR

- Owning the upstream cross-language KRPC protocol.
- Encoding application-specific schemas inside this crate.

## Components

- `proto/internal.proto`
- `src/proto.rs`
- `build.rs`

## Evolution

### Active

- Keep generated protocol types compatible with the selected prost and tonic line — Status: active

### Deferred / Obsolete

- Re-enable build-time protobuf generation — Status: deferred — current builds use checked-in generated code.
