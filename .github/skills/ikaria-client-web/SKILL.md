---
name: ikaria-client-web
description: Use for React/Phaser web client work in client/, including view routing, SpacetimeDB hooks, Phaser integration, and design system usage.
---

# Ikaria Client Web Workflow

Use this skill when implementing or refactoring the Ikaria web client in `client/`.

## Architecture

- **Entry**: `main.tsx` → imports Bootstrap CSS, then `index.css`, then renders `App`.
- **Routing**: `App.tsx` is a state-based view router (world-selection → character-list → character-creation → game). No UI logic in App.
- **Views**: Dedicated components in `client/src/views/` — one per screen.
- **Game engine**: Phaser 3 in `client/src/game/`. GameScene handles tiles, camera, mask. PlayerMovement handles player sprite and labels.
- **Hooks**: Custom hooks in `client/src/hooks/` for SpacetimeDB integration.

## SpacetimeDB integration

- `SubscriptionProvider` wraps views inside `SpacetimeDBProvider` and calls `subscribeToAllTables()` once.
- `useSubscriptionReady()` returns true once the subscription's `onApplied` fires — use for loading states.
- `useLocalTable(tables.someTable)` reads from local cache reactively — replaces `useTable` from the SDK.
- Never use `useTable` from `spacetimedb/react` — it creates per-table subscriptions that conflict with `subscribeToAllTables()`.
- Reducers are called via `useReducer(reducers.someReducer)` from the SDK.

## Design system

- All colors come from CSS custom properties in `client/src/index.css` — see `ikaria-design-system` skill for details.
- Never hardcode colors or use Bootstrap `--bs-*` variables.

## Phaser conventions

- Not pixel art — no `pixelArt: true` in config.
- Use `resolution: 4` on small text labels to prevent bilinear blur.
- `Scale.NONE` mode with manual `game.scale.resize()` via ResizeObserver.
- Camera zoom is dynamic: `canvasSize / VISIBLE_AREA` (VISIBLE_AREA = 21 tiles × 16px = 336).
- Font: Roboto Black (900) loaded via Google Fonts in `index.html`.

## Implementation checklist

1. Keep views as separate components — don't merge UI into App.tsx.
2. Use `useLocalTable` + `useSubscriptionReady`, never `useTable`.
3. Reference design system variables for all colors — never hardcode hex values in CSS or components.
4. Test Phaser changes with the panel layout (side panels 200px, top 40px, bottom resizable 100-500px).
5. Validate with `cd client && npm run lint`.
