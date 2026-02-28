---
name: ikaria-design-system
description: Enforces the Ikaria dark design system — single-source CSS variables, no hardcoded colors, no Bootstrap variable indirection.
target: github-copilot
tools: [read, edit, search, execute]
infer: true
---

# Ikaria Design System Agent

Use this agent when adding or modifying any visual styling across the project.

## Operating rules

1. All colors are defined as CSS custom properties in `client/src/index.css` under `:root`. This is the single source of truth.
2. The palette variables are: `--bg-base`, `--bg-panel`, `--bg-surface`, `--border-subtle`, `--text-primary`, `--text-muted`, `--text-heading`, `--accent-primary`, `--accent-hover`.
3. Every CSS rule must reference these variables — never hardcode hex/rgb color values. The only exception is `#000` for the Phaser canvas wrapper background.
4. Never use Bootstrap `--bs-*` CSS variables. Bootstrap components are overridden in `index.css` to use our palette directly.
5. CSS load order is critical: `bootstrap.min.css` (in `main.tsx`) → `index.css` (in `main.tsx`) → component CSS (via imports). Our overrides must load after Bootstrap.
6. When adding new UI components, style them using the existing variables. If a new semantic color is needed, add it to `:root` in `index.css` and use it everywhere — do not create one-off values.
7. The game layout panels (`GameLayout.css`) and Bootstrap component overrides (`index.css`) must use the same variables for the same purposes (e.g., panel backgrounds and card backgrounds both use `--bg-panel`).
8. After any styling change, visually verify that all views (WorldSelection, CharacterList, CharacterCreation, Game) share the same consistent dark tones.
