---
# litmus-3fax
title: Compact expandable palette display
status: completed
type: task
priority: normal
created_at: 2026-03-20T18:29:16Z
updated_at: 2026-03-20T18:37:17Z
parent: litmus-74j8
---

Collapse ANSI palette into horizontal strip that expands on click. Gets palette out of the way so scene is front and center.

## Summary of Changes

Replaced the full color palette on the detail page with a compact expandable palette. Compact mode shows a single row of swatches (bg, fg, cursor + ANSI strip). Clicking expands to show full detail with selection colors, labeled ANSI grid (8x2), and hex values. Driven by use_signal toggle.
