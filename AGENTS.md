# krpc-rust — Agent Guide

Rust runtime of the KRPC protocol: async server + client over the same
gRPC/HTTP-2 wire as the Java reference implementation. Capability cluster:
umbrella `docs/modules/polyglot-runtimes.md`.

## Scope

- FOR: a language-idiomatic Rust server/client runtime wire-compatible with
  `krpc` — server in `src/svr.rs`, client in `src/clt.rs`, protocol layer in
  `src/proto.rs` + `proto/internal.proto` + `build.rs`.
- NOT FOR: defining protocol/contract semantics (those follow `krpc`);
  application business logic, CLI tooling (`rpcurl` lives in `krpc-crates`), or
  deployment configuration.
- Maintained-only: work happens on concrete pressure (a real bug / a real
  consumer need), not speculative feature parity.

## Ecosystem rules

- Wire compatibility originates in `krpc`; this repo follows the wire, never
  forks the protocol, never leads a wire change (NS-2).
- Tier-2 maintained: touch only on a concrete need, not for speculative parity
  (NS-8).
- Long-term direction: the KRPC umbrella workspace `docs/NORTH_STAR.md` (cite
  principles by NS-ID when relevant).
- Repo-internal SoT: `docs/INDEX.md` and the linked ADR / module / roadmap
  files. ADR status (`proposed`/`accepted`/`deprecated`/`superseded`) and
  roadmap status (`proposed`/`active`/`deferred`/`obsolete`/`rejected`/
  `completed`) are separate vocabularies — never mix them.
- `.ai/` is local-only context and is ignored by git.

## Build & test

- Prefix shell commands with `rtk`; use `cargo` through `rtk cargo ...`.
- `rtk cargo build` (not verified).
- `rtk cargo test` (not verified).
- Keep Rust comments and documentation in English unless the surrounding file
  clearly uses another language.

## Discipline

- Behavior changes hide behind a flag, default OFF. No drive-by refactors or
  format churn; keep edits surgical.
- Boundary/strategy changes need an ADR; planned work maps to a roadmap item;
  ownership changes update the module doc under `docs/modules/`.
- Secrets, internal hostnames/IPs, topology never enter the committed tree,
  logs, or docs.
- Commits stay local until the owner approves a push. No AI signature lines.
- Report what changed, what validation ran, what did not run, and remaining
  risks; never claim a command passed unless it actually ran.
