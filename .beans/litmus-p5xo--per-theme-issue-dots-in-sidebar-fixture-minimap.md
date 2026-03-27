---
# litmus-p5xo
title: Per-theme issue dots in sidebar fixture minimap
status: completed
type: task
priority: normal
created_at: 2026-03-27T04:53:15Z
updated_at: 2026-03-27T05:39:20Z
parent: litmus-ysy5
blocked_by:
    - litmus-962t
---

Show per-theme colored dots on the sidebar fixture minimap during compare.

- [x] Added CompareIssueDots context signal with per-theme data
- [x] Uses each theme's foreground hex color for the dot
- [x] Render colored dots per fixture in the minimap
- [x] Each dot represents one theme's issue count (tooltip shows name + count)
- [x] Detail page clears compare dots, falls back to single badge


## Summary of Changes

Added CompareIssueDots context signal. Compare page publishes per-theme issue data. Minimap renders colored dots (one per theme with issues) when on compare page, falls back to single badge on detail page. Detail page clears compare dots on mount.
