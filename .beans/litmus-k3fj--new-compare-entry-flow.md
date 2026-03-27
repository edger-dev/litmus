---
# litmus-k3fj
title: New compare entry flow
status: completed
type: task
priority: normal
created_at: 2026-03-27T04:53:23Z
updated_at: 2026-03-27T06:34:07Z
parent: litmus-ysy5
blocked_by:
    - litmus-nqee
---

Strict 2-theme side-by-side compare: left = app theme, right = compared theme.

- [x] Set MAX_COMPARE to 2, removed compact from compare page
- [x] Added CompareButton component on theme cards
- [x] Added CompareButton on detail page
- [x] Added theme picker dropdowns in compare column headers
- [x] Feel Lucky already works correctly with MAX_COMPARE=2
- [x] Hardcoded 2-column grid, removed --compare-cols CSS var


## Summary of Changes

Simplified compare to strict 2-theme side-by-side. Added CompareButton component (compare against app theme) on cards and detail page. Added theme picker dropdowns in compare column headers to swap themes without leaving the page. Hardcoded 2-column grid layout. MAX_COMPARE reduced to 2.
