---
# litmus-72xs
title: Add compact issue chips per theme on compare page
status: completed
type: task
priority: normal
created_at: 2026-03-27T04:53:11Z
updated_at: 2026-03-27T15:45:47Z
order: zzzzzy
parent: litmus-ysy5
blocked_by:
    - litmus-962t
---

Interactive issue chips below each column header for navigation.

- [x] Render compact chip strip per theme (reuse detail-issue-chip CSS)
- [x] Adapt chip layout for narrower columns (scrollable or wrapping)
- [x] Click chip → scroll to first affected fixture in that column
- [x] Click again → cycle to next affected fixture
- [x] Escape to deactivate

## Summary of Changes

Added interactive contrast issue chips to the compare page's column headers. Each theme column shows its deduplicated contrast rules as clickable chips (reusing the existing detail-issue-chip CSS with compact sizing). Clicking a chip scrolls to the first affected fixture and highlights the contrast issue spans in that column's terminal output. Clicking again cycles to the next affected fixture. Escape clears the selection. Single-fixture chips toggle off on re-click.

Files changed:
- `crates/litmus-web/src/pages/compare.rs` — extended ThemeContrastData with rules + fixtures_per_rule, added active_issue signal, rendered chip strips, wired focused_rule to TermOutputView
- `crates/litmus-web/assets/style.css` — added .compare-chips container and compact chip overrides
