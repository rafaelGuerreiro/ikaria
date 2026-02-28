---
name: ikariadb-spacetimedb
description: Use for SpacetimeDB backend work in server/, including reducer/service architecture, deterministic state updates, and server validation.
---

# Ikaria Server (SpacetimeDB) Workflow

Use this skill for backend implementation under `server/`.

## Architecture rules

- Keep reducers thin: validate input, enforce access requirements, delegate to services.
- Keep business logic in domain services.
- Pass `Identity`/actor context into service methods explicitly.
- Prefer private tables and expose client-facing reads through views.
- Keep state transitions deterministic; use `ctx.timestamp` and context RNG, not system time/random.

## Repository-specific constraints

- Reuse shared protocol/domain types from `ikaria-shared` (`server/sdks/shared`) where boundaries are crossed.
- Add dependencies only in `server/Cargo.toml` `[workspace.dependencies]`, then consume with `workspace = true`.
- Follow server guidance in `server/sdks/ikariadb-core/AGENTS.md`.

## Validation

- `task test` (from repo root — runs check + server tests)
- `task build` when changing server runtime/module behavior
