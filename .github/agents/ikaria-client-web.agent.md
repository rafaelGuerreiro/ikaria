---
name: ikaria-client-web
description: Specialized agent for Ikaria React/Phaser web client work in client/, including view routing, SpacetimeDB hooks, and Phaser game integration.
target: github-copilot
tools: [read, edit, search, execute]
infer: true
---

# Ikaria Client Web Agent

Use this agent for client implementation and refactoring under `client/`.

## Operating rules

1. The web client is React + Phaser + SpacetimeDB (TypeScript). Entry point is `client/src/main.tsx`.
2. `App.tsx` is a view router only — it orchestrates views (WorldSelection → CharacterList → CharacterCreation → Game) and manages connection state. Do not add UI logic to App.tsx.
3. Each view is a dedicated component in `client/src/views/` (e.g., `CharacterListView.tsx`, `GameView.tsx`). Keep views separated.
4. SpacetimeDB data access uses `useLocalTable` (from `client/src/hooks/useLocalTable.ts`), NOT `useTable` from the SDK. The app calls `subscribeToAllTables()` once via `SubscriptionProvider`, and views read from local cache.
5. Use `useSubscriptionReady()` (from `client/src/hooks/useSubscriptionReady.ts`) for loading states, not custom timers or `isActive` checks.
6. Phaser game logic lives in `client/src/game/` (GameScene.ts, PlayerMovement.ts). Phaser is initialized in `GameView.tsx` with `Scale.NONE` and manual resize via ResizeObserver.
7. The game is NOT pixel art — do not set `pixelArt: true` in Phaser config. Use `resolution: 4` on small text labels to prevent blur under bilinear filtering.
8. All styling must use the CSS custom properties defined in `client/src/index.css` (e.g., `--bg-base`, `--bg-panel`, `--bg-surface`, `--border-subtle`, `--text-primary`, `--text-muted`, `--accent-primary`). Never hardcode colors or use Bootstrap `--bs-*` variables.
9. CSS load order matters: Bootstrap is imported first in `main.tsx`, then `index.css` overrides it. Component-specific CSS (e.g., `GameLayout.css`, `App.css`) loads last via their component imports.
10. Validate with `cd client && npm run lint` before finalizing.
