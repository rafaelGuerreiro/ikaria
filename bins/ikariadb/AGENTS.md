# Ikariadb Agent Guide

## Scope
This file covers backend/database work in `bins/ikariadb` only.

## Build, test, validate
Use root tasks (preferred):
- `task check`
- `task test`
- `task build`

## Backend architecture
- Entry points are in `src/lib.rs`:
  - `init`
  - `identity_connected`
  - `identity_disconnected`
- Domain modules are under `src/repository` (currently: `account`, `world`, `item`, `progression`, `event`).
- Shared backend utility modules are in:
  - `src/error.rs`
  - `src/extend/*`
- Event flow uses `repository/event*` with publisher/service + deferred scheduling pattern.

## Schema and type rules
- Table structs (`#[table(...)]`) must not derive traits.
- IDs:
  - users: `Identity`
  - everything else: `u64`
  - owning tables use `#[auto_inc]`.
- Coordinates (`x`, `y`, `z`) use `u16`.
- Ground baseline must come from `ikaria_shared::GROUND_LEVEL`.
- Gameplay enums are backend-only and live under repository domain `types.rs` files.
- Spacetime enums should use `V1` suffix (`DirectionV1`, `SkillV1`, etc.) and be annotated with `SpacetimeType`.
- Avoid ID type aliases; use direct `Identity` / `u64` in table fields.

## Generated client contract impact
- Backend schema/reducer changes affect generated SDK in `sdks/types/src/autogen`.
- Do not edit generated files manually.
- After backend contract changes, regenerate bindings (`task sdk-rust`) and validate workspace builds/tests.

## Implementation style
- Keep reducers thin: validate input + access control + delegate to services.
- Services own logic; pass `Identity` explicitly instead of reading sender implicitly deep in service code.
- Enforce domain boundaries: each domain mutates its own tables; cross-domain writes go through the owning domain service API.
- Prefer private tables and expose read models through explicit views.
- For scheduled reducers, require internal access before processing.
- Keep state transitions deterministic; use reducer context time/randomness (`ctx.timestamp`, context RNG) rather than system time/random.
- Use typed domain errors mapped into a shared `ServiceResult<T>`/`ServiceError` model.
- Keep shared protocol/data contracts in `sdks/shared` instead of duplicating server/client types.
- Keep business logic in services/helpers, not reducer entrypoints.
- Keep table changes compatible with event payloads and downstream generated types.
