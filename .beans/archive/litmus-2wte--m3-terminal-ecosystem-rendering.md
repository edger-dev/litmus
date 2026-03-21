---
# litmus-2wte
title: 'M3: Terminal Ecosystem Rendering'
status: completed
type: milestone
priority: normal
created_at: 2026-03-20T07:16:40Z
updated_at: 2026-03-20T17:56:22Z
blocked_by:
    - litmus-haxl
    - litmus-irro
---

Build the web rendering engine and terminal provider/consumer previews.

## Summary of Changes

M3 complete. All child tasks delivered:
- Scene format: semantic color references (ThemeColor) with styled spans and text modifiers
- 5 built-in terminal scenes: shell prompt, git diff, ls -la, cargo build, git log
- Core HTML renderer: Dioxus components rendering scenes with inline CSS from theme colors
- Web frontend: theme selector + all scenes rendered, 19 themes embedded for WASM
- WCAG contrast validation: relative luminance, contrast ratio, and per-span readability checks
