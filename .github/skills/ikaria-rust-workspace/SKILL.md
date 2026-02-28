---
name: ikaria-rust-workspace
description: Use for Rust workspace changes that touch crate layout, shared dependencies, and root-level validation commands in Ikaria.
---

# Ikaria Rust Workspace Workflow

Use this skill when work spans multiple crates or changes repository structure.

## Workspace conventions

- The Rust workspace lives under `server/`.
- Crates are organized as:
  - `server/bins/*` for executables (world-alpha-ikariadb, world-draconis-ikariadb)
  - `server/sdks/*` for reusable libraries (ikariadb-core, shared)
- Centralize dependency versions in `server/Cargo.toml` under `[workspace.dependencies]`.
- Child crates should consume shared dependencies with `workspace = true`.
- Shared contracts used by both server and client should live in `server/sdks/shared`.

## Standard validation flow (from repo root)

1. `task check` (runs server:check + client:lint)
2. `task test` (runs check + server:test)
3. `task build` (builds server wasm targets + client)

## Change checklist

- Keep edits minimal and scoped.
- Avoid direct binary-to-binary dependencies.
- Prefer adding reusable logic in `server/sdks/*` rather than duplicating across bins.
- For multi-world backend modules, extract shared server code into a dedicated `server/sdks/*` crate instead of copying source trees.
