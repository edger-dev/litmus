---
# litmus-5tj3
title: Theme quality checks
status: completed
type: task
priority: normal
created_at: 2026-03-20T07:17:11Z
updated_at: 2026-03-20T17:39:02Z
parent: litmus-irro
---

Ensure all required colors are present and no missing values for each curated theme.

## Summary of Changes

All 19 theme files validated:
- All parse successfully through the TOML parser
- All required colors present (background, foreground, 16 ANSI colors)
- Optional fields (cursor, selection_background, selection_foreground) explicitly provided for all themes
- All existing litmus-model tests pass (11/11)
