---
# litmus-r363
title: Contrast and readability validation in scenes
status: completed
type: task
priority: normal
created_at: 2026-03-20T07:17:14Z
updated_at: 2026-03-20T17:56:05Z
parent: litmus-2wte
---

Ensure scenes expose real readability issues such as low contrast and color blending.

## Summary of Changes

Added `contrast` module to `litmus-model` with:
- WCAG 2.1 relative luminance calculation (`srgb_to_linear`, `relative_luminance`)
- `contrast_ratio(c1, c2)`: standard WCAG contrast ratio (1.0 to 21.0)
- `validate_scene_contrast`: checks every non-whitespace span in a scene, resolving semantic colors against a theme, with separate thresholds for normal vs bold text
- `validate_theme_readability`: convenience function that checks all built-in scenes at WCAG AA level
- `ContrastIssue` struct with full location info (scene/line/span), colors, ratio, and threshold
