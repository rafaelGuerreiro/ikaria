---
name: ikaria-rust-workspace
description: Use for Rust workspace changes that touch crate layout, shared dependencies, and root-level validation commands in Ikaria.
---

# Ikaria Rust Workspace Workflow

Use this skill when work spans multiple crates or changes repository structure.

## Workspace conventions

- Crates are organized as:
  - `bins/*` for executables
  - `sdks/*` for reusable libraries
- Centralize dependency versions in root `Cargo.toml` under `[workspace.dependencies]`.
- Child crates should consume shared dependencies with `workspace = true`.
- Shared contracts used by both server/client should live in `sdks/shared`.

## Standard validation flow (from repo root)

1. `task fmt`
2. `task clippy`
3. `task check`
4. `task test`

For server-impacting changes, also run:

- `task test-server`
- `task build` (wasm + host split build)

## Change checklist

- Keep edits minimal and scoped.
- Avoid direct binary-to-binary dependencies.
- Prefer adding reusable logic in `sdks/*` rather than duplicating across bins.
