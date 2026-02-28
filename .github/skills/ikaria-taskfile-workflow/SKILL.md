---
name: ikaria-taskfile-workflow
description: Use for task automation updates and command execution strategy in Ikaria, especially dependency chaining and root-first task execution.
---

# Ikaria Taskfile Workflow

Use this skill when adding/changing automation or deciding how to validate changes.

## Root-first task usage

Run tasks from repository root unless a task is explicitly scoped:

- `task check` (server:check + client:lint)
- `task test` (check + server:test)
- `task build` (server wasm + client build)

Server-only tasks (via `server/Taskfile.yml`):

- `task server:fmt`
- `task server:check` (deps: fmt → clippy → cargo check)
- `task server:test` (deps: check)
- `task server:build` (sdk-ts + wasm targets)

Client-only tasks (via `client/Taskfile.yml`):

- `task client:lint`
- `task client:build`

## Dependency policy

- Server: `check` depends on `fmt`, which runs `cargo fmt`. Then clippy runs before cargo check.
- Client: `lint` runs eslint.
- Root `test` depends on `check` to guarantee formatting and linting run first.

## Task design patterns

- Keep tasks small and single-purpose.
- Prefer explicit `deps` over repeated command blocks.
- Root Taskfile includes server and client via `includes:`.
