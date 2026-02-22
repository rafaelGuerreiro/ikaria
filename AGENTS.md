# Ikaria Monorepo Agent Guide

## Scope
This file is for repository-wide guidance only.  
Backend/database-specific rules live in `bins/ikariadb/AGENTS.md`.

## Workspace layout
- `bins/ikariadb`: SpacetimeDB backend module.
- `bins/ikaria`: Bevy client.
- `sdks/types`: generated Rust bindings consumed by the client.
- `sdks/shared`: hand-written shared constants.

## Canonical workflow (root)
Use `Taskfile.yml` from repository root:

- `task sdk-rust` - regenerate Rust bindings into `sdks/types/src/autogen`.
- `task fmt` - format workspace.
- `task check` - lint + type-check workspace.
- `task test` - run workspace tests.
- `task build` - build backend wasm and host workspace targets.
- `task login`, `task publish`, `task publish-prod`, `task unsafe-overwrite` - Spacetime environment operations.

## Generated SDK expectations
- `sdks/types/src/autogen` is generated output; do not hand-edit it.
- Client-facing backend types come from `ikaria-types`.
- If backend schema/reducers change, regenerate bindings and re-check workspace compatibility.

## Cross-cutting conventions
- Keep dependency versions in root `[workspace.dependencies]`.
- Child crates should reference shared dependencies with `workspace = true`.
- Keep edits scoped: backend rules in backend AGENTS, client rules in client code, shared constants in `sdks/shared`.
- Do not add `#[allow(dead_code)]` to silence unused code.
- For unimplemented/placeholder work, keep `dead_code` warnings visible instead of suppressing them.

## Agent/skill catalog
- Custom agents: `.github/agents/*.agent.md`
- Skills: `.github/skills/**/SKILL.md`

Use requirement-auditing assets for constrained requests before finalizing:
- agent: `ikaria-requirement-auditor`
- skill: `ikaria-requirement-lockstep`
