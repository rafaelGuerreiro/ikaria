---
name: ikaria-taskfile-workflow
description: Use for task automation updates and command execution strategy in Ikaria, especially dependency chaining and root-first task execution.
---

# Ikaria Taskfile Workflow

Use this skill when adding/changing automation or deciding how to validate changes.

## Root-first task usage

Run tasks from repository root unless a task is explicitly server-local:

- `task fmt`
- `task clippy`
- `task check`
- `task test`
- `task build`
- `task test-server`
- `task test-client`

## Dependency policy

- All non-format tasks should depend on `clippy`.
- `clippy` should depend on `fmt`.
- This guarantees `task test`, `task build`, and other task entrypoints run formatting and linting first.

## Task design patterns

- Keep tasks small and single-purpose.
- Prefer explicit `deps` over repeated command blocks.
- Keep Rust workspace commands at root unless there is a strong server-only reason.
