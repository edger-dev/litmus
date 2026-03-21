---
# litmus-84j7
title: Shortlist UX improvements
status: completed
type: feature
priority: normal
created_at: 2026-03-21T08:02:40Z
updated_at: 2026-03-21T13:10:26Z
order: zy
---

1. Limit shortlist to 5 themes (keep most recent)
2. Apply pushes current theme to top of shortlist
3. Gray out Shortlist checkbox for current theme
4. Show 'Feel Lucky' when shortlist empty, pick random theme

## Summary of Changes

- Reduced MAX_SHORTLIST from 8 to 5
- Apply button now pushes previous app theme to top of shortlist (with dedup and truncate)
- ShortlistCheckbox and ShortlistToggle show 'Current' and are grayed out/disabled for the active app theme
- Detail page 'c' keyboard shortcut also respects current theme
- Sidebar shows 'Feel Lucky' button when shortlist is empty (no app theme set), which picks two random themes and navigates to compare
- Added js-sys dependency for wasm random
- Added CSS for disabled shortlist states and Feel Lucky button styling
