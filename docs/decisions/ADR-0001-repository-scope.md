# ADR-0001: Repository Scope And Boundary

Status: proposed

Date: 2026-05-30

## Context

This repository provides a Rust implementation of KRPC primitives used to expose native Rust work as gRPC-style unary endpoints. The current codebase is a single Cargo library crate with optional client and server features.

The repository needs a small source-of-truth structure before dependency, edition, and MSRV upgrades continue.

## Decision

This repository is for:

- A reusable KRPC Rust library crate.
- Client-side unary calls over tonic transport.
- Server-side route registration and dispatch helpers.
- Shared protobuf message definitions used by the Rust implementation.

This repository is not for:

- The `rpcurl` CLI implementation.
- Application-specific image processing or CPU-bound business modules.
- Production deployment configuration.
- A replacement for the upstream KRPC protocol repository.

## Consequences

Documentation and roadmap items should point back to this boundary. Changes that add a CLI, app modules, or deployment workflows need a new ADR or a repository split decision before implementation.
