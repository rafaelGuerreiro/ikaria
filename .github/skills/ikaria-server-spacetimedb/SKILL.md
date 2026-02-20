---
name: ikariadb-spacetimedb
description: Use for SpacetimeDB backend work in bins/server, including reducer/service architecture, deterministic state updates, and server validation.
---

# Ikaria Server (SpacetimeDB) Workflow

Use this skill for backend implementation under `bins/server`.

## Architecture rules

- Keep reducers thin: validate input, enforce access requirements, delegate to services.
- Keep business logic in domain services.
- Pass `Identity`/actor context into service methods explicitly.
- Prefer private tables and expose client-facing reads through views.
- Keep state transitions deterministic; use `ctx.timestamp` and context RNG, not system time/random.

## Repository-specific constraints

- Reuse shared protocol/domain types from `ikaria-shared` (`sdks/shared`) where boundaries are crossed.
- Add dependencies only in root `[workspace.dependencies]`, then consume with `workspace = true`.
- Follow server guidance in `bins/server/AGENTS.md`.

## Validation

- `task test-server`
- `task check`
- `task build` when changing server runtime/module behavior
