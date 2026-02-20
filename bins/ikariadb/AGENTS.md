# Ikaria Server Agent Guide

## Scope
This file is for `bins/server` only (SpacetimeDB backend work).

## Build, test, lint (server-focused)
Preferred from repository root (`Taskfile.yml`):
- `task fmt`
- `task clippy`
- `task check`
- `task test`
- `task build`
- `task test-server`

Dependency rule: all non-`fmt` tasks depend on `clippy`, and `clippy` depends on `fmt`.

Optional from `bins/server` (`bins/server/Taskfile.yml`):
- `task --taskfile bins/server/Taskfile.yml check`
- `task --taskfile bins/server/Taskfile.yml test`
- `task --taskfile bins/server/Taskfile.yml build`

Direct cargo fallback:
- Check server crate: `cargo check -p ikariadb`
- Run server tests: `cargo test -p ikariadb`
- Run one test by name: `cargo test -p ikariadb <test_name>`
- Lint server crate: `cargo clippy -p ikariadb --all-targets -- -D warnings`
- Run scaffold binary: `cargo run -p ikariadb`
- If server changes touch shared types, also run: `cargo check --workspace`

## Server architecture direction
- `ikariadb` is the authoritative backend crate and consumes shared contracts from `ikaria-shared` (`sdks/shared`).
- Dependency versions live in root `[workspace.dependencies]`; `bins/server/Cargo.toml` must reference them via `workspace = true`.
- Shared server utilities live in `src/error.rs` and `src/extend/validate.rs` and should be reused for new domains/reducers.
- As the module grows, follow a split similar to:
  - reducer entrypoints + lifecycle hooks (`init`, `client_connected`, `client_disconnected`)
  - domain services for business logic
  - domain tables/types/views grouped by domain

Suggested layout once reducers are introduced:
```
src/
├── lib.rs
├── error.rs
├── extend/
└── repository/
    ├── mod.rs
    ├── {domain}.rs
    └── {domain}/
        ├── reducers.rs
        ├── services.rs
        ├── types.rs
        └── views.rs
```

## Conventions to carry from blok/server
- Keep reducers thin: validate input + access control + delegate to services.
- Services own logic; pass `Identity` explicitly instead of reading sender implicitly deep in service code.
- Enforce domain boundaries: each domain mutates its own tables; cross-domain writes go through the owning domain service API.
- Prefer private tables and expose read models through explicit views.
- For scheduled reducers, require internal access before processing.
- Keep state transitions deterministic; use reducer context time/randomness (`ctx.timestamp`, context RNG) rather than system time/random.
- Use typed domain errors mapped into a shared `ServiceResult<T>`/`ServiceError` model.
- Keep shared protocol/data contracts in `sdks/shared` instead of duplicating server/client types.
