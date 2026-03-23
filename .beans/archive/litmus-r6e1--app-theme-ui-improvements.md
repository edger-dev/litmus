---
# litmus-r6e1
title: App Theme UI Improvements
status: completed
type: feature
priority: normal
created_at: 2026-03-21T07:19:13Z
updated_at: 2026-03-21T13:10:26Z
order: zk
---

Restructure ThemeCard, add ShortlistCheckbox, rework sidebar, rename button text, fix tooltip clipping

## Summary of Changes

### 1. ThemeCard restructured (theme_list.rs)
- Link now wraps only the card body (header + preview + swatches), not the footer actions
- Action buttons sit outside the Link in a `.theme-card-actions` div with app theme colors

### 2. ShortlistCheckbox added (components.rs)
- New `ShortlistCheckbox` component with `<label><input type=checkbox> Shortlist</label>`
- Used on browse page cards; `ShortlistToggle` (button) kept for detail page
- Renamed "Use as App Theme" → "Apply", "✓ App Theme" → "✓ Applied"

### 3. Sidebar reworked (sidebar.rs)
- Removed compare checkboxes (`checked: HashSet`) and all checkbox-based gating
- App theme pinned at top of shortlist with "current" badge
- Compare URL = app theme + shortlist slugs (deduped); shown when 2+ items

### 4. Tooltip clipping fixed (scene_renderer.rs, style.css)
- `line_idx` forwarded to `SpanView`; lines 0-1 get `contrast-tooltip-below` class
- `.scene-block pre`: `overflow-x: clip; overflow-y: visible` instead of `overflow-x: auto`
- `.contrast-issues-list`: increased max-height, `overflow-y: visible`

### 5. CSS updates (style.css)
- `.theme-card-actions` gets explicit app theme bg/fg/border
- `.shortlist-checkbox` styles
- `.sidebar-current-badge` and `.sidebar-shortlist-name-link` styles
- Removed old `.sidebar-shortlist-check` styles
