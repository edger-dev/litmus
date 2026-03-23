---
# litmus-jvkn
title: 'Iteration 2: Bug fixes + design feedback'
status: completed
type: epic
priority: normal
created_at: 2026-03-21T05:36:19Z
updated_at: 2026-03-21T13:10:26Z
order: z
---

8 items: fix app theme switcher, filter navigation, remove scene sidebar, separate filters, readability score, clickable contrast issues, move CVD, richer sidebar items

## Summary of Changes

All 8 items implemented:

1. **Fix app theme switcher** (`shell.rs`): Moved `app_theme.read()` inside `use_effect` so Dioxus tracks the dependency
2. **Filter navigation** (`sidebar.rs`): Added `use_navigator()` — search, variant, and readability filter changes push `Route::ThemeList`
3. **Remove scene tabs from sidebar** (`sidebar.rs`): Removed Scenes section entirely
4. **Separate variant/contrast filters** (`sidebar.rs`): All/Dark/Light now show count badges; readability is a separate dropdown below
5. **Readability score** (`contrast.rs`, `state.rs`, `sidebar.rs`, `theme_list.rs`, `theme_detail.rs`): Added `readability_score()`, replaced `good_contrast: bool` with `min_readability: Option<u8>`, shown on sidebar items, theme cards, and detail header
6. **Clickable contrast issues** (`theme_detail.rs`): Issues text is now a toggle button; expands to show per-scene issue list with fg/bg chips and ratios
7. **Move CVD** (`sidebar.rs`): CVD section moved below Compare, just above App Theme
8. **Richer sidebar items** (`sidebar.rs`, `style.css`): Items use theme bg/fg colors, show readability badge, subtle borders between items

Scene tabs moved to top of ThemeDetail (before palette, above scene content).
