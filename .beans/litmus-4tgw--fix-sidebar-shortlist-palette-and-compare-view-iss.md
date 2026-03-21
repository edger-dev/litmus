---
# litmus-4tgw
title: Fix sidebar, shortlist, palette, and compare view issues
status: completed
type: task
priority: normal
created_at: 2026-03-21T12:23:13Z
updated_at: 2026-03-21T12:26:24Z
---

Five fixes:
- [ ] Browse Themes sidebar link active on detail pages (should not be)
- [ ] Shortlist at max 5: should evict oldest instead of refusing
- [ ] Compare page: removing from shortlist should refresh; clearing should go to themes
- [ ] Detail page color palette always expanded
- [ ] Compare page: replace ColorDiffTable with per-theme color palettes at end

## Summary of Changes

1. **Browse Themes active state**: Changed to only highlight on ThemeList, not ThemeDetail
2. **Shortlist evicts oldest at max**: Both ShortlistCheckbox and keyboard handler now remove the oldest entry when adding a 6th theme
3. **Compare page reacts to shortlist changes**: Remove button rebuilds compare URL and navigates; clear button navigates to themes list
4. **Detail page palette always expanded**: Changed initial state from false to true
5. **Compare page palette**: Removed ColorDiffTable component and CSS, added per-theme color palettes at the end of the compare view
6. **Cleanup**: Removed dead ColorDiffTable component, ANSI_NAMES constant in components.rs, and all color-diff CSS
