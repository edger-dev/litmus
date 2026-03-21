---
# litmus-g6c0
title: ANSI color swatches display
status: completed
type: feature
priority: normal
created_at: 2026-03-20T07:17:02Z
updated_at: 2026-03-20T11:26:38Z
order: g
parent: litmus-f1b3
---

Display the 16 ANSI colors + fg/bg/cursor/selection as colored blocks in the terminal.

## Summary of Changes

Implemented ANSI color swatches TUI display using ratatui and crossterm. Added ThemeWithExtras wrapper with hardcoded Tokyo Night theme. SwatchesWidget renders 2x8 grid with labels plus fg/bg/cursor/sel row. main.rs bootstraps TUI with clean terminal restore on quit.
