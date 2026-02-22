# Ikaria Client Agent Guide

## Scope
This file covers client/gameplay work in `bins/ikaria` only.

## Build, test, validate
Use root tasks (preferred):
- `task check`
- `task test`
- `task build`

## Client baseline
- Client runtime uses Bevy `0.18.0` from workspace dependencies.
- Backend contract types consumed by the client come from `ikaria-types`.
- Shared gameplay constants should live in `ikaria-shared` when reused across crates.
- Before silencing any Rust/Clippy error or warning (including via `#[allow(...)]`), ask the user first and wait for explicit approval.

## Bevy 0.18 implementation rules
- Organize features into plugins (auth, character selection, game world, movement, camera), ideally one plugin per screen/feature.
- Use typed app states for flow control (`SignIn`, `CharacterSelect`, `InGame`); add `SubStates` only when a state needs nested phases.
- Register lifecycle systems with `OnEnter`, `Update`, and `OnExit`, gated by `run_if(in_state(...))`.
- Trigger screen changes through `NextState<AppState>`, not ad-hoc state mutation.
- Co-locate each state's setup (`OnEnter`) + update (`run_if`) + cleanup (`OnExit`) registration together.
- Prefer state-scoped cleanup helpers (`DespawnOnExit`, `DespawnOnEnter`, state-scoped entity patterns); use marker-component cleanup when teardown is custom.
- Store cross-view data in resources (auth token/session, selected character, local player entity), and keep screen-local entities state-scoped.
- Prefer events for cross-feature communication (auth success, character selected, move target requests) to keep plugins decoupled.
- Keep update ordering explicit with sets/stages (input -> simulation -> presentation).

## Suggested client module layout
- `src/app_state.rs`: top-level states and optional substates.
- `src/features/sign_in`: sign-in plugin, UI, token bootstrap systems.
- `src/features/character_select`: character list/create plugin and systems.
- `src/features/game`: map, movement, camera, and runtime gameplay systems.
- `src/resources`: long-lived cross-view resources.
- `src/events`: events shared between features/plugins.

## Client reference docs
- `bins/ikaria/GAME_STRUCTURE.md`: authoritative view/state flow, visibility model, and runtime ordering expectations.
- `bins/ikaria/SPACETIMEDB_RUST_SDK.md`: authoritative SpacetimeDB Rust SDK usage, module target, ticking policy, and subscription strategy.

## View and movement expectations
- Sign-in supports loading a saved token from file before manual auth fallback.
- Character creation input is restricted to `name` and `gender`.
- In-game movement must support:
  - keyboard movement via `ButtonInput<KeyCode>` (`KeyW`, `KeyA`, `KeyS`, `KeyD`)
  - click-to-move by converting cursor position to world/grid position
- Keep the player centered on screen; render an area around the player of about 11 tiles per direction.

## Map and assets
- Start with a small 2D grassland map that can be walked freely.
- Use Kenney spritesheet assets and keep atlas/tile mapping setup in one place.
- Keep tile/grid units consistent with movement and camera framing logic.
