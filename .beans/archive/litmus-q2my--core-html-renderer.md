---
# litmus-q2my
title: Core HTML renderer
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:14Z
updated_at: 2026-03-20T17:54:46Z
parent: litmus-2wte
---

Core renderer: theme data + annotated content → colored HTML spans (monospace).

## Summary of Changes

Added web rendering engine:
- `scene_renderer.rs`: Dioxus components that resolve ThemeColor references to inline CSS styles on HTML spans inside monospace pre blocks
- `themes.rs`: compile-time embedding of all 19 themes for WASM builds
- `main.rs`: full app with theme selector dropdown and all scenes rendered
- Added PartialEq derives to Theme/AnsiColors/Scene for Dioxus component props
