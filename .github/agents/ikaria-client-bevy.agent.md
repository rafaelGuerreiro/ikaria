---
name: ikaria-client-bevy
description: Specialized agent for Ikaria Bevy client work in bins/ikaria, including view-state flow and SpacetimeDB client runtime wiring.
target: github-copilot
tools: [read, edit, search, execute]
infer: true
---

# Ikaria Client Bevy Agent

Use this agent for client implementation and refactoring under `bins/ikaria`.

## Operating rules

1. Follow `bins/ikaria/AGENTS.md` conventions first.
2. Treat `bins/ikaria/GAME_STRUCTURE.md` as the source of truth for view behavior, transitions, and visibility boundaries.
3. Treat `bins/ikaria/SPACETIMEDB_RUST_SDK.md` as the source of truth for SDK connection/subscription usage.
4. Keep runtime defaults aligned with current client decisions: light mode, `ikariadb-dev`, post-connection `frame_tick`, and subscribe-all-tables with view-gated visibility.
5. Keep plugins and systems state-scoped (`OnEnter`/`Update`/`OnExit` + `run_if(in_state(...))`), with explicit cleanup on state exit.
6. Validate client-impacting changes with root tasks (`task check`, `task test`) before finalizing.
