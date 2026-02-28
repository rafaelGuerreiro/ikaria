---
name: ikaria-design-system
description: Use for any styling or theming changes to enforce single-source CSS variables and consistent dark palette across all views.
---

# Ikaria Design System Workflow

Use this skill when adding or modifying any visual styling in the Ikaria web client.

## Single source of truth

All colors live in `client/src/index.css` under `:root`:

```css
:root {
  --bg-base: #111118;       /* Darkest — body, page background */
  --bg-panel: #1a1a28;      /* Panels, cards, list items, form inputs */
  --bg-surface: #22223a;    /* Hover states, focus states, alerts */
  --border-subtle: #2a2a3a; /* All borders */
  --text-primary: #dddde8;  /* Main text */
  --text-muted: #8888aa;    /* Secondary text */
  --text-heading: #aaaacc;  /* Headings, emphasis */
  --accent-primary: #6366f1; /* Buttons, links, focus rings */
  --accent-hover: #818cf8;   /* Hover accent */
}
```

## Rules

1. Never hardcode hex/rgb color values in CSS or inline styles. Always use `var(--variable-name)`.
2. Never reference Bootstrap `--bs-*` variables. Override Bootstrap components directly with our variables.
3. CSS load order: `bootstrap.min.css` → `index.css` → component CSS. Never import Bootstrap after `index.css`.
4. When a new semantic color is needed, add it to `:root` in `index.css` — do not create per-component one-offs.
5. Game layout panels and Bootstrap component overrides must use the exact same variable for the same purpose.

## Where styles live

| File | Purpose |
|------|---------|
| `client/src/index.css` | Variables, global resets, Bootstrap overrides |
| `client/src/App.css` | App container layout, world card hover |
| `client/src/views/GameLayout.css` | Game view panel layout (left/right/top/bottom/center) |
| `client/index.html` | Font imports (Google Fonts Roboto:900), `data-bs-theme="dark"` |

## Checklist

1. New colors? Add to `:root` in `index.css`, reference everywhere.
2. New component? Style with existing variables — `--bg-panel` for backgrounds, `--border-subtle` for borders.
3. Verify all views look consistent — WorldSelection, CharacterList, CharacterCreation, and Game should share the same dark tones.
