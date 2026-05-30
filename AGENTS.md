# Repository Agent Guide

## Source Of Truth Priority

1. User instructions in the active task.
2. This `AGENTS.md`.
3. `docs/INDEX.md` and linked ADR, module, and roadmap files.
4. Existing code and tests.

`.ai/` is local-only context and is ignored by git.

When sources conflict, stop and surface the conflict before editing.

## Status Vocabularies

ADR status values: `proposed`, `accepted`, `deprecated`, `superseded`.

Roadmap and evolution status values: `proposed`, `active`, `deferred`, `obsolete`, `rejected`, `completed`.

Do not mix these vocabularies.

## Work Modes

DO without asking for reversible local work: read files, edit local files, add tests, run read-only commands, and run local validation.

THINK before coding for Rust edition, dependency, API, or architecture changes. State assumptions, tradeoffs, and simpler alternatives before implementation.

REQUIRE APPROVAL before irreversible or externally visible work: force push, branch deletion, production deployment, destructive cleanup, migrations, or external messages.

Push back explicitly when a plan has a real technical flaw, there is a materially simpler path, or work is about to touch production.

## Module Boundaries

- Client module: `src/clt.rs`.
- Server module: `src/svr.rs`.
- Protocol module: `src/proto.rs`, `proto/internal.proto`, `build.rs`.

Do not add application business logic, CLI code, or deployment configuration to this crate without a new ADR or an accepted roadmap item.

## Before Adding A New Component

- Prefer extending the existing module file when the responsibility fits.
- Add a new component only when it has a distinct stable responsibility.
- Add or update the module doc under `docs/modules/` when a new component changes ownership boundaries.

## Code Traceability

Every non-trivial change should map to:

- An ADR for boundary or strategy changes.
- A roadmap item for planned work.
- A module doc for ownership and scope.

Keep edits surgical. Do not reformat or refactor adjacent code unless required by the requested change.

## Tooling Preferences

- Prefix shell commands with `rtk`.
- Use `cargo` through `rtk cargo ...`.
- Keep Rust comments and documentation in English unless the surrounding file clearly uses another language.
- Do not add AI-generated signatures to git commits.

## Validation And Completion

Report what changed, what validation ran, what did not run, and any remaining assumptions or risks.

Do not claim a command passed unless it was actually run and succeeded.
