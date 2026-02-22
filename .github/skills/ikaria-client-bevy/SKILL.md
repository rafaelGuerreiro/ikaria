---
name: ikaria-client-bevy
description: Use for Bevy client implementation in bins/ikaria, including view-state organization and SpacetimeDB frame_tick/subscription wiring.
---

# Ikaria Client Bevy Workflow

Use this skill when implementing or refactoring the Ikaria client in `bins/ikaria`.

## Reference-first policy

- Client coding rules: `bins/ikaria/AGENTS.md`
- View/state behavior: `bins/ikaria/GAME_STRUCTURE.md`
- SpacetimeDB client integration: `bins/ikaria/SPACETIMEDB_RUST_SDK.md`

## Current implementation defaults

- UI mode: light mode.
- Development module: `ikariadb-dev`.
- Connection advancement: `frame_tick()`.
- Tick lifecycle: start ticking only after initial pre-connection screen.
- Subscription strategy: `subscribe_to_all_tables()`, with view/state systems controlling visibility and interaction.

## Implementation checklist

1. Map requested behavior to `AppState`/view plugin boundaries.
2. Keep systems state-gated (`OnEnter`, `Update` + `run_if(in_state(...))`, `OnExit`).
3. Keep connection/session data in resources and view-local entities state-scoped.
4. Wire SpacetimeDB callbacks into Bevy events/resources, not cross-feature direct coupling.
5. Ensure post-connection views own tick/subscription setup and teardown.
6. Validate from repo root (`task check`, `task test`) before finalizing.
