---
# litmus-oonb
title: TUI navigation
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:02Z
updated_at: 2026-03-20T17:01:37Z
order: "y"
parent: litmus-f1b3
---

Switch between themes, toggle between swatches/mock-ups/live views. Use ratatui or similar TUI framework.

## Summary of Changes

- Added `catppuccin_mocha()` and `solarized_dark()` theme constructors to `theme_data.rs`
- Added `all_themes()` returning all three themes
- Introduced `App` struct with themes, theme_index, view, git_diff, ls_output fields
- Added `View::name()` returning display name for status bar
- Refactored `run()` to use `App::new()` and split layout into content + 1-line status bar
- Status bar shows: theme name [N/M] | view name | key hints
- Key bindings: Tab (next view), BackTab/Shift+Tab (prev view), Left (prev theme), Right (next theme)
