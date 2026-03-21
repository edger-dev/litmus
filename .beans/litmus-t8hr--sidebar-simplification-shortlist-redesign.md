---
# litmus-t8hr
title: Sidebar simplification & shortlist redesign
status: completed
type: feature
priority: normal
created_at: 2026-03-21T06:17:32Z
updated_at: 2026-03-21T13:10:26Z
order: zz
---

Simplify sidebar to shortlist/favorites, move filters to browse page, fix compare workflow, add UseAsAppTheme button

## Summary of Changes

### State refactoring (`state.rs`, `main.rs`)
- Renamed `CompareSelection` to `Shortlist` with `MAX_SHORTLIST = 8`
- Extracted CVD from `FilterState` into new `CvdSimulation` global signal
- Made `FilterState` page-local (removed from global context providers)
- Updated all context providers in `App`

### Components (`components.rs`)
- Renamed `CompareToggle` to `ShortlistToggle` with +Shortlist/Shortlisted text
- Added `UseAsAppThemeButton` component (toggles app theme, shows checkmark when active)

### Browse page (`theme_list.rs`)
- Added inline filter bar (search, variant buttons, readability dropdown)
- Flat alphabetical grid — removed family group headings
- ThemeCard now shows ShortlistToggle + UseAsAppThemeButton
- FilterState is local `use_signal`

### Sidebar (`sidebar.rs`)
- Stripped search, variant filters, readability dropdown, theme list, app theme dropdown
- Added nav links (Browse Themes, Compare)
- Added shortlist panel with checkboxes for compare selection
- Compare button builds URL from checked themes (max 4)
- CVD stays in sidebar as global accessibility tool

### Detail page (`theme_detail.rs`)
- Replaced CompareToggle with ShortlistToggle + UseAsAppThemeButton
- Reads CVD from `CvdSimulation` signal
- Updated `c` keyboard shortcut to toggle shortlist

### Compare page (`compare.rs`)
- Removed `CompareSelector` dropdowns — URL is source of truth
- Reads CVD from `CvdSimulation` signal

### Scene across page (`scene_across.rs`)
- Updated to use ShortlistToggle and CvdSimulation

### CSS (`style.css`)
- Added: .filter-bar, .filter-bar-search, .filter-bar-readability, .shortlist-toggle, .use-as-app-theme-btn, .sidebar-shortlist*, .sidebar-nav*
- Removed: .sidebar-search, .sidebar-filters, .sidebar-readability*, .sidebar-theme-list, .sidebar-family*, .sidebar-theme-item, .sidebar-app-theme*, .family-group, .family-name, .compare-toggle, .compare-selector, .compare-select, .compare-vs, .compare-chip*

### Cleanup (`family.rs`)
- Removed unused `ThemeFamily` struct and `group_by_family` function
- Kept `theme_family()` (still used for search matching)
