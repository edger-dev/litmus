---
# litmus-h4yq
title: Define scene format
status: completed
type: task
priority: normal
created_at: 2026-03-20T07:17:14Z
updated_at: 2026-03-20T17:50:33Z
parent: litmus-2wte
---

Define the scene format: annotated sample content with semantic color references.

## Summary of Changes

Added `scene` module to `litmus-model` with:
- `ThemeColor` enum: semantic color references (Foreground, Background, Cursor, Selection*, Ansi(0-15)) with `resolve()` method
- `TextStyle`: bold/italic/underline/dim modifiers
- `StyledSpan`: text + optional fg/bg ThemeColor + TextStyle, with builder methods
- `SceneLine`: sequence of StyledSpans
- `Scene`: id, name, description, and lines — a complete terminal scene definition
- All types are serde-serializable for data-driven scene definitions
