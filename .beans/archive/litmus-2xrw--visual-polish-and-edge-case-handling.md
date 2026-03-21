---
# litmus-2xrw
title: Visual polish and edge case handling
status: completed
type: task
priority: normal
created_at: 2026-03-20T07:17:17Z
updated_at: 2026-03-20T18:07:01Z
parent: litmus-iiek
---

Visual polish, edge case handling for very bright/dark themes and low contrast scenarios.

## Summary of Changes

Visual polish for edge cases:
- Theme cards show light/dark variant label and fg/bg contrast ratio
- Neutral gray borders (rgba) visible on both light and dark theme cards/scene blocks
- Box shadow for card depth regardless of background
- Relative luminance detection for light vs dark theme classification
