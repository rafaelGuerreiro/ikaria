# Ikaria Agent Guide

## Workspace layout
- `bins/server`: SpacetimeDB-backed server executable.
- `bins/client`: Bevy-based game client executable.
- `sdks/shared`: Shared protocol/domain types consumed by both binaries.

## Build, test, lint
- Full workspace check: `cargo check --workspace`
- Check one crate: `cargo check -p ikaria-server` or `cargo check -p ikaria-client`
- Full test suite: `cargo test --workspace`
- Run a single test by name: `cargo test -p <crate> <test_name>`
- Run one integration test target: `cargo test -p <crate> --test <target_name>`
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`
- Format: `cargo fmt --all`
- Format check: `cargo fmt --all --check`

## Architecture (big picture)
- This repository is a Rust workspace monorepo for Ikaria (Tibia-inspired).
- Dependency versions are centralized in the root `Cargo.toml` under `[workspace.dependencies]`.
- `ikaria-shared` (`sdks/shared`) is the contract layer for types exchanged between server and client.
- The server side is intended to hold authoritative game state and game rules on top of SpacetimeDB patterns (tables + reducers).
- The client side is intended to use Bevy ECS composition (plugins, systems, resources) to render and simulate client behavior.

## Repository conventions
- Add new executable crates under `bins/<name>` and new reusable crates under `sdks/<name>`.
- When adding any dependency, define it in root `[workspace.dependencies]` and reference it in child crates with `workspace = true`.
- Keep cross-boundary protocol types in `sdks/shared` to avoid duplicate definitions in `bins/server` and `bins/client`.
- Avoid direct dependencies between binaries; share code via `sdks/*` crates.
- For Bevy code, organize features as plugins and schedule systems explicitly (`Startup`, `Update`, etc.).
- For SpacetimeDB code, keep reducer logic deterministic and centered around state transitions.
