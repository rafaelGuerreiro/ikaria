# Ikaria – Web Client

React + TypeScript + Vite web client for Ikaria. Uses [Phaser](https://phaser.io/) for the game canvas and [SpacetimeDB](https://spacetimedb.com/) for real-time backend connectivity.

## Prerequisites

- Node.js ≥ 20
- [Task](https://taskfile.dev/) (repo-wide task runner)

## Getting Started

```bash
# from the repo root
task client:dev
```

This starts the Vite dev server with HMR.

Other tasks (run from the repo root):

| Command              | Description                     |
| -------------------- | ------------------------------- |
| `task client:dev`    | Start dev server                |
| `task client:build`  | Type-check and build for production |
| `task client:lint`   | Type-check and lint             |

## Project Structure

```
src/
├── views/           # UI views (CharacterList, CharacterCreation, WorldSelection, Game)
├── hooks/           # React hooks (SpacetimeDB subscriptions, local tables)
├── game/            # Phaser game scene and player movement
├── module_bindings/ # Auto-generated SpacetimeDB client bindings
├── App.tsx          # Top-level router / view orchestrator
├── index.css        # Design-system CSS variables
└── worlds.ts        # World definitions
```

## SpacetimeDB

Server bindings live in `src/module_bindings/` and are auto-generated from the Rust server schema. The client connects via the `spacetimedb` SDK and uses a `SpacetimeDBProvider` in `App.tsx`.

## Design System

All colours and theming are defined as CSS custom properties in `src/index.css` (e.g. `--bg-base`, `--accent-primary`). Components must use these variables instead of hard-coded colour values.
