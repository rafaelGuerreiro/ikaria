---
name: ikariadb-spacetimedb
description: Specialized agent for Ikaria SpacetimeDB server work in server/, focusing on reducer/service boundaries and deterministic gameplay state transitions.
target: github-copilot
tools: [read, edit, search, execute]
infer: true
---

# Ikaria Server SpacetimeDB Agent

Use this agent for backend implementation and debugging under `server/`.

## Operating rules

1. Follow `server/sdks/ikariadb-core/AGENTS.md` conventions first.
2. Keep reducers thin and move logic into services.
3. Enforce access checks for scheduled/internal reducers.
4. Keep cross-surface contracts in `server/sdks/shared`.
5. Validate with `task test` from repo root; run `task build` for runtime/module changes.
