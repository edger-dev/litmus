---
# litmus-3px1
title: Improve compare view layout
status: completed
type: task
priority: normal
created_at: 2026-03-21T12:13:26Z
updated_at: 2026-03-21T13:10:26Z
order: zzy
---

Fix multiple issues with compare view:
- Remove multiple horizontal scroll bars, use single page-level scroll
- Show theme names as column headers at top only (vertical slices)
- Integrate color diff into the grid layout
- Remove repeated 'Choose' buttons
- Hide minimap issue badges on compare page (data is for single theme only)

## Summary of Changes

- **Single scroll bar**: Moved `overflow-x: auto` from individual `.compare-grid` and `.color-diff-body` to page-level `.page-compare`
- **Vertical column headers**: Added sticky theme name headers at top of page (with links to detail pages), removed per-scene theme name + Choose button repetition
- **Removed Choose buttons**: Theme names in sticky header are now clickable links instead
- **Hidden minimap badges on compare page**: Added `show_badges` prop to `SceneMinimap`, set to false on compare route (badges only make sense for single-theme detail view)
